fn main() {
    fn triangle(n: i32) -> i32 {
        let mut sum = 0;
        for i in 1..=n {
            sum += i;
        }
        sum
    }
    println!("{}", triangle(10));

    fn triangle2(n: i32) -> i32 {
        (1..=n).fold(0, |sum, item| sum + item)
    }
    println!("{}", triangle2(10));

    ex_15_1();
    ex_15_2();
    ex_15_3();
}

fn ex_15_1() {
    println!("There is :");
    let v = vec!["antimony", "arsenic", "aluminum", "selenium"];
    for element in &v {
        println!("{}", element);
    }

    println!("There is :");
    let mut iterator = (&v).into_iter();
    while let Some(element) = iterator.next() {
        println!("{}", element);
    }
}

fn ex_15_2() {
    use std::path::Path;
    let path = Path::new("C:/Users/JimB/Downloades/Fedora.iso");
    println!("{:?}", path);
    let mut iterator = path.iter();
    println!("{:?}", iterator.next());
    println!("{:?}", iterator.next());
    println!("{:?}", iterator.next());
    println!("{:?}", iterator.next());
    println!("{:?}", iterator.next());

    ex_15_2_2();
    ex_15_2_3();
    ex_15_2_4();
}

fn ex_15_2_2() {
    use std::collections::BTreeSet;
    let mut favorites = BTreeSet::new();
    favorites.insert("Lucy in the Sky With Diamonds".to_string());
    favorites.insert("Liebesträume No. 3".to_string());

    let mut it = favorites.into_iter();
    println!("{:?}", it.next());
    println!("{:?}", it.next());
    println!("{:?}", it.next());
}

fn ex_15_2_3() {
    use rand::random;
    use std::iter::from_fn;

    let length: Vec<f64> = from_fn(|| Some((random::<f64>() - random::<f64>()).abs()))
        .take(100)
        .collect();
    println!("{:?}", length);

    use num::Complex;
    use std::iter::successors;

    fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
        let zero = Complex { re: 0.0, im: 0.0 };
        successors(Some(zero), |&z| Some(z * z + c))
            .take(limit)
            .enumerate()
            .find(|(_i, z)| z.norm_sqr() > 4.0)
            .map(|(i, _z)| i)
    }
    let c = Complex { re: -1.0, im: 0.2 };
    let size = escape_time(c, 20);
    println!("c : {}, size : {:?}", c, size);

    fn fibonacci() -> impl Iterator<Item = usize> {
        let mut state = (0, 1);
        std::iter::from_fn(move || {
            state = (state.1, state.0 + state.1);
            Some(state.0)
        })
    }
    println!("{:?}", fibonacci().take(8).collect::<Vec<_>>());
}

fn ex_15_2_4() {
    let mut outer = "Earth".to_string();
    {
        let inner = String::from_iter(outer.drain(1..4));
        println!("{}", outer);
        println!("{}", inner);
    }
    println!("{}", outer);
}

fn ex_15_3() {
    ex_15_3_1();
}

fn ex_15_3_1() {
    let text = " ponies   \n   giraffes\niguanas   \nsquid".to_string();
    {
        let v: Vec<&str> = text.lines().map(str::trim).collect();
        println!("{:?}", v);
    }
    {
        let v: Vec<&str> = text
            .lines()
            .map(str::trim)
            .filter(|s| *s != "iguanas")
            .collect();
        println!("{:?}", v);
    }
    // {
    //     ["earth", "water", "air", "fire"].iter().map(|elt| println!("{}", elt));
    // }
}
