use http::StatusCode;
use std::fs;
use std::io;
use std::io::BufRead;

fn main() {
    println!("Hello, world!");
    ex_6_1();
    ex_6_4().expect("error 6.4");
    ex_6_5();
    ex_6_6();
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

fn ex_6_5() {
    let retcode = 10;
    match retcode {
        0 => println!("OK"),
        1 => println!("Wires Tabgled"),
        2 => println!("User Asleep"),
        _ => println!("Unrecognized Error {}", retcode),
    }

    //let params: Option<String> = Some("taro".to_string());
    let params: Option<String> = None;
    match params {
        Some(params) => println!("Hello, {}!", params),
        None => println!("Greetings, stranger"),
    }
}

fn ex_6_6() {
    let number = Some(7);
    //let number: Option<i32> = None;
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    if let Some(i) = number {
        println!("Matched {:?}!", i)
    } else {
        println!("Didn't match a number. Let's go with a letter")
    }

    if let Some(i) = letter {
        println!("Matched {:?}!", i)
    } else {
        println!("Didn't match a number. Let's go with a letter")
    }
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}", i);
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter");
    } else {
        println!("I don't like letters. Let's go with am emoticon')!");
    }
}
