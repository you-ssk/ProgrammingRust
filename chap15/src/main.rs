fn main() {
    fn triangle(n: i32)->i32 {
        let mut sum = 0;
        for i in 1..=n {
            sum += i;
        }
        sum
    }
    println!("{}", triangle(10));

    fn triangle2(n: i32) -> i32 {
        (1..=n).fold(0, |sum, item| sum + item)
    }
    println!("{}", triangle2(10));
}
