use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::mem;
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
        growth_rate: 0.001,
    };
    run_simulate(&mut fern, 1000);
    println!("final fern size: {}", fern.size);
}

pub struct Fern {
    pub size: f64,
    pub growth_rate: f64,
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

#[test]
fn math_works() {
    let x: i32 = 1;
    assert!(x.is_positive());
    assert_eq!(x + 1, 2);
}

#[test]
#[allow(unconditional_panic, unused_must_use)]
#[should_panic(expected = "divide by zero")]
fn test_divide_by_zero_error() {
    1 / 0;
}

use std::num::ParseIntError;

#[test]
fn explicit_radix() -> Result<(), ParseIntError> {
    i32::from_str_radix("1024", 10)?;
    Ok(())
}

#[cfg(test)]
mod test {
    fn roughly_equal(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-6
    }

    #[test]
    fn trig_works() {
        use std::f64::consts::PI;
        assert!(roughly_equal(PI.sin(), 0.0));
    }
}
