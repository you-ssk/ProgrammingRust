fn main() {
    println!("Hello, world!");
    print_padovan();

    {
        let point = Box::new((0.625, 0.5));
        let label = format!("{:?}", point);
        assert_eq!(label, "(0.625, 0.5)");
    }

    struct Person {
        name: String,
        birth: i32,
    }

    let mut composers = Vec::new();

    composers.push(Person {
        name: "Palestrina".to_string(),
        birth: 1525,
    });
    composers.push(Person {
        name: "Dowland".to_string(),
        birth: 1563,
    });
    composers.push(Person {
        name: "Lully".to_string(),
        birth: 1632,
    });
    for composer in &composers {
        println!("{}, born {}", composer.name, composer.birth);
    }

    ex_4_2();
    ex_4_2_1();
}

fn print_padovan() {
    let mut padovan = vec![1, 1, 1];
    for i in 3..10 {
        let next = padovan[i - 3] + padovan[i - 2];
        padovan.push(next);
    }
    println!("P(1..10) = {:?}", padovan);
}

fn ex_4_2() {
    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    let t = s;
    //let u = s;
    let u: Vec<String> = t.clone();
    println!("{:?} : {:?}", t, u);
}

fn ex_4_2_1() {
    let mut ss = "Govinda".to_string();
    ss = "Siddhartha".to_string();

    {
        let mut s = "Govinda".to_string();
        let t = s;
        s = "Siddhartha".to_string();
        println!("{} : {}", s, t);
    }
    println!("{}", ss);
}
