extern crate bresenham;
extern crate rustty;

use rustty::{Attr, Cell, Color, Terminal};
use std::{thread, time};

mod context;
pub mod controls;
pub mod draw;
pub mod layout;
pub mod table;
mod transform;
pub mod window;

pub use context::DrawingContext;
pub use transform::{FixedSize, offset, sized, Translated};

use draw::Background;
use window::FramedWindow;

pub type Style = (Color, Color, Attr);
const DEFAULT_STYLE: Style = (Color::Default, Color::Default, Attr::Default);

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
    // create a simple model for testing right now`
    let mut tbl_model =
        table::SimpleModel::new(vec!["First".to_owned(), "Second".to_owned(), "Third".to_owned()]);

    tbl_model.push_row(vec!["1234".to_owned(), "5678".to_owned(), "90ab".to_owned()]);
    tbl_model.push_row(vec!["1234".to_owned(), "5678".to_owned(), "90ab".to_owned()]);
    tbl_model.push_row(vec!["1234".to_owned(), "5678".to_owned(), "90ab".to_owned()]);
    tbl_model.push_row(vec!["1234".to_owned(), "5678".to_owned(), "90ab".to_owned()]);
    tbl_model.push_row(vec!["1234".to_owned(), "5678".to_owned(), "90ab".to_owned()]);

    for i in 0..1000 {
        tbl_model.push_row(vec!["auto-generated row".to_owned(),
                                format!("row num: {}", i),
                                "".to_owned()])
    }

    let mut vbox = Box::new(layout::VBox::new());
    vbox.push_item(layout::BoxItem::Fixed(1, Box::new(Background::new(Cell::with_char('1')))));

    let mut main = layout::Layers::new();
    main.push_widget(Box::new(Background::new(Cell::with_char('.'))));

    let table_view = Box::new(table::TableView::new(&tbl_model, vec![12, 10, -1]));
    vbox.push_item(layout::BoxItem::Expand(table_view));


    let btn1 = Box::new(controls::IndicatorButton::new()
        .text("Hello")
        .size(12)
        .hotkey("FO"));

    let btn2 = Box::new(controls::IndicatorButton::new()
        .text("World")
        .size(12)
        .hotkey("Wo"));

    let btn3 = Box::new(controls::IndicatorButton::new()
        .size(12)
        .text("Meep"));

    let mut hbox = Box::new(layout::HBox::new());
    hbox.push_item(layout::BoxItem::Fixed(14, btn1));
    hbox.push_item(layout::BoxItem::Fixed(14, btn2));
    hbox.push_item(layout::BoxItem::Fixed(14, btn3));

    vbox.push_item(layout::BoxItem::Fixed(1, hbox));

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
