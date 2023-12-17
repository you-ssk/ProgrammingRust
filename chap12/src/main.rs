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
}

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

use std::ops::{BitAnd, BitOr, BitXor, Neg, AddAssign};

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
        let x = Complex{re:0.25, im: 0.01};
        z += x;
        z.add_assign(x);
        println!("{}", z);
    }
}
