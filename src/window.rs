use rustty::Cell;
use super::{DrawingContext, Widget};


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
