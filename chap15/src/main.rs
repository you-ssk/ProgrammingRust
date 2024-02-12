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
    ex_15_3_2();
    ex_15_3_3();
    ex_15_3_4();
    ex_15_3_5();
    ex_15_3_6();
    ex_15_3_7();
    ex_15_3_8();
    ex_15_3_9();
    ex_15_3_10();
    ex_15_3_12();
    ex_15_3_13();
    ex_15_3_14();
    ex_15_3_15();
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

fn ex_15_3_2() {
    use std::str::FromStr;

    let text = "1\nfrond .25 289\n3.1415 estuary\n";
    for number in text
        .split_whitespace()
        .filter_map(|w| f64::from_str(w).ok())
    {
        println!("{:4.2}", number.sqrt());
    }

    let nums: Vec<f64> = text
        .split_whitespace()
        .map(|w| f64::from_str(w))
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap().sqrt())
        .collect();
    println!("{:?}", nums);

    use std::collections::HashMap;

    let mut major_cities = HashMap::new();
    major_cities.insert("Japan", vec!["Tokyo", "Kyoto"]);
    major_cities.insert("The United States", vec!["Portland", "Nashville"]);
    major_cities.insert("Brazil", vec!["Sao Paulo", "Brasilia"]);
    major_cities.insert("Kenya", vec!["Nairobi", "Mombasa"]);
    major_cities.insert("The Netherlands", vec!["Amsterdam", "Utrecht"]);
    let countries = ["Japan", "Brazil", "Kenya"];

    for &city in countries.iter().flat_map(|country| &major_cities[country]) {
        println!("{}", city);
    }
}

fn ex_15_3_3() {
    use std::collections::BTreeMap;

    let mut parks = BTreeMap::new();
    parks.insert("Portland", vec!["Mt. Toabor Park", "Forest Park"]);
    parks.insert("Kyoto", vec!["Tadasu-no-Mori Forest", "Maruyama Koen"]);
    parks.insert("Nashville", vec!["Percy Warner Park", "Dragon Park"]);
    let all_parks: Vec<_> = parks.values().flatten().cloned().collect();
    println!("{:?}", all_parks);
}

fn ex_15_3_4() {
    let message = "To: jimb\r\n\
    From: supergo <editor@oreilly.com>\r\n\
    \r\n\
    Did you get any writeing done today?\r\n\
    When will you stop wasting time plotting fractals?\r\n";
    for header in message.lines().take_while(|l| !l.is_empty()) {
        println!("{}", header);
    }
}

fn ex_15_3_5() {
    let message = "To: jimb\r\n\
    From: supergo <editor@oreilly.com>\r\n\
    Did you get any writeing done today?\r\n\
    When will you stop wasting time plotting fractals?\r\n";
    for body in message.lines().skip_while(|l| !l.is_empty()).skip(1) {
        println!("{}", body);
    }
    for body in message.lines().skip_while(|l| !l.is_empty()) {
        println!("{}", body);
    }
}

fn ex_15_3_6() {
    use std::iter::Peekable;

    fn parse_number<I>(tokens: &mut Peekable<I>) -> u32
    where
        I: Iterator<Item = char>,
    {
        let mut n = 0;
        loop {
            match tokens.peek() {
                Some(r) if r.is_digit(10) => {
                    n = n * 10 + r.to_digit(10).unwrap();
                }
                _ => return n,
            }
            tokens.next();
        }
    }
    let mut chars = "226153980,1766319049".chars().peekable();
    let n1 = parse_number(&mut chars);
    println!("{}", n1);
    chars.next();
    let n2 = parse_number(&mut chars);
    println!("{}", n2);
}

fn ex_15_3_7() {
    struct Flaky(bool);

    impl Iterator for Flaky {
        type Item = &'static str;
        fn next(&mut self) -> Option<Self::Item> {
            if self.0 {
                self.0 = false;
                Some("totally the last item")
            } else {
                self.0 = true;
                None
            }
        }
    }

    {
        let mut flaky = Flaky(true);
        println!("{:?}", flaky.next());
        println!("{:?}", flaky.next());
        println!("{:?}", flaky.next());
    }
    {
        let mut flaky = Flaky(true).fuse();
        println!("{:?}", flaky.next());
        println!("{:?}", flaky.next());
        println!("{:?}", flaky.next());
    }
}

fn ex_15_3_8() {
    let bee_parts = ["head", "thorax", "abdomen"];

    let mut iter = bee_parts.iter();
    println!("{:?}", iter.next());
    println!("{:?}", iter.next_back());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next_back());

    let meals = ["breakfast", "lunch", "dinner"];
    let mut iter = meals.iter().rev();

    while let Some(e) = iter.next() {
        println!("{}", e);
    }
}

fn ex_15_3_9() {
    let upper_case: String = "große"
        .chars()
        .inspect(|c| println!("before: {:?}", c))
        .flat_map(|c| c.to_uppercase())
        .inspect(|c| println!(" after:  {:?}", c))
        .collect();
    println!("{}", upper_case);
}

fn ex_15_3_10() {
    let a1 = [1, 2, 3, 4];
    let a2 = [10, 20, 30, 40];

    let mut it = a1.iter().chain(a2.iter());
    while let Some(e) = it.next() {
        println!("{}", e);
    }

    let a3 = 1..4;
    let v: Vec<i32> = a3.chain(a2).collect();
    println!("{:?}", v);
}

fn ex_15_3_12() {
    let v: Vec<_> = (0..).zip("ABCD".chars()).collect();
    println!("{:?}", v);

    use std::iter::repeat;
    let endings = ["once", "twice", "chicken sout with rice"];
    let rhyme: Vec<_> = repeat("going").zip(endings).collect();
    println!("{:?}", rhyme);
}

fn ex_15_3_13() {
    let message = "To: jimb\r\n\
    From: supergo <editor@oreilly.com>\r\n\
    \r\n\
    Did you get any writeing done today?\r\n\
    When will you stop wasting time plotting fractals?\r\n";

    let mut lines = message.lines();

    println!("Headers:");
    for header in lines.by_ref().take_while(|l| !l.is_empty()) {
        println!("{}", header);
    }

    println!("\nBody:");
    for body in lines {
        println!("{}", body);
    }
}

fn ex_15_3_14() {
    let a = ['1', '2', '3', '∞'];
    assert_eq!(a.iter().next(), Some(&'1'));
    assert_eq!(a.iter().cloned().next(), Some('1'));

    println!("{:?}", a.iter().next());
    println!("{:?}", a.iter().cloned().next());
}

fn ex_15_3_15() {
    let dirs = ["North", "East", "South", "West"];
    let mut spin = dirs.iter().cycle();
    for _ in 1..10 {
        println!("{}", spin.next().unwrap());
    }

    use std::iter::{once, repeat};

    let fizzes = repeat("").take(2).chain(once("fizz")).cycle();
    let buzzes = repeat("").take(5).chain(once("buzz")).cycle();
    let fizzes_buzzes = fizzes.zip(buzzes);

    let fizz_buzz = (1..100).zip(fizzes_buzzes).map(|tuple| match tuple {
        (i, ("", "")) => i.to_string(),
        (_, (fizz, buzz)) => format!("{}{}", fizz, buzz),
    });

    for line in fizz_buzz {
        println!("{}", line);
    }
}
