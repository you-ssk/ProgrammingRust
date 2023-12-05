use num::Complex;

fn main() {
    println!("Hello, world!");
    let c = Complex { re: 1.0, im: -0.0625 };
    println!("{}", c);
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..10 {
        z = z * z + c;
        println!("{}", z);
    }
}
