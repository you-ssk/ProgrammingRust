fn main() {
    ex_13_1();
    ex_13_2();
    ex_13_5();
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
