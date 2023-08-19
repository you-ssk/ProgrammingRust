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
    ex_4_2_2();
    ex_4_2_3();
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

fn f(x: &Vec<i32>) {
    println!("{:?}", x);
}

fn g(x: &Vec<i32>) {
    let v: Vec<i32> = x.iter().map(|i| i * i).collect();
    println!("{:?}", v);
}

fn h(x: &Vec<i32>) {
    let v: Vec<i32> = x.iter().map(|i| i + i).collect();
    println!("{:?}", v);
}

fn ex_4_2_2() {
    let x = vec![10, 20, 30];
    let c = true;
    if c {
        f(&x);
    } else {
        g(&x);
    }
    h(&x);
}

fn ex_4_2_3() {
    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }

    // let third = v[2];
    // let fifth = v[4];

    let fifth = v.pop().expect("vector empty");
    assert_eq!(fifth, "105");

    let second = v.swap_remove(1);
    assert_eq!(second, "102");

    let third = std::mem::replace(&mut v[2], "substitute".to_string());
    assert_eq!(third, "103");

    assert_eq!(v, vec!["101", "104", "substitute"]);

    {
        let x = vec![
            "liberte".to_string(),
            "egalite".to_string(),
            "fraternite".to_string(),
        ];
        for mut s in x {
            s.push('!');
            println!("{}", s);
        }
    }

    {
        struct Person {
            name: Option<String>,
            birth: i32,
        }
        let mut composers = Vec::new();
        composers.push(Person {
            name: Some("Palestrina".to_string()),
            birth: 1525,
        });
        composers.push(Person {
            name: Some("Dowland".to_string()),
            birth: 1563,
        });

        //let first_name = composers[0].name;

        //let first_name = std::mem::replace(&mut composers[0].name, None);
        let first_name = composers[0].name.take();
        assert_eq!(first_name, Some("Palestrina".to_string()));
        assert_eq!(composers[0].name, None);
    }
}
