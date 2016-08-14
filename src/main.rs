extern crate bresenham;
extern crate rustty;

use rustty::{Cell, Terminal};
use std::{thread, time};

mod context;
pub mod draw;
pub mod layout;
mod transform;
pub mod window;

pub use context::DrawingContext;
pub use transform::{FixedSize, offset, sized, Translated};

use draw::Background;
use window::FramedWindow;

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


// later on, optimize this by caching, using Rc and such and returning
// the same widgets over and over? also, actual rendering could optimize the
// drawing as well?
fn draw(term: &mut Terminal, model: &Model) {
    let mut main = layout::Layers::new();
    main.push_widget(Box::new(Background::new(Cell::with_char('.'))));

    let mut vbox = Box::new(layout::VBox::new());
    vbox.push_item(layout::VBoxItem::Fixed(1, Box::new(Background::new(Cell::with_char('1')))));
    vbox.push_item(layout::VBoxItem::Fixed(2, Box::new(Background::new(Cell::with_char('2')))));
    vbox.push_item(layout::VBoxItem::Fixed(3, Box::new(Background::new(Cell::with_char('3')))));
    vbox.push_item(layout::VBoxItem::Expand(Box::new(Background::new(Cell::with_char('*')))));
    vbox.push_item(layout::VBoxItem::Fixed(4, Box::new(Background::new(Cell::with_char('4')))));


    main.push_widget(vbox);
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
