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

enum Direction {
    Left,
    Right,
    Up,
    Bottom,
    Current
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
    pub positions: WeightedQuickUnion
}

impl Percolation {
    pub fn new(n: uint) -> Percolation {
        Percolation{n: n, states: Vec::from_fn(n * n, |_| Blocked), positions: WeightedQuickUnion::new(n * n)}
    }

    fn to_index(&self, i: uint, j: uint) -> uint {
        i * self.n + j
    }

    fn index(&self, point: (uint, uint)) -> uint {
        self.to_index(point.val0(), point.val1())
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

    fn right_state(&self, i: uint, j: uint) -> State {
        self.states[self.index(self.right(i, j))]
    }

    fn left_state(&self, i: uint, j: uint) -> State {
        self.states[self.index(self.left(i, j))]
    }

    fn bottom_state(&self, i: uint, j: uint) -> State {
        self.states[self.index(self.bottom(i, j))]
    }

    fn up_state(&self, i: uint, j: uint) -> State {
        self.states[self.index(self.up(i, j))]
    }

    fn is_blocked(&self, i: uint, j: uint, direction: Direction) -> bool {
        match direction {
            Right => self.has_right(i, j) && self.right_state(i, j) == Blocked,
            Left  => self.has_left(i, j) && self.left_state(i, j) == Blocked,
            Up    => self.has_up(i, j) && self.up_state(i, j) == Blocked,
            Bottom => self.has_bottom(i, j) && self.bottom_state(i, j) == Blocked,
            Current => self.states[self.to_index(i, j)] == Blocked
        }
    }

    fn fill_fast(&mut self, point: (uint, uint)) {
        let (i, j) = point;
        let q = self.to_index(i, j);
        if self.is_open_in_direction(i, j, Up) {
            let p = self.index(self.up(i, j));
            self.positions.union(p, q);
        }
        if self.is_open_in_direction(i, j, Left) {
            let p = self.index(self.left(i, j));
            self.positions.union(p, q);
        }
        if self.is_open_in_direction(i, j, Right) {
            let p = self.index(self.right(i, j));
            self.positions.union(p, q);
        }
        if self.is_open_in_direction(i, j, Bottom) {
            let p = self.index(self.bottom(i, j));
            self.positions.union(p, q);
        }
    }

    fn fill(&mut self, point: (uint, uint)) {
        let (i, j) = point;
        let p = self.to_index(i, j);

        if p >= 0 && p < self.n {
            *self.states.get_mut(p) = Full;
        } else if self.is_open_in_direction(i, j, Current) {
            if self.is_full(i, j, Left) {
                let q = self.index(self.left(i, j));
                *self.states.get_mut(p) = Full;
                self.positions.union(p, q)
            } else if self.is_full(i, j, Right) {
                let q = self.index(self.right(i, j));
                *self.states.get_mut(p) = Full;
                self.positions.union(p, q)
            } else if self.is_full(i, j, Up) {
                let q = self.index(self.up(i, j));
                *self.states.get_mut(p) = Full;
                self.positions.union(p, q)
            } else if self.is_full(i, j, Bottom) {
                let q = self.index(self.bottom(i, j));
                *self.states.get_mut(p) = Full;
                self.positions.union(p, q)
            } else {
                return;
            }
        } else {
            return;
        }
        
        let right_point = self.right(i, j);
        let left_point = self.left(i, j);
        let up_point = self.up(i, j);
        let bottom_point = self.bottom(i, j);
        let q = self.to_index(i, j);
        if self.is_open_in_direction(i, j, Right) {
            self.fill(right_point);
        }
        if self.is_open_in_direction(i, j, Left) {
            self.fill(left_point);
        }
        if self.is_open_in_direction(i, j, Up) {
            self.fill(up_point);
        }
        if self.is_open_in_direction(i, j, Bottom) {
            self.fill(bottom_point);
        }
    }

    pub fn open(&mut self, i: uint, j: uint) {
        let index = self.to_index(i, j);
        *self.states.get_mut(index) = Open;

        self.fill((i, j));
    }

    pub fn open_fast(&mut self, i: uint, j: uint) {
        let index = self.to_index(i, j);
        *self.states.get_mut(index) = Open;

        self.fill_fast((i, j));
    }

    pub fn is_open(&self, i: uint, j: uint) -> bool {
        self.is_open_in_direction(i, j, Current)
    }

    fn is_open_in_direction(&self, i: uint, j: uint, direction: Direction) -> bool {
        match direction {
            Right => self.has_right(i, j) && self.right_state(i, j) == Open,
            Left  => self.has_left(i, j) && self.left_state(i, j) == Open,
            Up    => self.has_up(i, j) && self.up_state(i, j) == Open,
            Bottom => self.has_bottom(i, j) && self.bottom_state(i, j) == Open,
            Current => self.states[self.to_index(i, j)] == Open
        }
    }

    pub fn is_full(&self, i: uint, j: uint, direction: Direction) -> bool {
        match direction {
            Right => self.has_right(i, j) && self.right_state(i, j) == Full,
            Left  => self.has_left(i, j) && self.left_state(i, j) == Full,
            Up    => self.has_up(i, j) && self.up_state(i, j) == Full,
            Bottom => self.has_bottom(i, j) && self.bottom_state(i, j) == Full,
            Current => self.states[self.to_index(i, j)] == Full
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
