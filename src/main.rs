extern crate rustty;

use rustty::{Cell, CellAccessor, Pos, Size, Terminal};
use std::{thread, time};
use std::ops::{Index, IndexMut};

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
    fn translation(&self) -> Pos {
        self.states.last().unwrap().translation
    }

    #[inline(always)]
    fn size(&self) -> Size {
        self.states.last().unwrap().size
    }

    #[inline]
    fn translate(&mut self, p: Pos) {
        self.shrink(p);
        let cur = self.states.last_mut().unwrap();
        cur.translation.0 += p.0;
        cur.translation.1 += p.1;
    }

    fn shrink(&mut self, s: Size) {
        let cur = self.states.last_mut().unwrap();
        if cur.size.0 >= s.0 {
            cur.size.0 -= s.0
        }
        if cur.size.1 >= s.1 {
            cur.size.1 -= s.1
        }
    }

    #[inline(always)]
    fn transform(&self, mut p: Pos) -> Option<Pos> {
        p.0 += self.translation().0;
        p.1 += self.translation().1;

        if p.0 < self.size().0 && p.1 < self.size().1 {
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
    pub fn pop(&mut self) {
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

        for x in 0..w {
            for y in 0..h {
                self.term[(x + x0, y + y0)] = cell;
            }
        }
    }
}

struct Model {

}

impl Model {
    fn new() -> Model {
        Model {}
    }
}

trait Widget {
    fn draw_on(&self, &mut DrawingContext);
}

struct Background {
    bg_cell: Cell,
}

impl Background {
    fn new(cell: Cell) -> Background {
        Background { bg_cell: cell }
    }
}

impl Widget for Background {
    fn draw_on(&self, ctx: &mut DrawingContext) {
        ctx.translate((2, 3));
        ctx.fill(self.bg_cell)
    }
}

// later on, optimize this by caching, using Rc and such and returning
// the same widgets over and over? also, actual rendering could optimize the
// drawing as well?
fn draw(term: &mut Terminal, model: &Model) {

    let bg = Background::new(Cell::with_char('x'));

    {
        // create new context and draw upon it
        let mut ctx = DrawingContext::new(term);
        bg.draw_on(&mut ctx);
    }

    term.swap_buffers();
}

fn main() {
    // initialize terminal

    let mut term = Terminal::new().unwrap();

    let model = Model::new();

    loop {
        thread::sleep(time::Duration::from_millis(100));
        draw(&mut term, &model);
    }

}
