// mod foo;
mod image;
mod vec3;
mod color;
mod ray;

fn main() {
    //image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const WIDTH: u32 = 400;
    const HEIGHT: u32 = (WIDTH as f64 / ASPECT_RATIO) as u32;

    //camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = vec3::Vec3::new_empty();
    let horizontal = vec3::Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = vec3::Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin.subtract(&horizontal.divide_by(2.0)).subtract(&vertical.divide_by(2.0)).subtract(&vec3::Vec3::new(0.0, 0.0, focal_length));

    //render
    let mut v: Vec<u8> = Vec::new();
    image::create_test_image(&mut v, WIDTH, HEIGHT);

    //output
    image::output_image(&v, WIDTH, HEIGHT);
}
