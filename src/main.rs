use rand::*;
use std::rc::Rc;
use material::*;
use color::*;
use vec3::*;
use hittable::*;
// mod foo;
mod image;
mod vec3;
mod color;
mod ray;
mod hittable;
mod utils;
mod camera;
mod material;

fn main() {
    //image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const WIDTH: u32 = 400;
    const HEIGHT: u32 = (WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 50;
    const MAX_DEPTH: u32 = 50;

    //world
    let mut world = hittable::HittableList::new();
    // world.add(hittable::Sphere::new(vec3::Vec3::new(0.0, 0.0, -1.0), 0.5, Rc::new(material::Lambertian::new()))); //center sphere
    // world.add(hittable::Sphere::new(vec3::Vec3::new(0.0, -100.5, -1.0), 100.0, Rc::new(material::Lambertian::new()))); //ground
    // world.add(hittable::Sphere::new(vec3::Vec3::new(1.0, 0.2, -1.0), 0.3));
    // world.add(hittable::Sphere::new(vec3::Vec3::new(-3.0, -0.5, -3.0), 1.0));
    // world.add(hittable::Sphere::new(vec3::Vec3::new(-2.5, 1.5, -3.0), 0.4));
    // world.add(hittable::Sphere::new(vec3::Vec3::new(1.0, -0.2, -0.75), 0.1));

    world.add(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0))))); //ground
    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3))))); //center
    world.add(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3))));
    world.add(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0))));
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