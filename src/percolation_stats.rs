use self::percolation::Percolation;
use std::rand::random;
use std::num::Float;

#[path = "./percolation.rs"]
mod percolation;

pub struct PercolationStats {
    n: uint,
    t: uint,
    mean: f64,
    stddev: f64,
    confidence_hi: f64,
    confidence_lo: f64,
    thresholds: Vec<f64>
}

impl PercolationStats {
    pub fn new(n: uint, t: uint) -> PercolationStats {
        PercolationStats { n: n, t: t, mean: 0f64, stddev: 0f64, confidence_lo: 0f64, confidence_hi: 0f64, thresholds: vec!() }
    }

    pub fn run_simulation(&mut self) {
        for i in range(0, self.t) {
            let t = self.calculate_threshold();
            self.thresholds.push(t);
        }
        self.mean = self.mean();
        self.stddev = self.stddev();
        self.confidence_lo = self.confidence_lo();
        self.confidence_hi = self.confidence_hi();
        println!("mean\t\t\t = {}", self.mean);
        println!("stddev\t\t\t = {}", self.stddev);
        println!("95% confidence interval\t = {}, {}\n", self.confidence_lo, self.confidence_hi);
    }

    pub fn calculate_threshold(&mut self) -> f64 {
        let mut percolation = Percolation::new(self.n);
        let mut opened = 0f64;
        loop {
            let (i, j) = self.random_tuple();
            if !percolation.is_open(i, j) {
                percolation.open(i, j);
                opened += 1f64;
            }
            if percolation.percolates() {
                return (opened / (self.n * self.n) as f64);
            }
        }
    }

    pub fn mean(&mut self) -> f64 {
        let mut x = 0f64;
        for i in range(0, self.thresholds.len()) {
            x += self.thresholds[i];
        }
        x / self.t as f64
    }

    pub fn stddev(&mut self) -> f64 {
        let mut sum = 0f64;
        // fold it pls
        for i in range(0, self.thresholds.len()) {
            sum += (self.thresholds[i] - self.mean) * (self.thresholds[i] - self.mean);
        }
        (sum / (self.t as f64 - 1f64)).sqrt()
    }

    pub fn confidence_lo(&mut self) -> f64 {
        self.mean - (1.96 * self.stddev) / (self.t as f64).sqrt()
    }

    pub fn confidence_hi(&mut self) -> f64 {
        self.mean + (1.96 * self.stddev) / (self.t as f64).sqrt()
    }

    fn random_tuple(&self) -> (uint, uint) {
        let i = random::<uint>() % self.n;
        let j = random::<uint>() % self.n;
        (i, j)
    }
}
