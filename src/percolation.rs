use std::vec::Vec;
use std::fmt::{Show, Formatter, Result};

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
    list: Vec<State>
}

impl Percolation {
    pub fn new(n: uint) -> Percolation {
        Percolation{n: n, list: Vec::from_fn(n * n, |_| Blocked)}
    }

    pub fn print_grid(&self) {
        for i in range(1, self.list.len() + 1) {
            print!("{}  ", self.list[i - 1]);
            if i % self.n == 0 { println!(""); }
        }
    }
}
