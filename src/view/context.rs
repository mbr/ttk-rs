use bresenham::Bresenham;

use super::Style;
use rustty::{Cell, Pos, Size, Terminal};
use std::ascii::AsciiExt;
use std::ops::{Index, IndexMut};
use std::cmp::min;

pub struct DrawingContext<'a> {
    states: Vec<DrawingContextState>,
    term: &'a mut Terminal,
}

struct DrawingContextState {
    translation: Pos,
    size: Size,
}

impl<'a> DrawingContext<'a> {
    pub fn new(term: &mut Terminal) -> DrawingContext {
        DrawingContext {
            states: vec![DrawingContextState {
                             translation: (0, 0),
                             size: (term.cols(), term.rows()),
                         }],
            term: term,
        }

    }

    #[inline(always)]
    pub fn translation(&self) -> Pos {
        self.states.last().unwrap().translation
    }

    #[inline(always)]
    pub fn size(&self) -> Size {
        self.states.last().unwrap().size
    }

    #[inline]
    pub fn translate(&mut self, p: Pos) {
        self.shrink(p);
        let cur = self.states.last_mut().unwrap();
        cur.translation.0 += p.0;
        cur.translation.1 += p.1;
    }

    pub fn shrink(&mut self, s: Size) {
        let cur = self.states.last_mut().unwrap();
        if cur.size.0 >= s.0 {
            cur.size.0 -= s.0
        }
        if cur.size.1 >= s.1 {
            cur.size.1 -= s.1
        }
    }

    pub fn clip(&mut self, s: Size) {
        let cur = self.states.last_mut().unwrap();

        cur.size.0 = min(cur.size.0, s.0);
        cur.size.1 = min(cur.size.1, s.1);
    }

    #[inline(always)]
    pub fn transform(&self, mut p: Pos) -> Option<Pos> {
        // Size-check bounding box is not violated
        if p.0 > self.size().0 || p.1 > self.size().0 {
            return None;
        }

        // translate
        p.0 += self.translation().0;
        p.1 += self.translation().1;

        // check we're not drawing off-screen
        if p.0 < self.term.cols() && p.1 < self.term.rows() {
            Some(p)
        } else {
            None
        }
    }

    #[inline]
    pub fn save(&mut self) {
        let new_state = DrawingContextState {
            translation: self.translation(),
            size: self.size(),
        };
        self.states.push(new_state)
    }

    #[inline]
    pub fn restore(&mut self) {
        if self.states.len() <= 1 {
            panic!("Empty DrawingContext popped");
        }
        self.states.pop().unwrap();
    }

    #[inline]
    pub fn get_cell(&self, p: Pos) -> Option<&Cell> {
        self.transform(p).map(move |q| self.term.index(q))
    }

    #[inline]
    pub fn get_mut_cell(&mut self, p: Pos) -> Option<&mut Cell> {
        self.transform(p).map(move |q| self.term.index_mut(q))
    }

    #[inline]
    pub fn set_cell(&mut self, p: Pos, cell: Cell) {
        if let Some(p) = self.transform(p) {
            self.term[p] = cell;
        }
    }

    pub fn fill(&mut self, cell: Cell) {
        let (w, h) = self.size();
        let (x0, y0) = self.translation();

        for x in x0..min(self.term.cols(), (w + x0)) {
            for y in y0..min(self.term.rows(), (h + y0)) {
                self.term[(x, y)] = cell;
            }
        }
    }

    pub fn line(&mut self, start: Pos, end: Pos, cell: Cell) {
        let line = Bresenham::new((start.0 as isize, start.1 as isize),
                                  (end.0 as isize, end.1 as isize));
        for p in line {
            // FIXME: check bounds earlier to have faster access
            self.set_cell((p.0 as usize, p.1 as usize), cell);
        }
    }

    pub fn text<S: AsRef<str>>(&mut self, start: Pos, text: S, style: Style) {
        let s: &str = text.as_ref();

        // FIXME: unicoe support
        assert!(s.is_ascii());

        // FIXME: check bounds earlier to avoid drawing useless chars?
        for (idx, ch) in s.chars().enumerate() {
            self.set_cell((start.0 + idx, start.1),
                          Cell::new(ch, style.0, style.1, style.2));
        }
    }
}
