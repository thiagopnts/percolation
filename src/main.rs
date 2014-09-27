use std::vec::Vec;
use std::rand::random;
use self::percolation_stats::PercolationStats;

mod percolation_stats;

fn main() {
    let mut stats = PercolationStats::new(2, 100000);
    stats.run_simulation();
//    p.open(1, 0);
//    p.open(2, 0);
//    p.open(2, 1);
//    p.open(2, 2);
//    p.open(3, 2);
//    p.open(3, 3);
//    p.open(4, 2);
//    p.open(4, 3);
//    p.open(4, 1);
//    p.open(5, 1);
//    p.open(6, 0);
//    p.open(6, 2);
//    p.open(7, 0);
//    p.open(7, 1);
//    p.open(7, 2);
//    p.open(7, 3);
//    p.open(7, 5);
//    p.open(6, 4);
//    p.open(6, 5);
//    p.open(6, 6);
//    p.open(6, 7);
//    p.open(5, 6);
//    p.open(5, 7);
//    p.open(4, 5);
//    p.open(4, 6);
//    p.open(3, 5);
//    p.open(3, 6);
//    p.open(2, 5);
//    p.open(2, 6);
//    p.open(1, 5);
//    p.open(1, 6);
//    p.open(1, 3);
//    p.open(1, 4);
//    p.open(0, 2);
//    p.open(0, 3);
//    p.open(0, 4);
//    p.print_grid();
//    println!("percolates? {}", p.percolates());
}
