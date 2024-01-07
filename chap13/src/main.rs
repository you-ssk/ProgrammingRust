use std::io::Read;

fn main() {
    ex_13_1();
    ex_13_2();
    ex_13_5();
    ex_13_6();
    ex_13_7();
    ex_13_9();
}

fn ex_13_1() {
    struct Appellation {
        name: String,
        nicknames: Vec<String>,
    }
    impl Drop for Appellation {
        fn drop(&mut self) {
            print!("Dropping {}", self.name);
            if !self.nicknames.is_empty() {
                print!(" (AKA {})", self.nicknames.join(", "));
            }
            println!();
        }
    }
    {
        let mut a = Appellation {
            name: "Zeus".to_string(),
            nicknames: vec![
                "cloud collector".to_string(),
                "king of the gods".to_string(),
            ],
        };
        println!("befor assignment");
        a = Appellation {
            name: "Hera".to_string(),
            nicknames: vec![],
        };
        println!("at end of block");
    }
}

fn ex_13_2() {
    struct RcBox<T: ?Sized> {
        ref_count: usize,
        value: T,
    }
    let boxed_lunch: RcBox<String> = RcBox {
        ref_count: 1,
        value: "lunch".to_string(),
    };
    use std::fmt::Display;
    let boxed_displayable: &RcBox<dyn Display> = &boxed_lunch;
    fn display(boxed: &RcBox<dyn Display>) {
        println!("For your enjoyment: {}", &boxed.value);
    }
    display(&boxed_lunch);
}

fn ex_13_5() {
    struct Selector<T> {
        elements: Vec<T>,
        current: usize,
    }
    use std::ops::{Deref, DerefMut};

    impl<T> Deref for Selector<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.elements[self.current]
        }
    }
    impl<T> DerefMut for Selector<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.elements[self.current]
        }
    }
    let mut s = Selector {
        elements: vec!['x', 'y', 'z'],
        current: 2,
    };
    assert_eq!(*s, 'z');
    assert!(s.is_alphabetic());
    *s = 'w';
    assert_eq!(s.elements, ['x', 'y', 'w']);

    {
        let s = Selector {
            elements: vec!["good", "bad", "ugly"],
            current: 2,
        };
        fn show_it(thing: &str) {
            println!("{}", thing)
        }
        show_it(&s);

        use std::fmt::Display;
        fn show_it_generic<T: Display>(thing: T) {
            println!("{}", thing);
        }
        //show_it_generic(&s);
        show_it_generic(&*s);
        show_it_generic(&s as &str);
    }
}

fn ex_13_6() {
    use std::collections::HashSet;
    let squares = [4, 9, 16, 25, 36, 49, 64];
    let (powers_of_two, impure): (HashSet<i32>, HashSet<i32>) =
        squares.iter().partition(|&n| n & (n - 1) == 0);
    assert_eq!(powers_of_two.len(), 3);
    assert_eq!(impure.len(), 4);
    println!("{:?}", powers_of_two);
    println!("{:?}", impure);

    let (upper, lower): (String, String) = "Great Teacher Onizuka"
        .chars()
        .partition(|&c| c.is_uppercase());
    assert_eq!(upper, "GTO");
    assert_eq!(lower, "reat eacher nizuka");
    println!("{:?}", upper);
    println!("{:?}", lower);
}

fn ex_13_7() {
    let mut f = std::fs::File::open(".gitignore").unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    println!("{}", buf);
}

fn ex_13_9() {
    use std::net::Ipv4Addr;
    fn ping<A>(address: A) -> std::io::Result<bool>
    where
        A: Into<Ipv4Addr>,
    {
        let ipv4_address = address.into();
        println!("{}", ipv4_address);
        Result::Ok(true)
    }
    println!("{:?}", ping(Ipv4Addr::new(23, 21, 68, 141)));
    println!("{:?}", ping([66, 146, 219, 98]));
    println!("{:?}", ping(0xd07eb94_u32));
    let addr1 = Ipv4Addr::from([66, 146, 219, 98]);
    let addr2 = Ipv4Addr::from(0xd07eb94_u32);
    println!("{:?}, {:?}", addr1, addr2);

    let huge = 2_000_000_000_000i64;
    let smaller = huge as i32;
    println!("{}", smaller);
}
