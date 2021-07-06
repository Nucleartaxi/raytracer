use super::vec3;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct ray {
    origin: vec3::Vec3, //origin point is a vec3 to simplify mathematical operations with other vectors
    direction: vec3::Vec3,
}
impl ray {
    pub fn new_empty() -> ray {
        ray {
            origin: vec3::Vec3::new_empty(),
            direction: vec3::Vec3::new_empty(),
        }
    }
    pub fn new(origin: vec3::Vec3, direction: vec3::Vec3) -> ray {
        ray {
            origin,
            direction,
        }
    }
    pub fn at(&self, t: f64) -> vec3::Vec3 {
        let dir = self.direction.multiply_by(t);
        self.origin.add(&dir)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_at() {
        let r = ray::new(vec3::Vec3::new(1.0, 2.0, 3.0), vec3::Vec3::new(1.0, 1.0, 1.0));
        let p1 = r.at(1.0);
        assert_eq!(p1, vec3::Vec3::new(2.0, 3.0, 4.0));
        let p2 = r.at(10.0);
        assert_eq!(p2, vec3::Vec3::new(11.0, 12.0, 13.0));
    }
}