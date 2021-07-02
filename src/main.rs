// mod foo;
mod image;
mod vec3;
mod color;

fn main() {
    //image
    const WIDTH: u32 = 256;
    const HEIGHT: u32 = 256;

    //render
    let mut v: Vec<u8> = Vec::new();
    image::create_test_image(&mut v, WIDTH, HEIGHT);

    //output
    image::output_image(&v, WIDTH, HEIGHT);
}
