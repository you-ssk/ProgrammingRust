use http::StatusCode;
use std::fs;
use std::io;
use std::io::BufRead;

fn main() {
    println!("Hello, world!");
    ex_6_1();
    ex_6_4();
}

fn ex_6_1() {
    let a = 20;
    let status = if a <= 10 {
        http::StatusCode::OK
    } else {
        http::StatusCode::INTERNAL_SERVER_ERROR
    };
    println!("{} {}", a, status);
}

fn ex_6_4() -> io::Result<()> {
    // let cont = fs::read_to_string("Cargo.toml").unwrap();
    // for l in cont.lines(){
    //     println!("{}", l);
    // }

    let file = fs::File::open("Cargo.toml")?;
    let buffered = io::BufReader::new(file);
    for line in buffered.lines() {
        let line = line?;
        println!("{}", line)
    }
    Ok(())
}
