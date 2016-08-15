use rustty::{Pos, Size};
use super::{DrawingContext, Widget};


pub struct FixedSize<'a> {
    size: Size,
    widget: Box<Widget + 'a>,
}

impl<'a> FixedSize<'a> {
    pub fn new(s: Size, w: Box<Widget + 'a>) -> FixedSize<'a> {
        FixedSize {
            size: s,
            widget: w,
        }
    }
}

impl<'a> Widget for FixedSize<'a> {
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

pub struct Translated<'a> {
    offset: Pos,
    widget: Box<Widget + 'a>,
}

impl<'a> Translated<'a> {
    fn new(o: Pos, w: Box<Widget + 'a>) -> Translated<'a> {
        Translated {
            offset: o,
            widget: w,
        }
    }
}

impl<'a> Widget for Translated<'a> {
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

pub fn offset<'a>(offset: Pos, widget: Box<Widget + 'a>) -> Box<Widget + 'a> {
    Box::new(Translated::new(offset, widget))
}

pub fn sized<'a>(size: Size, widget: Box<Widget + 'a>) -> Box<Widget + 'a> {
    Box::new(FixedSize::new(size, widget))
}
