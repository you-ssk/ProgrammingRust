fn main() {
    ex_7_1();
}

fn pirate_share(total: u64, crew_size: usize) -> u64 {
    let half = total/2;
    half /crew_size as u64
}

fn ex_7_1() {
    let total = 1200;
    let crew_size = 0;
    let share = pirate_share(total, crew_size);
    println!("share = {}", share);
}
