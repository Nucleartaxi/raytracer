// mod foo;
mod image;




fn main() {
    println!("Hello, world!");
    const WIDTH: u32 = 256;
    const HEIGHT: u32 = 256;
    let size = (WIDTH * HEIGHT * 3) as usize;
    let mut v: Vec<u8> = Vec::with_capacity(size);
    v.resize(size, 0);

    image::create_test_image(&mut v, WIDTH, HEIGHT);
    // for i in &v {
    //     println!("{}", i);
    // }
    image::output_image(&v, WIDTH, HEIGHT)
}
