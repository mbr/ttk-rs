use rustty::Cell;
use std::ascii::AsciiExt;
use std::cmp::min;
use super::{DefaultStyle, DrawingContext, Style, Widget};

// FIXME: make other parts of framework use builder patterns too?
pub struct IndicatorButton<'a> {
    enabled: bool,
    text: &'a str,
    hotkey: &'a str,
    size: usize,

    // FIXME: find pattern for style object, maybe using trait?
    active_style: Style,
    inactive_style: Style,
    hotkey_style: Style,
}

impl<'a> IndicatorButton<'a> {
    #[inline]
    pub fn new() -> IndicatorButton<'a> {
        IndicatorButton {
            enabled: false,
            text: "",
            hotkey: "",
            size: 12,
            active_style: DefaultStyle,
            inactive_style: DefaultStyle,
            hotkey_style: DefaultStyle,
        }
    }

    #[inline]
    pub fn enabled(mut self, enabled: bool) -> IndicatorButton<'a> {
        self.enabled = enabled;
        self
    }

    #[inline]
    pub fn text(mut self, text: &'a str) -> IndicatorButton<'a> {
        assert!(text.is_ascii());
        self.text = text;
        self
    }

    #[inline]
    pub fn hotkey(mut self, hotkey: &'a str) -> IndicatorButton<'a> {
        assert!(hotkey.is_ascii());
        self.hotkey = hotkey;
        self
    }

    #[inline]
    pub fn size(mut self, size: usize) -> IndicatorButton<'a> {
        self.size = size;
        self
    }

    #[inline]
    pub fn active_style(mut self, style: Style) -> IndicatorButton<'a> {
        self.active_style = style;
        self
    }

    #[inline]
    pub fn inactive_style(mut self, style: Style) -> IndicatorButton<'a> {
        self.inactive_style = style;
        self
    }

    #[inline]
    pub fn hotkey_style(mut self, style: Style) -> IndicatorButton<'a> {
        self.hotkey_style = style;
        self
    }
}

impl<'a> Widget for IndicatorButton<'a> {
    fn draw_on(&self, ctx: &mut DrawingContext) {
        let btn_offset = min(self.size, self.hotkey.len());

        // first, draw hotkey
        ctx.text((0, 0), &self.hotkey[0..btn_offset], self.hotkey_style);

        // then draw button
        ctx.text((btn_offset, 0),
                 &self.text[0..min(self.size - btn_offset, self.text.len())],
                 if self.enabled {
                     self.active_style
                 } else {
                     self.inactive_style
                 });
    }
}
