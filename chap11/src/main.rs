use std::io::Write;

fn main() {
    println!("Hello, world!");
    ex_11_0();
    ex_11_1();
    ex_11_1_2();
}

fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}

fn ex_11_0() -> std::io::Result<()> {
    use std::fs::File;
    let mut local_file = File::create("hello.txt")?;
    say_hello(&mut local_file)?;

    let mut bytes = vec![];
    say_hello(&mut bytes)?;
    assert_eq!(bytes, b"hello world\n");
    Ok(())
}

fn ex_11_1() {
    let mut buf: Vec<u8> = vec![];
    buf.write_all(b"hello").unwrap();
    println!("{:?}", buf);

    //11.1.1
    let mut buf: Vec<u8> = vec![];
    // let writer: dyn Write = buf;
    let writer: &mut dyn Write = &mut buf;

    use std::fs::File;
    let mut local_file = File::create("hello.txt").unwrap();
    let w: Box<dyn Write> = Box::new(local_file);
}

fn say_hello_generic<W: Write>(out: &mut W) -> std::io::Result<()> {
    out.write_all(b"hello world(generic)")?;
    out.flush()
}

use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

fn top_ten<T: Debug + Hash + Eq>(values: &Vec<T>) {
    let mut h: HashMap<&T, i32> = HashMap::new();
    for v in values {
        h.entry(v).and_modify(|e| *e += 1).or_insert(1);
    }

    let mut v: Vec<(&&T, &i32)> = h.iter().collect();
    v.sort_by(|a, b| (-a.1).cmp((&(-b.1))));
    println!("{:?}", v);
}

fn dot_product<const N: usize>(a: [f64; N], b: [f64; N]) -> f64 {
    let mut sum = 0.;
    for i in 0..N {
        sum += a[i] * b[i];
    }
    sum
}

fn ex_11_1_2() -> std::io::Result<()> {
    use std::fs::File;
    let mut local_file = File::create("hello.txt")?;
    say_hello_generic(&mut local_file)?;

    // let v1 = (0..1000).collect();  //error[E0282]: type annotations needed
    let v2 = (0..100).collect::<Vec<i32>>();
    // println!("{:?}", v2);

    {
        use rand::{distributions::Uniform, Rng}; // 0.6.5

        let mut rng = rand::thread_rng();
        let range = Uniform::new(0, 20);

        let vals: Vec<u64> = (0..100).map(|_| rng.sample(&range)).collect();
        println!("{:?}", vals);
        top_ten(&vals);
    }
    {
        let dot = dot_product([1., 0.], [0., 1.]);
        println!("dot product = {}", dot);

        let dot = dot_product([0.5, 0.5], [-0.5, 0.5]);
        println!("dot product = {}", dot);
    }

    Ok(())
}
