fn main() {
    println!("Hello, world!");
    let v = build_vector();
    println!("{:?}", v);
    let v = build_vector_infer();
    println!("{:?}", v);

    assert_eq!(10_i8 as u16, 10_u16);
    assert_eq!(2525_u16 as i16, 2525_i16);

    assert_eq!(-1_i16 as i32, -1_i32);
    assert_eq!(65535_u16 as i32, 65535_i32);

    println!("{}:{}", 1000_i16, 1000_i16 as u8);
    assert_eq!(1000_i16 as u8, 232_u8);

    assert_eq!(65535_u32 as i16, -1_i16);

    assert_eq!(-1_i8 as u8, 255_u8);
    assert_eq!(255_u8 as i8, -1_i8);

    assert_eq!(2_u16.pow(4), 16);
    assert_eq!((-4_i32).abs(), 4);
    assert_eq!(0b101101_u8.count_ones(), 4);

    // let mut i = 1;
    // loop {
    //     i *= 2;
    //     println!("i = {:b}", i);
    //     if i==0 {break;}
    // }

    // let mut i: i32 =1 ;
    // loop {
    //     println!("i = {:b}", i);
    //     i = i.checked_mul(10).expect("multiplication overflowed");
    // }

    assert_eq!(10_u8.checked_add(20), Some(30));
    assert_eq!(100_u8.checked_add(200), None);
    let x = 127_u8;
    //let x = 128_u8;
    let sum = x.checked_add(x).unwrap();
    println!("sum : {}", sum);

    assert_eq!(100_u16.wrapping_mul(200), 20000);
    assert_eq!(500_u16.wrapping_mul(500), 53392);

    //println!("{}",500_i16.wrapping_mul(500));
    println!("{}", i16::MAX);
    assert_eq!(500_i16.wrapping_mul(500), -12144);

    assert_eq!(32760_i16.saturating_add(10), 32767);
    assert_eq!((-32760_i16).saturating_sub(10), -32768);

    assert_eq!(255_u8.overflowing_sub(2), (253,false));
    assert_eq!(255_u8.overflowing_add(2), (1, true));

    assert_eq!(5_u16.overflowing_shl(17), (10,true));
}

fn build_vector() -> Vec<i16> {
    let mut v: Vec<i16> = Vec::<i16>::new();
    v.push(10);
    v.push(20i16);
    v.push(30i16);
    v
}

fn build_vector_infer() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(123);
    v.push(345);
    v.push(456);
    for i in 1..10 {
        v.push(i);
    }
    v
}
