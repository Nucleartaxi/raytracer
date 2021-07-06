use super::vec3;
use super::color;

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
    pub fn at(&self, t: f64) -> vec3::Vec3 {
        let dir = self.direction.multiply_by(t);
        self.origin.add(&dir)
    }
    pub fn ray_color(&self) -> color::Color { //does math with vectors, then returns a color
        let unit_direction = self.direction.unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        let start_value = vec3::Vec3::new(1.0, 1.0, 1.0);
        let end_value = vec3::Vec3::new(0.5, 0.7, 1.0);
        let temp = start_value.multiply_by(1.0 - t).add(&end_value.multiply_by(t));
        // let temp = vec3::Vec3::new(1.0, 1.0, 1.0).multiply_by(1.0 - t).add(&vec3::Vec3::new(0.5, 0.7, 1.0).multiply_by(t));
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