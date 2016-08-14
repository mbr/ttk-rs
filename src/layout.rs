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

pub enum VBoxItem {
    Fixed(usize, Box<Widget>),
    Expand(Box<Widget>),
}

pub struct VBox {
    widgets: Vec<VBoxItem>,
}

impl VBox {
    pub fn new() -> VBox {
        VBox { widgets: Vec::new() }
    }

    pub fn push_item(&mut self, item: VBoxItem) {
        self.widgets.push(item)
    }
}

impl Widget for VBox {
    fn draw_on(&self, ctx: &mut DrawingContext) {
        // count fixed lines
        let fixed_lines = self.widgets
            .iter()
            .map(|item| match item {
                &VBoxItem::Fixed(n, _) => n,
                _ => 0,
            })
            .fold(0, |n, m| n + m);

        let expand_size = ctx.size().1.saturating_sub(fixed_lines);

        let width = ctx.size().0;
        let mut y = 0;
        for item in self.widgets.iter() {
            let (h, widget) = match item {
                &VBoxItem::Fixed(h, ref widget) => (h, widget),
                &VBoxItem::Expand(ref widget) => (expand_size, widget),
            };

            ctx.save();
            ctx.translate((0, y));
            ctx.clip((width, h));
            // FIXME: clip by shrinking
            widget.draw_on(ctx);
            ctx.restore();

            y += h;

            // we went off-screen, stop drawing
            if y >= ctx.size().1 {
                break;
            }
        }
    }
}
