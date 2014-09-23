use std::vec::Vec;
use self::percolation::Percolation;

mod percolation;

fn main() {
    let mut p = Percolation::new(3);
    p.open(0, 1);
    p.open(1, 1);
    p.open(2, 2);
    p.open(2, 1);
    p.print_grid();
    println!("percolates? {}", p.percolates());
}
