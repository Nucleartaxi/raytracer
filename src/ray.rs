use super::vec3;
use super::color;
use super::hittable;
use super::material;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Ray {
    origin: vec3::Vec3, //origin point is a vec3 to simplify mathematical operations with other vectors
    direction: vec3::Vec3,
}
impl Ray {
    pub fn new_empty() -> Ray {
        Ray {
            origin: vec3::Vec3::new_empty(),
            direction: vec3::Vec3::new_empty(),
        }
    }
    pub fn new(origin: vec3::Vec3, direction: vec3::Vec3) -> Ray {
        Ray {
            origin,
            direction,
        }
    }
    pub fn origin(&self) -> vec3::Vec3 {
        self.origin
    }
    pub fn direction(&self) -> vec3::Vec3 {
        self.direction
    }
    pub fn at(&self, t: f64) -> vec3::Vec3 {
        let dir = self.direction.multiply_by(t);
        self.origin.add(&dir)
    }
    pub fn hit_sphere(&self, center: vec3::Vec3, radius: f64) -> f64 {
        let oc = self.origin().subtract(&center);
        let a = self.direction().length_squared();
        let half_b = oc.dot(&self.direction());
        let c = oc.length_squared() - (radius * radius);
        let discriminant = (half_b * half_b) - (a * c);
        
        if discriminant < 0.0 {
            return -1.0
        } else {
            return (-half_b - discriminant.sqrt()) / a
        }
    }
    pub fn ray_color(&self, world: & dyn hittable::Hittable, depth: u32) -> color::Color { //does math with vectors, then returns a color
        // let t = self.hit_sphere(vec3::Vec3::new(0.0, 0.0, -1.0), 0.5);
        // if t > 0.0 {
        //     let temp = self.at(t).subtract(&vec3::Vec3::new(0.0, 0.0, -1.0));
        //     let n = temp.unit_vector();
        //     let color_temp = vec3::Vec3::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
        //     let color_temp = color_temp.multiply_by(0.5);
        //     return color::Color::new(color_temp.x(), color_temp.y(), color_temp.z())
        // }
        // if we've exceeded the ray bounce limit, no more light is gathered
        if depth <= 0 {
            return color::Color::new_empty();
        }

        let (was_hit, temp_rec) = world.hit(&self, 0.001, f64::INFINITY, &hittable::HitRecord::new_empty());
        if was_hit {
            let (scattered, attenuation, scattered_ray) = temp_rec.mat.scatter(&self, &temp_rec);
            if scattered {
                return attenuation.multiply(scattered_ray.ray_color(world, depth - 1));
            }
            return color::Color::new_empty();
            // let target = temp_rec.p.add(&temp_rec.normal).add(&vec3::Vec3::random_unit_vector());
            // return Ray::new(temp_rec.p, target.subtract(&temp_rec.p)).ray_color(world, depth - 1).multiply_by(0.5);
        }

        // sky/background
        self.sky()
    }
    fn sky(&self) -> color::Color { //generates sky background
        let unit_direction = self.direction.unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);

        let start_value = vec3::Vec3::new(1.0, 1.0, 1.0);
        let end_value = vec3::Vec3::new(0.5, 0.7, 1.0);
        let temp = start_value.multiply_by(1.0 - t).add(&end_value.multiply_by(t));
        color::Color::new(temp.x(), temp.y(), temp.z())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_at() {
        let r = Ray::new(vec3::Vec3::new(1.0, 2.0, 3.0), vec3::Vec3::new(1.0, 1.0, 1.0));
        let p1 = r.at(1.0);
        assert_eq!(p1, vec3::Vec3::new(2.0, 3.0, 4.0));
        let p2 = r.at(10.0);
        assert_eq!(p2, vec3::Vec3::new(11.0, 12.0, 13.0));
    }
}