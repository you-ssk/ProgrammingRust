mod grayscale;

fn main() {
    println!("Hello, world!");
    ex_9_1();
}

fn new_map(size: (usize, usize), pixels: Vec<u8>)->grayscale::GrayscaleMap {
    assert_eq!(size.0*size.1, pixels.len());
    grayscale::GrayscaleMap { pixels, size }
}

fn ex_9_1(){
    let width = 1024;
    let height = 576;
    let image = grayscale::GrayscaleMap {
        pixels: vec![0; width*height],
        size: (width, height)
    };
    assert_eq!(image.size, (width, height));
}