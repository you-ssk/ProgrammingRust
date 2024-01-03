fn main() {
    ex_13_1();
    ex_13_2();
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
        a = Appellation { name: "Hera".to_string(), nicknames: vec![]};
        println!("at end of block");
    }
}

fn ex_13_2(){
    struct RcBox<T: ?Sized>{
        ref_count: usize,
        value: T,
    }
    let boxed_lunch : RcBox<String> = RcBox {
        ref_count: 1,
        value: "lunch".to_string()
    };
    use std::fmt::Display;
    let boxed_displayable: &RcBox<dyn Display> = &boxed_lunch;
    fn display(boxed: &RcBox<dyn Display>) {
        println!("For your enjoyment: {}", &boxed.value);
    }
    display(&boxed_lunch);
}
