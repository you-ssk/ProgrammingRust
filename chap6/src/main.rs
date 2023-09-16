use core::num;
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
    ex_6_7();
    ex_6_8();
    ex_6_9();
    ex_6_11();
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

fn ex_6_7() {
    let mut num = 0;
    while num < 10 {
        println!("num = {}", num);
        num = num + 1;
    }

    for i in num..num * 2 {
        println!("i = {}", i)
    }

    let l = loop {
        num = num + 1;
        if num % 9 == 0 {
            break num;
        }
    };
    println!("after loop : num = {}, l = {:?}", num, l);

    // let file = fs::File::open("Cargo.toml").expect("error open file");
    // let buffered = io::BufReader::new(file);
}

fn ex_6_8() {
    // labeled loop
    'loop_i: for i in 1..10 {
        'loop_j: for j in 1..10 {
            println!("{}", i * j);
            if i * j == 40 {
                //break 'loop_i
                break 'loop_j;
                //break
            }
            if i * j == 72 {
                break 'loop_i;
            }
        }
    }
}

use std::fs::File;
fn ret() -> Result<(), io::Error> {
    let filename = "aaa.txt"; // Ok
                              // let filename = "\\.txt"; // Err
    let output = match File::create(filename) {
        Ok(f) => f,
        Err(err) => return Err(err),
    };
    Ok(())
}

fn ex_6_9() {
    let r = match ret() {
        Ok(r) => println!("succ."),
        Err(err) => println!("{}", err),
    };
}

use rand::prelude::*;

fn ex_6_11() {
    let mut numbers = Vec::new();
    numbers.push(1);
    println!("{:?}", numbers);

    // let n = 10;
    // let ramp = (0..n).collect::<Vec<i32>>();
    let mut rng = thread_rng();
    let mut vals = (0..10).collect::<Vec<i32>>(); // ターボフィッシュ
    let mut vals2: Vec<i32> = (0..10).collect(); //　ほかの書き方

    vals.shuffle(&mut rng);
    println!("{:?}", vals);
}
