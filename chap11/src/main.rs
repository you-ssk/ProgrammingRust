use std::io::Write;

fn main() {
    println!("Hello, world!");
    ex_11_0();
}

fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}

fn ex_11_0() -> std::io::Result<()>{
    use std::fs::File;
    let mut local_file = File::create("hello.txt")?;
    say_hello(&mut local_file)?;

    let mut bytes = vec![];
    say_hello(&mut bytes)?;
    assert_eq!(bytes, b"hello world\n");
    Ok(())
}
