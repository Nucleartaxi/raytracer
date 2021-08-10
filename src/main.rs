use rand::*;
use std::rc::Rc;
use material::*;
use color::*;
use vec3::*;
use hittable::*;

mod image;
mod vec3;
mod color;
mod ray;
mod hittable;
mod utils;
mod camera;
mod material;

fn main() {
    // println!("{:?}", vec3::Vec3::refract(&vec3::Vec3::new(1.0, 0.0, 0.0), &vec3::Vec3::new(0.0, 0.5, 0.25), 1.5));
    // let test_sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Rc::new(Dielectric::new(1.5)));

    // test_sphere.mat.scatter(r_in: &Ray, rec: &HitRecord);

    //image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const WIDTH: u32 = 200;
    const HEIGHT: u32 = (WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 50;
    const MAX_DEPTH: u32 = 50;

    //world
    let mut world = hittable::HittableList::new();
    // world.add(hittable::Sphere::new(vec3::Vec3::new(0.0, 0.0, -1.0), 0.5, Rc::new(material::Lambertian::new()))); //center sphere
    // world.add(hittable::Sphere::new(vec3::Vec3::new(0.0, -100.5, -1.0), 100.0, Rc::new(material::Lambertian::new()))); //ground
    // world.add(hittable::Sphere::new(vec3::Vec3::new(1.0, -0.2, -0.75), 0.1, Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.7)))));

    // world.add(hittable::Sphere::new(vec3::Vec3::new(1.0, 0.2, -1.0), 0.3, Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.7)))));
    // world.add(hittable::Sphere::new(vec3::Vec3::new(-3.0, -0.5, -3.0), 1.0, Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.7)))));
    // world.add(hittable::Sphere::new(vec3::Vec3::new(-2.5, 1.5, -3.0), 0.4, Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.7)))));

    world.add(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0))))); //ground
    world.add(Sphere::new(Vec3::new(0.5, 0.0, -1.25), 0.6, Rc::new(Metal::new(Color::new(0.5, 0.5, 0.8), 0.0)))); //center
    // world.add(Sphere::new(Vec3::new(1.0, -0.1, -1.0), 0.5, Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.8))))); //right
    world.add(Sphere::new(Vec3::new(-0.6, 0.0, -1.75), 0.6, Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.8))))); //left


    // world.add(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, Rc::new(Metal::new(Color::new(1.0, 1.0, 1.0), 0.1))));
    // world.add(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0))));

    let mut rng = rand::thread_rng();
    for i in -4..5 {
        for j in -4..1 {
            let temp_mat: Rc<dyn Material> = if rng.gen_range(0.0..1.0) > 0.5 {
                Rc::new(Lambertian::new(Color::random_color()))
            } else {
                Rc::new(Metal::new(Color::random_color(), rng.gen_range(0.0..1.0)))
            };
            world.add(Sphere::new(Vec3::new(i as f64 + 0.5 * rng.gen::<f64>(), -0.3, j as f64 + 0.5 * rng.gen::<f64>()), 0.2, temp_mat));
        }
    }

    // world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Rc::new(Dielectric::new(1.5))));
    //camera
    let cam = camera::Camera::new();

    //render
    let mut image_data: Vec<u8> = Vec::new();
    // image::create_test_image(&mut image_data, WIDTH, HEIGHT);
    for i in (0..(HEIGHT)).rev() {
        println!("Lines remaining: {}", i + 1); //progress indicator in case of long renders
        for j in 0..WIDTH {
            let mut pixel_color = color::Color::new_empty();
            // println!("x: {} y: {}", i, j);
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