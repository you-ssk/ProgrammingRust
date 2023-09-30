use std::mem;
use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::mem::swap as memswap;

fn main() {
    println!("Hello, world!");
    let mut x = 5;
    let mut y = 42;
    println!("{} : {}", x, y);
    mem::swap(&mut x, &mut y);
    println!("{} : {}", x, y);
    memswap(&mut x, &mut y);
    println!("{} : {}", x, y);

    let mut fern = Fern {
        size: 1.0,
        growth_rate: 0.001
    };
    run_simulate(&mut fern, 1000);
    println!("final fern size: {}", fern.size);
}

pub struct Fern {
    pub size: f64,
    pub growth_rate: f64
}

impl Fern {
    fn grow(&mut self) {
        self.size *= 1.0 + self.growth_rate;
    }
}

pub fn run_simulate(fern: &mut Fern, days: usize) {
    for _ in 0..days {
        fern.grow();
    }
}
