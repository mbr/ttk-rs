use rustty::Cell;
use super::{DefaultStyle, DrawingContext, Style, Widget};

// FIXME: make other parts of framework use builder patterns too?
pub struct IndicatorButton<'a> {
    enabled: bool,
    text: &'a str,
    hotkey: Option<&'a str>,
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
            hotkey: None,
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
        self.text = text;
        self
    }

    #[inline]
    pub fn hotkey(mut self, hotkey: Option<&'a str>) -> IndicatorButton<'a> {
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
        // first, draw hotkey
        // ctx.text("")

    }
}
