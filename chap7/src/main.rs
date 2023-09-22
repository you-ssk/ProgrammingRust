fn main() {
    ex_7_1();
    ex_7_2_1();
    ex_7_2_2();
}

fn pirate_share(total: u64, crew_size: usize) -> u64 {
    let half = total / 2;
    half / crew_size as u64
}

fn ex_7_1() {
    let total = 1200;
    let crew_size = 1;
    let share = pirate_share(total, crew_size);
    println!("share = {}", share);
}

use std::io;

fn foo() -> io::Result<()> {
    let a = 100;
    let b = 200;
    if a > b {
        let r = io::Error::new(io::ErrorKind::Other, "time out");
        return Err(r);
    } else {
        return Ok(());
    };
    //let r = io::Error::new(io::ErrorKind::Other, "time out");
    //return r;
    //Ok(())
}

fn ex_7_2_1() {
    let r = io::Error::new(io::ErrorKind::Other, "time out");
    println!("{:?}", r);
    println!("{:?}", foo());
    match foo() {
        Ok(r) => {
            println!("Ok.{:?}", r);
        }
        Err(err) => {
            println!("error = {:?}", err);
        }
    }
    let ret = foo();
    println!("{}", ret.is_ok());
    println!("{}", ret.is_err());
    let ret2 = ret.as_ref();
    println!("ret.ok = {:?}, ret.err = {:?}", ret2.ok(), ret2.err());
}

fn ex_7_2_2() {
    let r = std::fs::remove_file("a.txt");
    println!("{:?}", r)
}
