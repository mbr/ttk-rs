extern crate bresenham;
extern crate rustty;

use rustty::{Cell, Terminal};
use std::{thread, time};

mod context;
pub mod layout;
mod transform;

pub use context::DrawingContext;
pub use transform::{FixedSize, offset, sized, Translated};

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
        let csize = ctx.size();

        if csize.0 < 2 || csize.1 < 2 {
            return;
        }

        let top = 0;
        let right = csize.0 - 1;
        let bottom = csize.1 - 1;
        let left = 0;

        ctx.fill(self.bg_cell);

        // draw border
        ctx.line((left, top), (right, top), self.frame_cell);
        ctx.line((right, top), (right, bottom), self.frame_cell);
        ctx.line((right, bottom), (left, bottom), self.frame_cell);
        ctx.line((left, bottom), (left, top), self.frame_cell);
    }
}

// later on, optimize this by caching, using Rc and such and returning
// the same widgets over and over? also, actual rendering could optimize the
// drawing as well?
fn draw(term: &mut Terminal, model: &Model) {
    let mut main = layout::Layers::new();
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

    term.swap_buffers().unwrap();
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
