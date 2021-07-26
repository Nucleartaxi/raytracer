use super::ray::Ray;
use super::hittable::HitRecord;
use super::color::Color;
use super::vec3;
use std::fmt;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (bool, Color, Ray); //returns true/false, attenuation, scattered
}

impl fmt::Debug for dyn Material {
    fn fmt(& self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "")
    }
}






pub struct Lambertian {
    albedo: Color,
}
impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian {albedo}
    }
}
impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (bool, Color, Ray) {
        let mut scatter_direction = rec.normal.add(&vec3::Vec3::random_unit_vector());
        //catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        (true, self.albedo, Ray::new(rec.p, scatter_direction))
    }
}
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}
impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal {albedo, fuzz}
    }
}
impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (bool, Color, Ray) {
        let reflected = vec3::Vec3::reflect(&r_in.direction().unit_vector(), &rec.normal);
        let scattered = Ray::new(rec.p, reflected.add(&vec3::Vec3::random_in_unit_sphere().multiply_by(self.fuzz)));
        (scattered.direction().dot(&rec.normal) > 0.0, self.albedo, scattered)
    }
}
pub struct Dielectric {
    ir: f64 //index of refraction
}
impl Dielectric {
    pub fn new(ir: f64) -> Dielectric {
        Dielectric {ir}
    }
}
impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (bool, Color, Ray) {
        let mut refraction_ratio = self.ir;
        if rec.front_face {
            refraction_ratio = 1.0 / self.ir;
        } 
        

        let unit_direction = r_in.direction().unit_vector();
        let refracted = vec3::Vec3::refract(&unit_direction, &rec.normal, refraction_ratio);
        (true, Color::new(1.0, 1.0, 1.0), Ray::new(rec.p, refracted))
    }
}