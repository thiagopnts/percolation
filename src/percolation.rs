use std::vec::Vec;
use std::fmt::{Show, Formatter, Result};
use self::dynamic_connectivity::{QuickFind, QuickUnion, WeightedQuickUnion};

//how do I remove this?
#[path = "dynamic_connectivity/mod.rs"]
mod dynamic_connectivity;

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

    pub fn open(&mut self, i: uint, j: uint) {
        let index = self.to_index(i, j);
        *self.states.get_mut(index) = Open;
        self.positions.union(i, index);
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
