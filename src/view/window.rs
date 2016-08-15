use rustty::Cell;
use std::ascii::AsciiExt;
use std::cmp::min;
use super::{DEFAULT_STYLE, DrawingContext, Style, Widget};


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


pub struct MessageFill<'a> {
    bg_cell: Cell,
    msg: &'a str,
    padding: usize,
    style: Style,
}

impl<'a> MessageFill<'a> {
    pub fn new() -> MessageFill<'a> {
        MessageFill {
            bg_cell: Cell::new(' ', DEFAULT_STYLE.0, DEFAULT_STYLE.1, DEFAULT_STYLE.2),
            msg: "",
            padding: 2,
            style: DEFAULT_STYLE,
        }
    }

    pub fn bg_cell(mut self, bg_cell: Cell) -> MessageFill<'a> {
        self.bg_cell = bg_cell;
        self
    }

    pub fn msg(mut self, msg: &'a str) -> MessageFill<'a> {
        self.msg = msg;
        self
    }

    pub fn padding(mut self, padding: usize) -> MessageFill<'a> {
        self.padding = padding;
        self
    }

    pub fn style(mut self, style: Style) -> MessageFill<'a> {
        self.style = style;
        self
    }
}

impl<'a> Widget for MessageFill<'a> {
    fn draw_on(&self, ctx: &mut DrawingContext) {
        assert!(self.msg.is_ascii());
        // FIXME: proper word-wrapping would be good here
        // get line length

        let (cols, rows) = ctx.size();

        if cols <= self.padding * 2 || rows < 1 {
            return;
        }

        let tx_row_len = cols - 2 * self.padding;

        let tx_rows = self.msg.len() % tx_row_len;

        for n in 0..tx_rows {
            let start = min(n * tx_row_len, self.msg.len());
            let end = min(self.msg.len(), start + tx_row_len);
            ctx.text((tx_row_len / 2 + self.padding, rows.saturating_sub(tx_rows) / 2 + n),
                     &self.msg[start..end],
                     self.style)
        }

    }
}
