use std::vec::Vec;
use self::percolation::Percolation;

mod percolation;

fn main() {
    let mut p = Percolation::new(5);
    p.open(0, 2);
    p.open(1, 2);
    p.open(2, 2);
    p.open(2, 3);
    p.open(3, 3);
    p.open(4, 3);
    p.print_grid();
    println!("percolates? {}", p.percolates());
}
