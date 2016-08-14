extern crate bresenham;
extern crate rustty;

use bresenham::Bresenham;
use rustty::{Cell, CellAccessor, Pos, Size, Terminal};
use std::{thread, time};
use std::convert::AsRef;

mod context;
pub use context::DrawingContext;

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
        let csize = (ctx.size().0 as isize, ctx.size().1 as isize);

        if csize.0 < 2 || csize.1 < 2 {
            return;
        }

        let top = 0;
        let right = csize.0 - 1;
        let bottom = csize.1 - 1;
        let left = 0;

        ctx.fill(self.bg_cell);

        // top
        for (x, y) in Bresenham::new((left, top), (right, top)) {
            ctx.set_cell((x as usize, y as usize), self.frame_cell);
        }
        // right
        for (x, y) in Bresenham::new((right, top), (right, bottom)) {
            ctx.set_cell((x as usize, y as usize), self.frame_cell);
        }
        // bottom
        for (x, y) in Bresenham::new((right, bottom), (left, bottom)) {
            ctx.set_cell((x as usize, y as usize), self.frame_cell);
        }
        // left
        for (x, y) in Bresenham::new((left, bottom), (left, top)) {
            ctx.set_cell((x as usize, y as usize), self.frame_cell);
        }
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
