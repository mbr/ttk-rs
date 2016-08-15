use std::slice;
use super::{DrawingContext, Widget};

pub struct Layers<'a> {
    widgets: Vec<Box<Widget + 'a>>,
}

impl<'a> Layers<'a> {
    pub fn new() -> Layers<'a> {
        Layers { widgets: Vec::new() }
    }

    pub fn push_widget(&mut self, w: Box<Widget + 'a>) {
        self.widgets.push(w)
    }
}

impl<'a> Widget for Layers<'a> {
    fn draw_on(&self, ctx: &mut DrawingContext) {
        for w in self.widgets.iter() {
            w.draw_on(ctx)
        }
    }
}

pub enum BoxItem<'a> {
    Fixed(usize, Box<Widget + 'a>),
    Expand(Box<Widget + 'a>),
}

pub struct BoxLayout<'a>(Vec<BoxItem<'a>>);

pub struct BoxLayoutIter<'a> {
    layout_iter: slice::Iter<'a, BoxItem<'a>>,
    expand_size: usize,
}

impl<'a> Iterator for BoxLayoutIter<'a> {
    type Item = (usize, &'a Box<Widget + 'a>);

    fn next(&mut self) -> Option<Self::Item> {
        let item = match self.layout_iter.next() {
            None => return None,
            Some(v) => v,
        };

        let (size, widget) = match item {
            &BoxItem::Fixed(size, ref widget) => (size, widget),
            &BoxItem::Expand(ref widget) => (self.expand_size, widget),
        };

        Some((size, widget))
    }
}

impl<'a> BoxLayout<'a> {
    fn new() -> BoxLayout<'a> {
        BoxLayout(Vec::new())
    }

    fn iter_sized_items(&'a self, total_space: usize) -> BoxLayoutIter<'a> {
        // add up fixed item space usage
        let fixed_items = self.0
            .iter()
            .map(|item| match item {
                &BoxItem::Fixed(n, _) => n,
                _ => 0,
            })
            .fold(0, |n, m| n + m);

        BoxLayoutIter {
            layout_iter: self.0.iter(),
            expand_size: total_space.saturating_sub(fixed_items),
        }
    }

    #[inline]
    fn push_item(&'a mut self, item: BoxItem<'a>) {
        self.0.push(item)
    }
}

pub struct VBox<'a>(BoxLayout<'a>);

impl<'a> VBox<'a> {
    pub fn new() -> VBox<'a> {
        VBox(BoxLayout::new())
    }

    pub fn push_item(&'a mut self, item: BoxItem<'a>) {
        self.0.push_item(item)
    }
}

impl<'a> Widget for VBox<'a> {
    fn draw_on(&self, ctx: &mut DrawingContext) {
        let (width, height) = ctx.size();

        let mut y = 0;

        for (item_height, widget) in self.0.iter_sized_items(height) {
            ctx.save();
            ctx.translate((0, y));
            ctx.clip((width, item_height));
            // FIXME: clip by shrinking
            widget.draw_on(ctx);
            ctx.restore();

            y += item_height;

            // we went off-screen, stop drawing
            if y >= height {
                break;
            }
        }
    }
}

pub struct HBox<'a>(BoxLayout<'a>);

impl<'a> HBox<'a> {
    pub fn new() -> HBox<'a> {
        HBox(BoxLayout::new())
    }

    pub fn push_item(&'a mut self, item: BoxItem<'a>) {
        self.0.push_item(item)
    }
}

impl<'a> Widget for HBox<'a> {
    fn draw_on(&self, ctx: &mut DrawingContext) {
        let (width, height) = ctx.size();

        let mut x = 0;

        for (item_width, widget) in self.0.iter_sized_items(width) {
            ctx.save();
            ctx.translate((x, 0));
            ctx.clip((item_width, height));
            // FIXME: clip by shrinking
            widget.draw_on(ctx);
            ctx.restore();

            x += item_width;

            // we went off-screen, stop drawing
            if x >= width {
                break;
            }
        }
    }
}
