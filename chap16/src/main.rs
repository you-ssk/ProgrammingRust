use std::ops::Index;

fn main() {
    ex_16_2();
}

use std::collections::HashSet;

fn ex_16_2() {
    let mut numbers: Vec<i32> = vec![];
    let mut words = vec!["step", "on", "no", "pets"];
    let mut buffer = vec![0u8; 1024];
    let w = &words;
    println!("{:?}, {:?}, {:?}", w.first(), w.last(), w.get(2));
    let f = words.first_mut().unwrap();
    *f = "steps";
    println!("{:?}", words);
    //let mut v = w.to_vec();
    //v[1] = "oh";
    //println!("{:?}, {:?}", words, v);

    let mut byte_vec = b"Misssssssssissipppii".to_vec();
    byte_vec.dedup();
    println!("{:?}", String::from_utf8(byte_vec).unwrap());

    {
        let mut byte_vec = b"Missssssisssipppiiii".to_vec();
        let mut seen = HashSet::new();
        byte_vec.retain(|r| seen.insert(*r));
        println!("{:?}", String::from_utf8(byte_vec).unwrap());
    }
}
