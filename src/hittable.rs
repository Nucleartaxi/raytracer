use super::vec3;
use super::ray;

// #[derive(Copy)]
#[derive(Debug)]
pub struct HitRecord {
	pub p: vec3::Vec3,
    pub normal: vec3::Vec3,
    pub t: f64,
    pub front_face: bool,
}
impl HitRecord {
    pub fn set_face_normal(&mut self, r: &ray::Ray, outward_normal: vec3::Vec3) {
        self.front_face = outward_normal.dot(&r.direction()) < 0.0;
        if self.front_face {
            println!("self.normal {:?}", self.normal);
            self.normal = outward_normal;
            println!("self.normal {:?}", self.normal);
        } else {
            self.normal = outward_normal.multiply_by(-1.0);
        }
    }
    pub fn new_empty() -> HitRecord {
        HitRecord {
            p: vec3::Vec3::new_empty(),
            normal: vec3::Vec3::new_empty(),
            t: 0.0,
            front_face: true,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64, rec: &HitRecord) -> (bool, HitRecord);
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
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64, rec: &HitRecord) -> (bool, HitRecord) {
        let oc = r.origin().subtract(&self.center);
        let a = r.direction().length_squared();
        let half_b = oc.dot(&r.direction());
        let c = oc.length_squared() - (self.radius * self.radius);

        let discriminant = (half_b * half_b) - (a * c);
        if discriminant < 0.0 {
            return (false, HitRecord {p: rec.p, normal: rec.normal, t: rec.t, front_face: rec.front_face})
        }
        let sqrtd = discriminant.sqrt();

        //find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrtd) / a;
        println!("root {} t_min {} t_max {}", root, t_min, t_max); 
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            println!("if2");
            if root < t_min || t_max < root {
                println!("if3");
                return (false, HitRecord {p: rec.p, normal: rec.normal, t: rec.t, front_face: rec.front_face})
            }
        }
        println!("root {}", root); 
        println!("modifying temp_rec");
        
        let mut temp_rec = HitRecord {p: rec.p, normal: rec.normal, t: rec.t, front_face: rec.front_face};
        println!("temp_rec {:?}", temp_rec);
        temp_rec.t = root;
        temp_rec.p = r.at(rec.t);
        let outward_normal = temp_rec.p.subtract(&self.center).divide_by(self.radius);
        temp_rec.set_face_normal(r, outward_normal); //sets front_face to true or false
        (true, temp_rec)
    }
}

pub struct HittableList {
    vec: Vec<Box<dyn Hittable>>,
}
impl HittableList {
    pub fn add(&mut self, s: Sphere) {
        self.vec.push(Box::new(s));
    }
    pub fn clear(&mut self) {
        self.vec.clear();
    }
    pub fn new() -> HittableList {
        HittableList {
            vec: Vec::new(),
        }
    }
}
impl Hittable for HittableList {
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64, rec: &HitRecord) -> (bool, HitRecord) {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        let return_rec = HitRecord {p: rec.p, normal: rec.normal, t: rec.t, front_face: rec.front_face};
        
        for s in &self.vec {
            let (was_hit, return_rec) = s.hit(r, t_min, closest_so_far, &rec);
            if was_hit {
                hit_anything = true;
                closest_so_far = return_rec.t;
                // return_rec = HitRecord {p: temp_rec_2.p, normal: temp_rec_2.normal, t: temp_rec_2.t, front_face: temp_rec_2.front_face};
            }
        }
        (hit_anything, return_rec)
    }
}
