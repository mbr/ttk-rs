use rustty::{Attr, Color};

mod context;
pub mod controls;
pub mod draw;
pub mod layout;
pub mod table;
mod transform;
pub mod window;

pub use self::context::DrawingContext;
pub use self::transform::{FixedSize, offset, sized, Translated};

pub type Style = (Color, Color, Attr);
const DEFAULT_STYLE: Style = (Color::Default, Color::Default, Attr::Default);

pub trait Widget {
    fn draw_on(&self, &mut DrawingContext);
}
