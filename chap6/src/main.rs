use http::StatusCode;

fn main() {
    println!("Hello, world!");
    ex_6_1();
}

fn ex_6_1() {
    let a = 20;
    let status = if a <= 10 {
        http::StatusCode::OK
    } else {
        http::StatusCode::INTERNAL_SERVER_ERROR
    };
    println!("{} {}", a, status);
}
