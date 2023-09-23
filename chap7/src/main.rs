
use std::io;
use std::fs;
use std::io::BufRead;


fn main() {
    ex_7_1();
    ex_7_2_1();
    ex_7_2_2();
    ex_7_2_3();
    ex_7_2_4();
    ex_7_2_5();
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

fn ex_7_2_3() {
    let r = std::fs::remove_file("a.txt");
    println!("{}", r.as_ref().err().unwrap());
    //println!("{}", r.as_ref().ok().unwrap());
}

fn bar() -> Result<u32, io::Error> {
    let r = std::fs::remove_file("a.txt")?;
    Ok(32)
}

fn ex_7_2_4() {
    let r = bar();
    println!("{:?}", r);
}

type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
type GenericResult<T> = Result<T, GenericError>;

fn read_numbers(file: &mut dyn BufRead) -> GenericResult<Vec<i64>> {
    let mut numbers = vec![];
    for line_result in file.lines() {
        let line = line_result?;
        numbers.push(line.parse()?);
    }
    Ok(numbers)
}

fn ex_7_2_5() {
    //let mut file = fs::File::open("Cargo.toml").unwrap();
    let mut file = fs::File::open("ex7_2_5.txt").unwrap();
    let mut bufread = io::BufReader::new(file);
    let r = read_numbers(&mut bufread);
    println!("{:?}", r);
}
