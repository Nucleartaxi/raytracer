// mod foo;
mod image;
mod vec3;
mod color;
mod ray;
mod hittable;
mod utils;
mod camera;

fn main() {
    //image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const WIDTH: u32 = 192;
    const HEIGHT: u32 = (WIDTH as f64 / ASPECT_RATIO) as u32;

    //world
    let mut world = hittable::HittableList::new();
    world.add(hittable::Sphere::new(vec3::Vec3::new(0.0, 0.0, -1.0), 0.5));
    world.add(hittable::Sphere::new(vec3::Vec3::new(0.0, -100.5, -1.0), 100.0));

    //camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = vec3::Vec3::new_empty();
    let horizontal = vec3::Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = vec3::Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin.subtract(&horizontal.divide_by(2.0)).subtract(&vertical.divide_by(2.0)).subtract(&vec3::Vec3::new(0.0, 0.0, focal_length));

    //render
    let mut image_data: Vec<u8> = Vec::new();
    // image::create_test_image(&mut image_data, WIDTH, HEIGHT);
    for i in (0..(HEIGHT)).rev() {
        println!("Lines remaining: {}", i + 1); //progress indicator in case of long renders
        for j in 0..WIDTH {
            let u = (j as f64) / ((WIDTH - 1) as f64);
            let v = (i as f64) / ((HEIGHT - 1) as f64);
            let r = ray::Ray::new(origin, lower_left_corner.add(&horizontal.multiply_by(u)).add(&vertical.multiply_by(v)).subtract(&origin));
            let pixel_color = r.ray_color(&world);
            pixel_color.write_color(&mut image_data);
        }
    }

    //output
    image::output_image(&image_data, WIDTH, HEIGHT);
}