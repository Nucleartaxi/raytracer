use rand::*;
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
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_DEPTH: u32 = 50;

    //world
    let mut world = hittable::HittableList::new();
    world.add(hittable::Sphere::new(vec3::Vec3::new(0.0, 0.0, -1.0), 0.5));
    world.add(hittable::Sphere::new(vec3::Vec3::new(0.0, -100.5, -1.0), 100.0));

    //camera
    let cam = camera::Camera::new();

    //render
    let mut image_data: Vec<u8> = Vec::new();
    // image::create_test_image(&mut image_data, WIDTH, HEIGHT);
    let mut rng = rand::thread_rng();
    for i in (0..(HEIGHT)).rev() {
        println!("Lines remaining: {}", i + 1); //progress indicator in case of long renders
        for j in 0..WIDTH {
            let mut pixel_color = color::Color::new_empty();
            for s in 0..SAMPLES_PER_PIXEL {
                let u = (j as f64 + rng.gen::<f64>()) / ((WIDTH - 1) as f64);
                let v = (i as f64 + rng.gen::<f64>()) / ((HEIGHT - 1) as f64);
                let r = cam.get_ray(u, v);
                pixel_color = pixel_color.add(r.ray_color(&world, MAX_DEPTH));
            }
            pixel_color.write_color(&mut image_data, SAMPLES_PER_PIXEL);
        }
    }

    //output
    image::output_image(&image_data, WIDTH, HEIGHT);
}