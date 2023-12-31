fn main() {
    ex_13_1();
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
