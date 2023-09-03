use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    ex_5_1();
    ex_5_2_1();
    ex_5_2_2();
    ex_5_2_3();
    ex_5_2_4();
    ex_5_2_6();

    ex_5_3_2();
    ex_5_3_3();

    ex_5_3_4();
    ex_5_3_5();
    ex_5_3_6();
    ex_5_3_7();

    ex_5_4();
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
    fn factorial(n: usize) -> usize {
        (1..n + 1).product()
    }
    let r = &factorial(6);
    assert_eq!(r + &1009, 1729);
}

#[allow(dead_code)]
fn ex_5_3_1() {
    {
        let r;
        {
            let x = 1;
            r = &x; //^^ borrowed value does not live long enough
            assert_eq!(*r, 1);
        }
        //- `x` dropped here while still borrowed
        //assert_eq!(*r, 1);
    }
}

static mut STASH: &i32 = &128;

fn ex_5_3_2() {
    fn f(p: &'static i32) {
        unsafe {
            STASH = p;
        }
    }
    static WORTH_POINTING_AT: i32 = 1000;
    f(&WORTH_POINTING_AT);
    unsafe {
        println!("{}", STASH);
    }
}

fn ex_5_3_3() {
    fn g<'a>(p: &'a i32) {
        println!("{}", p * 2);
    }
    let x = 10;
    g(&x);

    fn f(p: &'static i32) {
        println!("{}", p * 3);
    }
    let xx = 10;
    //f(&xx); //borrowed value does not live long enough
    static STATIC_X: i32 = 11;
    f(&STATIC_X);
}

fn ex_5_3_4() {
    fn smallest(v: &[i32]) -> &i32 {
        let mut s = &v[0];
        for r in &v[1..] {
            if *r < *s {
                s = r;
            }
        }
        s
    }
    // let s;
    {
        let parabola = [9, 4, 1, 0, 1, 4, 9];
        let s = smallest(&parabola);
        assert_eq!(*s, 0);
    }
    //assert_eq!(*s, 0);
}

fn ex_5_3_5() {
    // struct S{
    //     // r: &i32 // error[E0106]: missing lifetime specifier
    //     r: &'static i32
    // }

    struct S<'a> {
        r: &'a i32,
    }

    // struct D {
    //     //s: S //error[E0106]: missing lifetime specifier
    //     s: S<'static>
    // }

    struct D<'a> {
        s: &'a S<'a>,
    }

    let s;
    let d;
    let x = 10;
    {
        s = S { r: &x };
        d = D { s: &s }
    }

    assert_eq!(*s.r, 10);
    assert_eq!(*d.s.r, 10);
}

fn ex_5_3_6() {
    struct S<'a, 'b> {
        x: &'a i32,
        y: &'b i32,
    }
    let x = 10;
    let r;
    {
        let y = 20;
        {
            let s = S { x: &x, y: &y }; //^^ borrowed value does not live long enough
            r = s.x;
        }
        println!("{}", r);
    }
    println!("{}", r);
}

fn ex_5_3_7() {
    struct S<'a, 'b> {
        x: &'a i32,
        y: &'b i32,
    }

    fn sum_r_xy(r: &i32, s: S) -> i32 {
        r + s.x + s.y
    }
    let sum;
    let x = 10;
    {
        let y = 20;
        {
            let r = 100;
            {
                let s = S { x: &x, y: &y };
                sum = sum_r_xy(&r, s);
                println!("{}", sum);
            }
        }
    }

    fn first_third<'a>(point: &'a [i32; 3]) -> (&'a i32, &'a i32) {
        (&point[0], &point[2])
    }
    let point = [9, 4, 1];
    let ft = first_third(&point);
    println!("{:?}", ft);
}

fn ex_5_4() {
    let v = vec![4, 8, 19, 27, 34, 10];
    //let r = &v;
    {
        let r = &v;
        println!("{:?}", r);
    }
    let aside = v; // error[E0505]: cannot move out of `v` because it is borrowed
                   //println!("{}", r[0]);
    println!("{:?}", aside);

    fn extend(vec: &mut Vec<f64>, slice: &[f64]) {
        for elt in slice {
            vec.push(*elt);
        }
    }

    let mut wave = Vec::new();
    let head = vec![0.0, 1.0];
    let tail = [0.0, -1.0];
    extend(&mut wave, &head);
    println!("{:?}", wave);
    extend(&mut wave, &tail);
    println!("{:?}", wave);

    assert_eq!(wave, vec![0.0, 1.0, 0.0, -1.0]);

    //extend(&mut wave, &wave);  //error[E0502]: cannot borrow `wave` as immutable because it is also borrowed as mutable

    {
        let mut x = 10;
        let r1 = &x;
        let r2 = &x;
        //x += 10; // error[E0506]: cannot assign to `x` because it is borrowed
        //let m = &mut x; // error[E0502]: cannot borrow `x` as mutable because it is also borrowed as immutable
        println!("{}, {}, {}", r1, r2, x);
    }
    {
        let mut y = 20;
        let m1 = &mut y;
        // let m2 = &mut y; // error[E0499]: cannot borrow `y` as mutable more than once at a time
        // let z = y; // error[E0503]: cannot use `y` because it was mutably borrowed
        // println!("{}, {}, {}", m1, m2, z);
    }
    {
        let mut w = (107, 109);
        let r = &w;
        let r0 = &r.0;
        // let m1 = &mut r.1; // error[E0596]: cannot borrow `r.1` as mutable, as it is behind a `&` reference
        println!("{}", r0);
    }
    {
        let mut v = (136, 139);
        let m = &mut v;
        let m0 = &mut m.0;
        *m0 = 137;

        let r1 = &m.1;
        // println!("{}", v.1); // error[E0502]: cannot borrow `v.1` as immutable because it is also borrowed as mutable
        println!("{}", r1);
    }

    {
        struct File {
            descriptor: i32
        }
        fn new_file(d: i32) -> File {
            File { descriptor: d}
        }
        fn close_from(this: &mut File, rhs: &File){
            //close( this.descriptor);
        }
    }
}
