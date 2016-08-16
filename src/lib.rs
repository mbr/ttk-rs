extern crate bresenham;
extern crate rustty;

use std::{convert, thread, time, sync};

pub mod view;

pub trait Application: Sized {
    type Action: convert::From<rustty::Event> + Send + 'static;
    type Task;

    fn handle_action(self, action: Self::Action) -> (Self, Option<Self::Task>);
    fn exec_task(&self, task: Self::Task) -> bool;
    fn view(&self, ctx: &mut view::DrawingContext);
}

fn draw_view<T: Application>(term: sync::Arc<sync::Mutex<rustty::Terminal>>, app: &T) {
    // unlock terminal
    let mut t = term.lock().unwrap();

    {
        let mut ctx = view::DrawingContext::new(&mut *t);
        app.view(&mut ctx);
    }

    // FIXME: remove all unwraps
    t.swap_buffers().unwrap();
}

pub struct MainLoop {
    min_delay: Option<time::Duration>,
    last_draw: time::Instant,
}

impl MainLoop {
    pub fn new() -> MainLoop {
        MainLoop {
            min_delay: None,
            last_draw: time::Instant::now(),
        }
    }

    pub fn min_delay(&mut self, delay: time::Duration) -> &mut MainLoop {
        self.min_delay = Some(delay);
        self
    }

    pub fn unlimited(&mut self) -> &mut MainLoop {
        self.min_delay = None;
        self
    }

    pub fn max_fps(&mut self, fps: u64) -> &mut MainLoop {
        self.min_delay(time::Duration::from_millis(1000 / fps));
        self
    }

    pub fn run<T: Application>(&mut self, app: T) {
        self.run_with_setup(app, |_| {});
    }

    pub fn run_with_setup<T: Application, F>(&mut self, mut app: T, setup: F)
        where F: FnOnce(sync::mpsc::Sender<<T as Application>::Action>) -> ()
    {
        // unfortunately, the rustty API is pretty bad here; we cannot draw on the
        // terminal while receiving events from it. for this reason, we need to
        // lock and poll ...
        let term = sync::Arc::new(sync::Mutex::new(rustty::Terminal::new().unwrap()));

        let (action_send, action_recv) = sync::mpsc::channel();
        let running = sync::Arc::new(sync::atomic::AtomicBool::new(true));

        // run setup function
        setup(action_send.clone());

        // start background thread that turns events into actions
        let bg_term = term.clone();
        let bg_running = running.clone();
        let bg_thread = thread::spawn(move || {
            loop {
                // check if app is alive
                if !bg_running.load(sync::atomic::Ordering::Relaxed) {
                    break;
                }

                let mut t = bg_term.lock().unwrap();
                match t.get_event(time::Duration::from_millis(10)).unwrap() {
                    None => continue,
                    Some(ev) => {
                        let act: <T as Application>::Action = ev.into();
                        action_send.send(act).unwrap();
                    }
                }
            }
        });

        // draw once initially
        draw_view(term.clone(), &app);

        for action in action_recv.iter() {
            // update state according to action
            let (napp, ntask) = app.handle_action(action);
            app = napp;

            if let Some(task) = ntask {
                if app.exec_task(task) {
                    break;
                }
            }

            let now = time::Instant::now();

            if let Some(min_delay) = self.min_delay {
                if now - self.last_draw < min_delay {
                    // do not redraw, updating too fast
                    continue;
                }
            }

            // redraw
            self.last_draw = now;
            draw_view(term.clone(), &app);
        }

        // shutdown background thread for clean terminal restoration
        running.store(false, sync::atomic::Ordering::Relaxed);
        bg_thread.join().unwrap();
    }
}
