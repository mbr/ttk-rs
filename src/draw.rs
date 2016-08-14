use rustty::Cell;
use super::{DrawingContext, Widget};


pub struct Background {
    bg_cell: Cell,
}

impl Background {
    pub fn new(cell: Cell) -> Background {
        Background { bg_cell: cell }
    }
}

impl Widget for Background {
    fn draw_on(&self, ctx: &mut DrawingContext) {
        ctx.fill(self.bg_cell)
    }
}
