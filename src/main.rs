use std::vec::Vec;
use self::percolation::Percolation;

mod percolation;

fn main() {
    let mut p = Percolation::new(7);
    p.open(0, 3);
    p.open(1, 3);
    p.open(2, 3);
    p.open(4, 3);
    p.open(5, 3);
    p.open(6, 3);
    p.print_grid();
    println!("percolates? {}", p.percolates());
}
