fn main() {
    println!("Hello, world!");
    let v = build_vector();
    println!("{:?}", v);
    let v = build_vector_infer();
    println!("{:?}", v);

    assert_eq!(10_i8 as u16, 10_u16);
    assert_eq!(2525_u16 as i16, 2525_i16);

    assert_eq!(-1_i16 as i32, -1_i32);
    assert_eq!(65535_u16 as i32, 65535_i32);

    println!("{}:{}", 1000_i16, 1000_i16 as u8);
    assert_eq!(1000_i16 as u8, 232_u8);

    assert_eq!(65535_u32 as i16, -1_i16);

    assert_eq!(-1_i8 as u8, 255_u8);
    assert_eq!(255_u8 as i8, -1_i8);

    assert_eq!(2_u16.pow(4), 16);
    assert_eq!((-4_i32).abs(), 4);
    assert_eq!(0b101101_u8.count_ones(), 4);

    // let mut i = 1;
    // loop {
    //     i *= 2;
    //     println!("i = {:b}", i);
    //     if i==0 {break;}
    // }

    // let mut i: i32 =1 ;
    // loop {
    //     println!("i = {:b}", i);
    //     i = i.checked_mul(10).expect("multiplication overflowed");
    // }

    assert_eq!(10_u8.checked_add(20), Some(30));
    assert_eq!(100_u8.checked_add(200), None);
    let x = 127_u8;
    //let x = 128_u8;
    let sum = x.checked_add(x).unwrap();
    println!("sum : {}", sum);

    assert_eq!(100_u16.wrapping_mul(200), 20000);
    assert_eq!(500_u16.wrapping_mul(500), 53392);

    //println!("{}",500_i16.wrapping_mul(500));
    println!("{}", i16::MAX);
    assert_eq!(500_i16.wrapping_mul(500), -12144);

    assert_eq!(32760_i16.saturating_add(10), 32767);
    assert_eq!((-32760_i16).saturating_sub(10), -32768);

    assert_eq!(255_u8.overflowing_sub(2), (253, false));
    assert_eq!(255_u8.overflowing_add(2), (1, true));

    assert_eq!(5_u16.overflowing_shl(17), (10, true));

    ex_3_1_3();
    ex_3_2();
    ex_3_3();
    ex_3_4();
    ex_3_5();
    ex_3_6_1();
    ex_3_6_2();
    ex_3_6_3();
    ex_3_7();
}

use regex::Regex;

fn ex_3_7() {
    let speech = "\"Ouch!\" said the well.\n";
    println!("{}", speech);

    println!(
        "In the room the woma come and go,
    Singing of Mount
     Abora"
    );

    println!("It was a bright, cold day in April, and \
    there were four of us-\
    more or less.");

    let default_with_install_path = r"C:\Program Files\Gorillas";
    let pattern = Regex::new(r"\d:(\.\d+)*");

    println!(r###"
    This raw string started with 'r###"'.
    Therefore it does not end untail we reach a quote mark ('*')
    followed immediately by three pound signs ('###'):"###);

    let method = b"GET";
    assert_eq!(method, &[b'G', b'E', b'T']);

    {
        let noodles = "noodles".to_string();
        let oodles = &noodles[1..];
        let poodles = "ಠ_ಠ";
        println!("{} {} {}", noodles, oodles, poodles);
        assert_eq!(poodles.len(), 7);
        assert_eq!(poodles.chars().count(), 3);

        //let mut s = "Hello";
        //s[0] = 'c';
        //s.push('\n');

        let bits = vec!["veni", "vidi", "vici"];
        assert_eq!(bits.concat(), "venividivici");
        assert_eq!(bits.join(", "), "veni, vidi, vici");
    }
    {
        assert!("peanut".contains("nut"));
        assert_eq!("ಠ_ಠ".replace("ಠ", "■"), "■_■");
        assert_eq!("        clean\n".trim(), "clean");

        for word in "veni, vidi, vici".split(", ") {
            assert!(word.starts_with("v"));
        }
    }
}

fn ex_3_6_3() {
    let v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
    let a: [f64; 4] = [0.0, -0.707, 1.0, -0.707];

    let sv: &[f64] = &v;
    let sa: &[f64] = &a;

    fn print(n: &[f64]) {
        for elt in n {
            print!("{} ", elt);
        }
        println!("")
    }
    print(&a);
    print(&v);
    print(&sa);
    print(&sv);
    print(&v[0..2]);
    print(&sa[2..]);
    print(&sv[1..3]);
}

fn ex_3_1_3() {
    assert!((-1. / f32::INFINITY).is_sign_negative());
    assert_eq!(-f32::MIN, f32::MAX);

    assert_eq!(5f32.sqrt() * 5f32.sqrt(), 5.);
    assert_eq!((-1.01f64).floor(), -2.0);
}

fn ex_3_2() {
    assert_eq!(false as i32, 0);
    assert_eq!(true as i32, 1);
}

fn ex_3_3() {
    assert_eq!('*' as i32, 42);
    println!("{}", "ಠ_ಠ");
    assert_eq!('ಠ' as u16, 0xca0);
    assert_eq!('ಠ' as i8, -0x60);

    assert_eq!('*'.is_alphabetic(), false);
    assert_eq!('β'.is_alphabetic(), true);
    assert_eq!('8'.to_digit(10), Some(8));
    assert_eq!('ಠ'.len_utf8(), 3);
    assert_eq!(std::char::from_digit(2, 10), Some('2'));
}

fn ex_3_4() {
    let text: &str = "I see the eigenvalue in thine eye";
    let (head, tail) = text.split_at(21);
    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");
}

fn ex_3_5() {
    let t = (12, "eggs");
    let b = Box::new(t);
    println!("{:?} : {:?}", t, b);
}

fn ex_3_6_1() {
    let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
    let taxonomy = ["Animalia", "Arthropoda", "Insecta"];
    assert_eq!(lazy_caterer[3], 7);
    assert_eq!(taxonomy.len(), 3);

    let mut sieve = [true; 10000];
    for i in 2..100 {
        if sieve[i] {
            let mut j = i * i;
            while j < 10000 {
                sieve[j] = false;
                j += i;
            }
        }
    }
    assert!(sieve[211]);
    assert!(!sieve[9876]);

    let mut chaos = [3, 5, 4, 1, 2];
    chaos.sort();
    println!("chaos : {:?}", chaos);
    assert_eq!(chaos, [1, 2, 3, 4, 5]);
}

fn ex_3_6_2() {
    let mut primes = vec![2, 3, 5, 7];
    assert_eq!(primes.iter().product::<i32>(), 210);

    primes.push(11);
    primes.push(13);
    assert_eq!(primes.iter().product::<i32>(), 30030);

    let mut pal = Vec::new();
    pal.push("step");
    pal.push("on");
    pal.push("no");
    pal.push("pets");
    assert_eq!(pal, vec!["step", "on", "no", "pets"]);

    let v: Vec<i32> = (0..5).collect();
    assert_eq!(v, [0, 1, 2, 3, 4]);

    let mut palindrome = vec!["a man", "a plan", "a canal", "panama"];
    palindrome.reverse();
    assert_eq!(palindrome, vec!["panama", "a canal", "a plan", "a man"]);

    let mut v = Vec::with_capacity(2);
    assert_eq!(v.len(), 0);
    assert_eq!(v.capacity(), 2);
    v.push(1);
    v.push(2);
    assert_eq!(v.len(), 2);
    assert_eq!(v.capacity(), 2);
    v.push(3);
    assert_eq!(v.len(), 3);
    println!("capacity is now {}", v.capacity());

    {
        let mut v = vec![10, 20, 30, 40, 50];
        v.insert(3, 35);
        assert_eq!(v, [10, 20, 30, 35, 40, 50]);
        v.remove(1);
        assert_eq!(v, [10, 30, 35, 40, 50])
    }
    {
        let mut v = vec!["Snow Puff", "Glass Gem"];
        assert_eq!(v.pop(), Some("Glass Gem"));
        assert_eq!(v.pop(), Some("Snow Puff"));
        assert_eq!(v.pop(), None);
    }
    {
        let languages: Vec<String> = std::env::args().skip(1).collect();
        for l in languages {
            println!(
                "{}: {}",
                l,
                if l.len() % 2 == 0 {
                    "functional"
                } else {
                    "imperative"
                }
            );
        }
    }
}

fn build_vector() -> Vec<i16> {
    let mut v: Vec<i16> = Vec::<i16>::new();
    v.push(10);
    v.push(20i16);
    v.push(30i16);
    v
}

fn build_vector_infer() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(123);
    v.push(345);
    v.push(456);
    for i in 1..10 {
        v.push(i);
    }
    v
}
