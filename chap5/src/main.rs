use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    ex_5_1();
    ex_5_2_1();
    ex_5_2_2();
    ex_5_2_3();
    ex_5_2_4();
    ex_5_2_6();
}

type Table = HashMap<String, Vec<String>>;

fn show(table: &Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!(" {}", work);
        }
    }
}

fn sort_works(table: &mut Table) {
    for (_artist, works) in table {
        works.sort();
    }
}

fn ex_5_1() {
    let mut table = Table::new();
    table.insert(
        "Gesualdo".to_string(),
        vec![
            "many madrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );
    table.insert(
        "Caravaggio".to_string(),
        vec![
            "The Musicians".to_string(),
            "The Calling of St. Matthew".to_string(),
        ],
    );
    table.insert(
        "Cellini".to_string(),
        vec![
            "Perseus with the head of Medusa".to_string(),
            "a salt cellar".to_string(),
        ],
    );
    sort_works(&mut table);
    show(&table);

    //assert_eq!(table["Gesualdo"][0], "many madrigals");
    assert_eq!(table["Gesualdo"][1], "many madrigals");
}

fn ex_5_2_1() {
    struct Anime {
        name: &'static str,
        bechdel_pass: bool,
    }
    let aria = Anime {
        name: "Aria: The Animation",
        bechdel_pass: true,
    };
    let anime_ref = &aria;
    assert_eq!(anime_ref.name, "Aria: The Animation");
    assert_eq!((*anime_ref).name, "Aria: The Animation");

    let mut v = vec![1973, 1968];
    v.sort();
    (&mut v).sort();
    println!("{:?}", v);
}

fn ex_5_2_2() {
    let x = 20;
    let y = 10;
    let mut r = &x;
    if true {
        r = &y;
    }
    assert!(*r == 10 || *r == 20);
}

fn ex_5_2_3() {
    struct Point {
        x: i32,
        y: i32,
    }
    let point = Point { x: 1000, y: 729 };
    let r: &Point = &point;
    let rr: &&Point = &r;
    let rrr: &&&Point = &rr;
    assert_eq!(rrr.y, 729);
}

fn ex_5_2_4() {
    let x = 10;
    let y = 10;

    let rx = &x;
    let ry = &y;

    let rrx = &rx;
    let rry = &ry;

    assert!(rrx <= rry);
    assert!(rrx == rry);
    // assert!(rrx == ry); //error[E0277]: can't compare `&{integer}` with `{integer}`

    assert!(rx == ry);
    assert!(!std::ptr::eq(rx, ry));

    //assert!(rx == rrx); //error[E0277]: can't compare `{integer}` with `&{integer}`
    assert!(rx == *rrx);
}

fn ex_5_2_6() {
    fn factorial( n: usize) -> usize{
        (1..n+1).product()
    }
    let r = &factorial(6);
    assert_eq!(r + &1009, 1729);
}
