use std::ascii::AsciiExt;
use std::cmp::{max, min};
use super::{DEFAULT_STYLE, DrawingContext, Style, Widget};

pub type RowIter<'a> = Box<Iterator<Item = &'a str>>;

pub trait TableModel<'a> {
    fn headers(&'a self) -> Box<Iterator<Item = &'a str> + 'a>;
    fn get_row(&'a self, row_id: usize) -> Box<Iterator<Item = &'a str> + 'a>;
    fn num_rows(&self) -> usize;
    fn num_cols(&self) -> usize;
}

pub struct SimpleModel {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
}

impl SimpleModel {
    pub fn new(headers: Vec<String>) -> SimpleModel {
        SimpleModel {
            headers: headers,
            rows: Vec::new(),
        }
    }

    pub fn push_row(&mut self, row: Vec<String>) {
        self.rows.push(row)
    }
}

impl<'a> TableModel<'a> for SimpleModel {
    fn headers(&'a self) -> Box<Iterator<Item = &'a str> + 'a> {
        Box::new(self.headers.iter().map(|x| x.as_str()))
    }

    fn get_row(&'a self, row_id: usize) -> Box<Iterator<Item = &'a str> + 'a> {
        Box::new(self.rows[row_id].iter().map(|x| x.as_str()))
    }

    fn num_rows(&self) -> usize {
        self.rows.len()
    }

    fn num_cols(&self) -> usize {
        self.headers.len()
    }
}

pub struct TableView<'a> {
    model: &'a TableModel<'a>,
    col_width: Vec<i32>,
    header_style: Style,
    cell_style: Style,
    offset: usize,
}

impl<'a> TableView<'a> {
    pub fn new(model: &'a TableModel<'a>, col_width: Vec<i32>) -> TableView<'a> {
        assert_eq!(model.num_cols(), col_width.len());
        TableView {
            model: model,
            col_width: col_width,
            header_style: DEFAULT_STYLE,
            cell_style: DEFAULT_STYLE,
            offset: 0,
        }
    }

    pub fn header_style(mut self, style: Style) -> TableView<'a> {
        self.header_style = style;
        self
    }

    pub fn cell_style(mut self, style: Style) -> TableView<'a> {
        self.cell_style = style;
        self
    }

    pub fn offset(mut self, offset: usize) -> TableView<'a> {
        self.offset = offset;
        self
    }
}

impl<'a> Widget for TableView<'a> {
    fn draw_on(&self, ctx: &mut DrawingContext) {
        let (cols, rows) = ctx.size();

        if rows == 0 {
            return;
        }

        // FIXME: Solve this using a share component with Layout (needs to be
        //        decoupled from widget?

        let expand_size = cols.saturating_sub(self.col_width
            .iter()
            .map(|&z| max(z, 0))
            .fold(0, |n, m| n + m) as usize);

        // cache widths
        let widths: Vec<_> = self.col_width
            .iter()
            .map(|&z| if z < 0 { expand_size } else { z as usize })
            .collect();

        // first, draw header
        let mut x_pos = 0;
        for (header, &width) in self.model.headers().zip(widths.iter()) {
            // FIXME
            assert!(header.is_ascii());
            ctx.text((x_pos, 0),
                     &header[0..min(header.len(), width)],
                     self.header_style);
            x_pos += width;
        }

        // then, draw data rows
        if rows < 1 {
            return;
        }

        let num_rows = self.model.num_rows();
        for i in 0..(rows - 1) {
            let row_pos = i + 1;
            let row_idx = i + self.offset;

            if row_idx >= num_rows {
                break; // nothing more to draw, exit
            }

            x_pos = 0;
            for (cell, &width) in self.model.get_row(row_idx).zip(widths.iter()) {
                assert!(cell.is_ascii());
                ctx.text((x_pos, row_pos),
                         &cell[0..min(cell.len(), width)],
                         self.cell_style);
                x_pos += width;
            }
        }
    }
}
