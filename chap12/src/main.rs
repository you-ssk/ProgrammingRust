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

use std::ops::Neg;

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
