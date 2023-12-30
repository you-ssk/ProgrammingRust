use num::Complex;

fn main() {
    println!("Hello, world!");
    let c = Complex {
        re: 1.0,
        im: -0.0625,
    };
    println!("{}", c);
    let mut z = Complex { re: 0.0, im: 0.0 };
    // for i in 0..10 {
    //     z = z * z + c;
    //     println!("{}", z);
    // }

    ex_12_1();
    ex_12_1_1();
    ex_12_1_2();
    ex_12_1_3();
    ex_12_2();
    ex_12_3();
}

use std::cmp::Reverse;
use std::{ascii::EscapeDefault, ops::Add};

fn ex_12_1() {
    let mut c = Complex {
        re: 1.0,
        im: -0.0625,
    };
    let mut z = Complex { re: 0.0, im: 0.0 };
    z = z + c;
    z = z.add(c);
    println!("{}", z);
}

use std::ops::{AddAssign, BitAnd, BitOr, BitXor, Neg};

// impl<T> Neg for Complex<T>
// where
//     T: Neg<Output = T>,
// {
//     type Output = Complex<T>;
//     fn neg(self) -> Complex<T> {
//         Complex {
//             re: -self.re,
//             im: -self.im,
//         }
//     }
// }

fn ex_12_1_1() {
    let mut z = Complex { re: 0.0, im: 0.0 };
    println!("{}", z);
    z = z.neg();
    println!("{}", z);
}

use std::fmt::Write as _;

fn ex_12_1_2() {
    let mut x = 0b101;
    let mut y = 0b010;
    x = x.add(y);
    println!("{:b} {:b}", x, y);
    x = 10;
    y = 5;
    println!("{:b} {:o} {:x} {:X}", x, x, x, x);
    println!(
        "{:b} {:b} {:b} {:b}",
        x.bitand(y),
        x.bitor(y),
        (x << 1).bitxor(y),
        x << 1
    );

    let mut s = String::new();
    write!(&mut s, "{} {}", "abc", 123).unwrap(); // uses fmt::Write::write_fmt
    println!("{}", s);
}

fn ex_12_1_3() {
    let mut x = 3;
    let y = 4;
    x += y;
    println!("x:{} y:{}", x, y);

    {
        let mut z = Complex { re: 0.5, im: 0.5 };
        let x = Complex { re: 0.25, im: 0.01 };
        z += x;
        z.add_assign(x);
        println!("{}", z);
    }
}

fn ex_12_2() {
    let x = true;
    let mut y = true;
    assert_eq!(x == y, x.eq(&y));
    y = false;
    assert_eq!(x != y, x.ne(&y));

    {
        let x = Complex { re: 5, im: 2 };
        let y = Complex { re: 2, im: 5 };

        println!("{}", x.eq(&y));
        assert_eq!(x * y, Complex { re: 0, im: 29 });
    }
    {
        let s = "d\x6fv\x65t\x61i\x6c".to_string();
        let t = "\x64o\x76e\x74a\x69l".to_string();
        println!("{} {}", s, t);
        assert!(s == t);
        assert_eq!(format!("{} {}", s, t), "dovetail dovetail");

        assert!("ungula" != "ungulate");
        assert!("ungula".ne("ungulate"));
    }
    {
        println!("{}", 0.0 / 0.0);
        assert!(f64::is_nan(0.0 / 0.0));
        assert_eq!(0.0 / 0.0 == 0.0 / 0.0, false);
        assert_eq!(0.0 / 0.0 != 0.0 / 0.0, true);

        assert_eq!(0.0 / 0.0 < 0.0 / 0.0, false);
        assert_eq!(0.0 / 0.0 > 0.0 / 0.0, false);
        assert_eq!(0.0 / 0.0 <= 0.0 / 0.0, false);
        assert_eq!(0.0 / 0.0 >= 0.0 / 0.0, false);
        println!("{}", f64::NAN);
    }
}

fn ex_12_3() {
    #[derive(Debug, PartialEq)]
    struct Interval<T> {
        lower: T,
        upper: T,
    }
    use std::cmp::{Ordering, PartialOrd};

    impl<T: PartialOrd> PartialOrd<Interval<T>> for Interval<T> {
        fn partial_cmp(&self, other: &Interval<T>) -> Option<Ordering> {
            if self == other {
                Some(Ordering::Equal)
            } else if self.lower >= other.upper {
                Some(Ordering::Greater)
            } else if self.upper <= other.lower {
                Some(Ordering::Less)
            } else {
                None
            }
        }
    }

    let x = Interval {
        lower: 10,
        upper: 20,
    };
    let y = Interval {
        lower: 20,
        upper: 40,
    };
    println!("{:?} {:?}", x, y);
    assert!(x < y);
    assert!(Interval { lower: 7, upper: 8 } >= Interval { lower: 0, upper: 1 });
    assert!(Interval { lower: 7, upper: 8 } <= Interval { lower: 7, upper: 8 });

    let left = Interval {
        lower: 10,
        upper: 30,
    };
    let right = Interval {
        lower: 20,
        upper: 30,
    };
    assert!(!(left < right));
    assert!(!(left >= right));

    {
        let mut intervals = [
            Interval{lower: 1, upper: 3},
            Interval{lower: -1, upper: 2},
            Interval{lower: 4, upper:8},
            Interval{lower: 0, upper:4},
        ];
        println!("{:?}", intervals);
        intervals.sort_by_key(|i| i.upper);
        println!("{:?}", intervals);
        intervals.sort_by_key(|i| i.lower);
        println!("{:?}", intervals);
        intervals.sort_by_key(|i| Reverse(i.lower));
        println!("{:?}", intervals);
    }
}
