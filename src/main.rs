
mod percolation;

fn main() {
    let p = percolation::Percolation::new(50);
    p.print_grid();
}
