fn main() {
    println!("Hello, world!");
    let v = build_vector();
    println!("{:?}", v);
    let v = build_vector_infer();
    println!("{:?}", v);
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
