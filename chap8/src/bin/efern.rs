use chap8::{Fern, run_simulate};

fn main() {
    let mut fern = Fern {
        size: 1.0,
        growth_rate: 0.002
    };
    run_simulate(&mut fern, 1000);
    println!("final fern size: {}", fern.size);
}