// mod foo;
mod image;




fn main() {
    println!("Hello, world!");
    const WIDTH: u32 = 2;
    const HEIGHT: u32 = 1;
    // let size = WIDTH * HEIGHT * 3;
    // let a: [u8; size];
    let mut v: Vec<u8> = Vec::new();
    image::create_test_image(&mut v, WIDTH, HEIGHT);
    for i in v {
        println!("{}", i);
    }
}
