use std::vec::Vec;
use std::fmt::{Show, Formatter, Result};
use self::dynamic_connectivity::{QuickFind, QuickUnion, WeightedQuickUnion};

//how do I remove this?
#[path = "dynamic_connectivity/mod.rs"]
mod dynamic_connectivity;

#[deriving(PartialEq)]
enum State {
    Open,
    Full,
    Blocked
}

impl Show for State {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            Open => "□".fmt(f),
            Full => "■".fmt(f),
            Blocked => "x".fmt(f),
        }
    }
}

pub struct Percolation {
    n: uint,
    states: Vec<State>,
    pub positions: QuickFind
}

impl Percolation {
    pub fn new(n: uint) -> Percolation {
        Percolation{n: n, states: Vec::from_fn(n * n, |_| Blocked), positions: QuickFind::new(n * n)}
    }

    fn to_index(&self, i: uint, j: uint) -> uint {
        i * self.n + j
    }

    fn index(&self, point: (uint, uint)) -> uint {
        self.to_index(point.val0(), point.val1())
    }

    fn is_upper_left_corner(&self, i: uint, j: uint) -> bool {
        i == 0 && j == 0
    }

    fn is_bottom_left_corner(&self, i: uint, j: uint) -> bool {
        i == (self.n - 1) && j == 0
    }


    fn is_upper_right_corner(&self, i: uint, j: uint) -> bool {
        i == 0 && j == (self.n - 1)
    }

    fn is_bottom_right_corner(&self, i: uint, j: uint) -> bool {
        i == self.n - 1 && j == (self.n - 1)
    }

    fn up(&self, i: uint, j: uint) -> (uint, uint) {
        (i - 1, j)
    }

    fn bottom(&self, i: uint, j: uint) -> (uint, uint) {
        (i + 1, j)
    }

    fn left(&self, i: uint, j: uint) -> (uint, uint) {
        (i, j - 1)
    }

    fn right(&self, i: uint, j: uint) -> (uint, uint) {
        (i, j + 1)
    }

    fn has_left(&self, i: uint, j: uint) -> bool {
        j > 0
    }

    fn has_right(&self, i: uint, j: uint) -> bool {
        j < self.n - 1
    }

    fn has_up(&self, i: uint, j: uint) -> bool {
        i > 0
    }

    fn has_bottom(&self, i: uint, j: uint) -> bool {
        i < self.n - 1
    }


    pub fn open(&mut self, i: uint, j: uint) {
        let index = self.to_index(i, j);
        let state = if index >= 0 && index < self.n {
            Full
        } else if self.has_right(i, j) && self.states[self.index(self.right(i, j))] == Full {
            Full
        } else if self.has_left(i, j) && self.states[self.index(self.left(i, j))] == Full {
            Full
        } else if self.has_up(i, j) && self.states[self.index(self.up(i, j))] == Full {
            Full
        } else if self.has_bottom(i, j) && self.states[self.index(self.bottom(i, j))] == Full {
            Full
        } else {
            Open
        };
        *self.states.get_mut(index) = state;
    }

    pub fn is_open(&self, i: uint, j: uint) -> bool {
        match self.states[self.to_index(i, j)] {
            Open => true,
            _    => false
        }
    }

    pub fn is_full(&self, i: uint, j: uint) -> bool {
        match self.states[self.to_index(i, j)] {
            Full => true,
            _    => false
        }
    }

    pub fn percolates(&self) -> bool {
        let lower_row_start = (self.n * self.n) - self.n;
        let lower_row_end = self.n * self.n;
        for i in range(0, self.n) {
            for j in range(lower_row_start, lower_row_end) {
                if self.positions.connected(i, j) {
                    return true;
                }
            }
        }
        false
    }

    pub fn print_grid(&self) {
        for i in range(1, self.states.len() + 1) {
            print!("{}  ", self.states[i - 1]);
            if i % self.n == 0 { println!(""); }
        }
    }
}
