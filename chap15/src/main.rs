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
    use std::ffi::OsStr;
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
