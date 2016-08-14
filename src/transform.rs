use rustty::{Pos, Size};
use super::{DrawingContext, Widget};


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

pub fn offset(offset: Pos, widget: Box<Widget>) -> Box<Widget> {
    Box::new(Translated::new(offset, widget))
}

pub fn sized(size: Size, widget: Box<Widget>) -> Box<Widget> {
    Box::new(FixedSize::new(size, widget))
}
