use std::io::{self, Write};

fn main() {
    println!("Hello, world!");
    ex_11_0();
    ex_11_1();
    ex_11_1_2();
    ex_11_2_2();
    ex_11_3();
    ex_11_4_1();
    ex_11_4_2();
    ex_11_4_3();
    ex_11_4_4();
    ex_11_5();
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
use std::ops::Mul;

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

    {
        let mut sink = std::io::sink();
        say_hello(&mut sink)?;
    }

    Ok(())
}

fn ex_11_2_2() {
    trait IsEmoji {
        fn is_emoji(&self) -> bool;
    }
    impl IsEmoji for char {
        fn is_emoji(&self) -> bool {
            true
        }
    }

    struct HtmlDocument {}
    trait WriteHtml {
        fn write_html(&mut self, html: &HtmlDocument) -> io::Result<()>;
    }

    impl<W: Write> WriteHtml for W {
        fn write_html(&mut self, _html: &HtmlDocument) -> io::Result<()> {
            Ok(())
        }
    }

    use serde::Serialize;
    use serde_json;
    use std::fs::File;

    pub fn save_configuration(config: &HashMap<String, String>) -> std::io::Result<()> {
        let writer = File::create("test_config.json")?;
        let mut serializer = serde_json::Serializer::new(writer);
        config.serialize(&mut serializer)?;
        Ok(())
    }

    let mut h: HashMap<String, String> = HashMap::new();
    h.insert("A".to_string(), "aaaaaa".to_string());
    let _ = save_configuration(&h);
}

fn ex_11_2_3() {
    trait Spliceable {
        fn splice(&self, other: &Self) -> Self;
    }

    struct Example {}
    impl Spliceable for Example {
        fn splice(&self, other: &Self) -> Self {
            let r = Example {};
            r
        }
    }
}

fn ex_11_3() {
    let s = "hello".to_string();
    let s2 = str::to_string("hello");
    ToString::to_string("hello");
    <str as ToString>::to_string("hello"); // fully qualified

    let zero = 0;
    //println!("{}",zero.abs()); //error[E0689]: can't call method `abs` on ambiguous numeric type `{integer}`
    println!("{}", i64::abs(zero));
}

fn ex_11_4_1() {
    fn collect_into_vector<I: Iterator>(iter: I) -> Vec<I::Item> {
        let mut result = Vec::new();
        for value in iter {
            result.push(value);
        }
        result
    }

    fn dump<I>(iter: I)
    where
        I: Iterator,
        I::Item: Debug,
    {
        for (index, value) in iter.enumerate() {
            println!("{}: {:?}", index, value);
        }
    }
}

fn ex_11_4_2() {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let n1: u8 = rng.gen();
    let n2: u8 = rng.gen();
    //let n3 = n1.saturating_mul(n2);
    //let n3 = n1.mul(n2);
    //println!("{} * {} = {:?}", n1, n2, n3);
}

fn ex_11_4_3() {
    use std::iter;
    use std::vec::IntoIter;

    fn cyclical_zip(
        v: Vec<u8>,
        u: Vec<u8>,
    ) -> iter::Cycle<iter::Chain<IntoIter<u8>, IntoIter<u8>>> {
        v.into_iter().chain(u.into_iter()).cycle()
    }

    fn cyclical_zip2(v: Vec<u8>, u: Vec<u8>) -> Box<dyn Iterator<Item = u8>> {
        Box::new(v.into_iter().chain(u.into_iter()).cycle())
    }

    fn cyclical_zip3(v: Vec<u8>, u: Vec<u8>) -> impl Iterator<Item = u8> {
        v.into_iter().chain(u.into_iter()).cycle()
    }
    let v: Vec<u8> = vec![1, 2, 3, 4, 5];
    let u: Vec<u8> = vec![101, 102, 103, 104, 105];
    let w = cyclical_zip3(v, u);
    for e in w.take(20) {
        println!("{}", e);
    }
}

fn ex_11_4_4() {
    trait Greet {
        const GREETING: &'static str = "Hello";
        fn greet(&self) -> String;
    }

    struct Example {}

    impl Greet for Example {
        fn greet(&self) -> String {
            Self::GREETING.to_string()
        }
    }

    let e = Example {};
    println!("Example {}", e.greet());

    trait Float {
        const ZERO: Self;
        const ONE: Self;
    }

    impl Float for f32 {
        const ZERO: f32 = 0.0;
        const ONE: f32 = 1.0;
    }
    impl Float for f64 {
        const ZERO: f64 = 0.0;
        const ONE: f64 = 2.0;
    }

    use std::ops::Add;
    fn add_one<T: Float + Add<Output = T>>(value: T) -> T {
        value + T::ONE
    }
    {
        let n: f32 = 100.0;
        let m = add_one(n);
        println!("{} {}", n, m);
    }
    {
        let n: f64 = 100.0;
        let m = add_one(n);
        println!("{} {}", n, m);
    }

    fn fib<T: Float + Add<Output = T>>(n: usize) -> T {
        match n {
            0 => T::ZERO,
            1 => T::ONE,
            n => fib::<T>(n - 1) + fib::<T>(n - 2),
        }
    }
    {
        for i in 1usize..10 {
            println!("{}", fib::<f32>(i));
        }
    }
}

fn ex_11_5() {
    // fn dot(v1: &[i64], v2: &[i64]) -> i64 {
    //     let mut total = 0;
    //     for i  in 0..v1.len() {
    //         total = total + v1[i] * v2[i];
    //     }
    //     total
    // }

    use std::ops::{Add, Mul};
    //fn dot<N: Add<Output = N> + Mul<Output = N> + Default + Copy>(v1: &[N], v2: &[N]) -> N {
    fn dot<N>(v1: &[N], v2: &[N]) -> N
    where
        N: Add<Output = N> + Mul<Output = N> + Default + Copy,
    {
        let mut total: N = N::default();
        for i in 0..v1.len() {
            total = total + v1[i] * v2[i];
        }
        total
    }
    let v1: [i64; 4] = [1, 2, 3, 4];
    let v2: [i64; 4] = [1, 1, 1, 1];
    let d = dot(&v1, &v2);
    println!("dot = {}", d);
}
