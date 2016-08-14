extern crate rustty;

use rustty::{Cell, CellAccessor, Pos, Size, Terminal};
use std::{thread, time};
use std::ops::{Index, IndexMut};
use std::convert::AsRef;

pub struct DrawingContext<'a> {
    states: Vec<DrawingContextState>,
    term: &'a mut Terminal,
}

struct DrawingContextState {
    translation: Pos,
    size: Size,
}

impl<'a> DrawingContext<'a> {
    pub fn new(term: &mut Terminal) -> DrawingContext {
        DrawingContext {
            states: vec![DrawingContextState {
                             translation: (0, 0),
                             size: (term.cols(), term.rows()),
                         }],
            term: term,
        }

    }

    #[inline(always)]
    fn translation(&self) -> Pos {
        self.states.last().unwrap().translation
    }

    #[inline(always)]
    fn size(&self) -> Size {
        self.states.last().unwrap().size
    }

    #[inline]
    fn translate(&mut self, p: Pos) {
        self.shrink(p);
        let cur = self.states.last_mut().unwrap();
        cur.translation.0 += p.0;
        cur.translation.1 += p.1;
    }

    fn shrink(&mut self, s: Size) {
        let cur = self.states.last_mut().unwrap();
        if cur.size.0 >= s.0 {
            cur.size.0 -= s.0
        }
        if cur.size.1 >= s.1 {
            cur.size.1 -= s.1
        }
    }

    #[inline(always)]
    fn transform(&self, mut p: Pos) -> Option<Pos> {
        // Size-check bounding box is not violated
        if p.0 > self.size().0 || p.1 > self.size().0 {
            return None;
        }

        // translate
        p.0 += self.translation().0;
        p.1 += self.translation().1;

        // check we're not drawing off-screen
        if p.0 < self.term.cols() && p.1 < self.term.rows() {
            Some(p)
        } else {
            None
        }
    }

    #[inline]
    pub fn save(&mut self) {
        let new_state = DrawingContextState {
            translation: self.translation(),
            size: self.size(),
        };
        self.states.push(new_state)
    }

    #[inline]
    pub fn restore(&mut self) {
        if self.states.len() <= 1 {
            panic!("Empty DrawingContext popped");
        }
        self.states.pop().unwrap();
    }

    #[inline]
    pub fn get_cell(&self, p: Pos) -> Option<&Cell> {
        self.transform(p).map(move |q| self.term.index(q))
    }

    #[inline]
    pub fn get_mut_cell(&mut self, p: Pos) -> Option<&mut Cell> {
        self.transform(p).map(move |q| self.term.index_mut(q))
    }

    #[inline]
    pub fn set_cell(&mut self, p: Pos, cell: Cell) {
        if let Some(p) = self.transform(p) {
            self.term[p] = cell;
        }
    }

    pub fn fill(&mut self, cell: Cell) {
        let (w, h) = self.size();
        let (x0, y0) = self.translation();

        for x in 0..w {
            for y in 0..h {
                self.term[(x + x0, y + y0)] = cell;
            }
        }
    }
}

struct Model {

}

impl Model {
    fn new() -> Model {
        Model {}
    }
}

pub trait Widget {
    fn draw_on(&self, &mut DrawingContext);
}

struct Background {
    bg_cell: Cell,
}

impl Background {
    fn new(cell: Cell) -> Background {
        Background { bg_cell: cell }
    }
}

impl Widget for Background {
    fn draw_on(&self, ctx: &mut DrawingContext) {
        ctx.fill(self.bg_cell)
    }
}

pub struct Layers {
    widgets: Vec<Box<Widget>>,
}

impl Layers {
    pub fn new() -> Layers {
        Layers { widgets: Vec::new() }
    }

    pub fn push_widget(&mut self, w: Box<Widget>) {
        self.widgets.push(w)
    }
}

impl Widget for Layers {
    fn draw_on(&self, ctx: &mut DrawingContext) {
        for w in self.widgets.iter() {
            w.draw_on(ctx)
        }
    }
}

pub struct FramedWindow {
    bg_cell: Cell,
    frame_cell: Cell,
}

impl FramedWindow {
    pub fn new(bg_cell: Cell, frame_cell: Cell) -> FramedWindow {
        FramedWindow {
            bg_cell: bg_cell,
            frame_cell: frame_cell,
        }
    }
}

impl Widget for FramedWindow {
    fn draw_on(&self, ctx: &mut DrawingContext) {
        ctx.fill(self.bg_cell)
    }
}

pub struct FixedSize {
    size: Size,
    widget: Box<Widget>,
}

impl FixedSize {
    pub fn new(s: Size, w: Box<Widget>) -> FixedSize {
        FixedSize {
            size: s,
            widget: w,
        }
    }
}

impl Widget for FixedSize {
    fn draw_on(&self, ctx: &mut DrawingContext) {
        ctx.save();
        let shrink_x = if ctx.size().0 > self.size.0 {
            ctx.size().0 - self.size.0
        } else {
            0
        };

        let shrink_y = if ctx.size().1 > self.size.1 {
            ctx.size().1 - self.size.1
        } else {
            0
        };

        ctx.shrink((shrink_x, shrink_y));
        self.widget.draw_on(ctx);
        ctx.restore();
    }
}

pub struct Translated {
    offset: Pos,
    widget: Box<Widget>,
}

impl Translated {
    fn new(o: Pos, w: Box<Widget>) -> Translated {
        Translated {
            offset: o,
            widget: w,
        }
    }
}

impl Widget for Translated {
    fn draw_on(&self, ctx: &mut DrawingContext) {
        ctx.save();
        ctx.translate(self.offset);
        self.widget.draw_on(ctx);
        ctx.restore();
    }
}

// FIXME: not working
// trait MoveSize {
//     fn offset(self, offset: Pos) -> Box<Widget>;
//     fn sized(self, size: Size) -> Box<Widget>;
// }

// impl<T: 'static + Widget + ?Sized> MoveSize for Box<T> {
//     fn offset(self, offset: Pos) -> Box<Widget> {
//         Box::new(Translated::new(offset, self))
//     }
//     fn sized(self, size: Size) -> Box<Widget> {
//         Box::new(FixedSize::new(size, self))
//     }
// }

fn offset(offset: Pos, widget: Box<Widget>) -> Box<Widget> {
    Box::new(Translated::new(offset, widget))
}

fn sized(size: Size, widget: Box<Widget>) -> Box<Widget> {
    Box::new(FixedSize::new(size, widget))
}

// later on, optimize this by caching, using Rc and such and returning
// the same widgets over and over? also, actual rendering could optimize the
// drawing as well?
fn draw(term: &mut Terminal, model: &Model) {
    let mut main = Layers::new();
    main.push_widget(Box::new(Background::new(Cell::with_char('.'))));
    main.push_widget(offset((4, 5),
                            sized((7, 8),
                                  Box::new(FramedWindow::new(Cell::with_char(' '),
                                                             Cell::with_char('+'))))));
    {
        // create new context and draw upon it
        let mut ctx = DrawingContext::new(term);
        main.draw_on(&mut ctx);
    }

    term.swap_buffers();
}

fn main() {
    // initialize terminal

    let mut term = Terminal::new().unwrap();

    let model = Model::new();

    loop {
        thread::sleep(time::Duration::from_millis(100));
        draw(&mut term, &model);
    }

}
