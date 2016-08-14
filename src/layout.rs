use std::slice;
use super::{DrawingContext, Widget};

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

pub enum BoxItem {
    Fixed(usize, Box<Widget>),
    Expand(Box<Widget>),
}

struct BoxLayout(Vec<BoxItem>);

pub struct BoxLayoutIter<'a> {
    layout_iter: slice::Iter<'a, BoxItem>,
    expand_size: usize,
}

impl<'a> Iterator for BoxLayoutIter<'a> {
    type Item = (usize, &'a Box<Widget>);

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

impl BoxLayout {
    fn new() -> BoxLayout {
        BoxLayout(Vec::new())
    }

    fn iter_sized_items(&self, total_space: usize) -> BoxLayoutIter {
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
    fn push_item(&mut self, item: BoxItem) {
        self.0.push(item)
    }
}

pub struct VBox(BoxLayout);

impl VBox {
    pub fn new() -> VBox {
        VBox(BoxLayout::new())
    }

    pub fn push_item(&mut self, item: BoxItem) {
        self.0.push_item(item)
    }
}

impl Widget for VBox {
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

pub struct HBox(BoxLayout);

impl HBox {
    pub fn new() -> HBox {
        HBox(BoxLayout::new())
    }

    pub fn push_item(&mut self, item: BoxItem) {
        self.0.push_item(item)
    }
}

impl Widget for HBox {
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
