mod genqueue;
mod grayscale;
mod queue;
mod vec;

fn main() {
    println!("Hello, world!");
    ex_9_1();
    ex_9_2();
    ex_9_3();
    ex_9_5();
    ex_9_5_1();
    ex_9_5_2();

    ex_9_6();
    ex_9_7();
    ex_9_8();
    ex_9_9();
    ex_9_10();
    ex_9_11();
}

fn new_map(size: (usize, usize), pixels: Vec<u8>) -> grayscale::GrayscaleMap {
    assert_eq!(size.0 * size.1, pixels.len());
    grayscale::GrayscaleMap { pixels, size }
}

fn ex_9_1() {
    let width = 1024;
    let height = 576;
    let image = grayscale::GrayscaleMap {
        pixels: vec![0; width * height],
        size: (width, height),
    };
    assert_eq!(image.size, (width, height));

    let hokey = Broom {
        name: "Hokey".to_string(),
        height: 60,
        health: 100,
        position: (100.0, 200.0, 0.0),
        intent: BroomIntent::FetchWater,
    };
    let (hokey1, hokey2) = chop(hokey);
    println!("{:?} {:?}", hokey1, hokey2);
    assert_eq!(hokey1.name, "Hokey I");
    assert_eq!(hokey2.name, "Hokey II");
    assert_eq!(hokey1.height, hokey2.height);
}

#[derive(Debug)]
struct Broom {
    name: String,
    height: u32,
    health: u32,
    position: (f32, f32, f32),
    intent: BroomIntent,
}

#[derive(Debug, Copy, Clone)]
enum BroomIntent {
    FetchWater,
    DumpWater,
}

fn chop(b: Broom) -> (Broom, Broom) {
    let mut broom1 = Broom {
        height: b.height / 2,
        ..b
    };
    let mut broom2 = Broom {
        name: broom1.name.clone(),
        ..broom1
    };
    broom1.name.push_str(" I");
    broom2.name.push_str(" II");

    (broom1, broom2)
}

fn ex_9_2() {
    #[derive(Debug)]
    struct Bounds(usize, usize);

    let image_bounds = Bounds(1024, 768);
    println!("{:#?}", image_bounds);

    #[derive(Debug)]
    struct Ascii(Vec<u8>);
    let ascii: Ascii = Ascii(vec![1, 2, 3, 255]);
    println!("{:?}", ascii);
}

fn ex_9_3() {
    #[derive(Debug)]
    struct Onesuch;

    let o = Onesuch;
    println!("{:?}", o);
    assert_eq!((..), std::ops::RangeFull);
}

fn ex_9_5() {
    let mut q = queue::Queue {
        older: Vec::new(),
        younger: Vec::new(),
    };
    q.push('0');
    q.push('1');
    assert_eq!(q.pop(), Some('0'));
    q.push('∞');
    assert_eq!(q.pop(), Some('1'));
    println!("is_empty {}", q.is_empty());
    assert_eq!(q.pop(), Some('∞'));
    assert_eq!(q.pop(), None);
    println!("is_empty {}", q.is_empty());
    println!("{:?}", q);

    q.push('P');
    q.push('D');
    assert_eq!(q.pop(), Some('P'));
    q.push('X');
    let (older, younger) = q.split();
    assert_eq!(older, vec!['D']);
    assert_eq!(younger, vec!['X']);
}

fn ex_9_5_1() {
    let mut bq = Box::new(queue::Queue {
        older: Vec::new(),
        younger: Vec::new(),
    });
    bq.push('■');
    println!("{:?}", bq);
}

fn ex_9_5_2() {
    let mut q = queue::Queue::new();
    q.push('*');
}

fn ex_9_6() {
    let scaled = vec::Vector2::UNIT.scaled_by(2.0);
    println!("{:?}", scaled);
    let zero = vec::Vector2::ZERO;
    println!("{:?}", zero);
}

fn ex_9_7() {
    // let mut q = genqueue::Queue::<char>::new();
    let mut q = genqueue::Queue::new();
    let mut r = genqueue::Queue::new();
    q.push("CAD");
    r.push(0.74);
    q.push("BTC");
    r.push(13764.0);
    println!("{:?}", q);
    println!("{:?}", r);
}

fn ex_9_8() {
    struct Extrema<'elt> {
        greatest: &'elt i32,
        least: &'elt i32,
    }
    fn find_extrema<'s>(slice: &'s [i32]) -> Extrema<'s> {
        let mut greatest = &slice[0];
        let mut least = &slice[0];
        for i in 1..slice.len() {
            if slice[i] < *least {
                least = &slice[i];
            };
            if slice[i] > *greatest {
                greatest = &slice[i];
            }
        }
        Extrema { greatest, least }
    }
    let a = [0, -3, 0, 15, 48];
    let e = find_extrema(&a);
    assert_eq!(*e.least, -3);
    assert_eq!(*e.greatest, 48);
}

fn ex_9_9() {
    struct Polynomial<const N: usize> {
        coefficients: [f64; N],
    }

    impl<const N: usize> Polynomial<N> {
        fn new(coefficients: [f64; N]) -> Polynomial<N> {
            Polynomial { coefficients }
        }

        fn eval(&self, x: f64) -> f64 {
            let mut sum = 0.0;
            for i in (0..N).rev() {
                sum = self.coefficients[i] + x * sum;
            }
            sum
        }
    }

    use std::f64::consts::FRAC_PI_2;
    let sine_poly = Polynomial::new([0.0, 1.0, 0.0, -1.0 / 6.0, 0.0, 1.0 / 120.0]);
    assert_eq!(sine_poly.eval(0.0), 0.0);
    assert!((sine_poly.eval(FRAC_PI_2) - 1.).abs() < 0.005);
}

fn ex_9_10() {
    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point {
        x: f64,
        y: f64,
    }
    let p = Point { x: 0.0, y: 1.0 };
    let q = &p;
    let mut r = p;
    //r.y *= 2.0;
    println!("{:?} : {:?} : {:?}", p, q, r);
    if p == r {
        println!("p == r");
    } else {
        println!("p != r");
    }
}

mod spider;
use std::cell::RefCell;

fn ex_9_11() {
    let ref_cell: RefCell<String> = RefCell::new("hello".to_string());
    {
        let r = ref_cell.borrow();
        let count = r.len();
        assert_eq!(count, 5);
    }

    let mut w = ref_cell.borrow_mut();
    w.push_str(" world");
    println!("{}", w);
}
