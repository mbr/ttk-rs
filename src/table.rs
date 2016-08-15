pub type RowIter<'a> = Box<Iterator<Item = &'a str>>;

trait TableModel<'a> {
    fn headers(&'a self) -> Box<Iterator<Item = &'a str> + 'a>;
    fn get_row(&'a self, row_id: usize) -> Box<Iterator<Item = &'a str> + 'a>;
    fn num_rows(&self) -> usize;
    fn num_cols(&self) -> usize;
}

struct SimpleModel {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
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
