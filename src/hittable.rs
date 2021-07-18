use super::vec3;
use super::ray;

pub struct hit_record {
	p: vec3::Vec3,
    normal: vec3::Vec3,
    t: f64,
    front_face: bool,
}
impl hit_record {
    pub fn set_face_normal(&mut self, r: ray::Ray, outward_normal: vec3::Vec3) {
        self.front_face = outward_normal.dot(&r.direction()) < 0.0;
        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = outward_normal.multiply_by(-1.0);
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: ray::Ray, t_min: f64, t_max: f64, rec: &mut hit_record) -> bool;
}

pub struct Sphere {
    pub center: vec3::Vec3,
    pub radius: f64,
}
impl Sphere {
    pub fn new(center: vec3::Vec3, radius: f64) -> Sphere {
        Sphere {
            center,
            radius,
        }
    }
}
impl Hittable for Sphere {
    fn hit(&self, r: ray::Ray, t_min: f64, t_max: f64, rec: &mut hit_record) -> bool {
        let oc = r.origin().subtract(&self.center);
        let a = r.direction().length_squared();
        let half_b = oc.dot(&r.direction());
        let c = oc.length_squared() - (self.radius * self.radius);

        let discriminant = (half_b * half_b) - (a * c);
        if discriminant < 0.0 {
            return false
        }
        let sqrtd = discriminant.sqrt();

        //find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false
            }
        }
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = rec.p.subtract(&self.center).divide_by(self.radius);
        rec.set_face_normal(r, outward_normal);
        true
    }
}
