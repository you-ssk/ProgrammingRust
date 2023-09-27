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
}
