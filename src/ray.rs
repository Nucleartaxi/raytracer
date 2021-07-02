use super::vec3;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct point3 {
    x: f64,
    y: f64,
    z: f64,
}
impl point3 {
    pub fn new_empty() -> point3 {
        point3::new(0.0, 0.0, 0.0)
    }
    pub fn new(x: f64, y: f64, z: f64) -> point3 {
        point3 {x, y, z}
    }
    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }
}
#[derive(Debug)]
#[derive(PartialEq)]
pub struct ray {
    origin: point3,
    direction: vec3::Vec3,
}
impl ray {
    pub fn new_empty() -> ray {
        ray {
            origin: point3::new_empty(),
            direction: vec3::Vec3::new_empty(),
        }
    }
    pub fn new(origin: point3, direction: vec3::Vec3) -> ray {
        ray {
            origin,
            direction,
        }
    }
    pub fn at(&self, t: f64) -> point3 {
        let dir = self.direction.multiply_by(t);
        point3::new(dir.x() + self.origin.x, dir.y() + self.origin.y, dir.z() + self.origin.z)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_at() {
        let r = ray::new(point3::new(1.0, 2.0, 3.0), vec3::Vec3::new(1.0, 1.0, 1.0));
        let p1 = r.at(1.0);
        assert_eq!(p1, point3::new(2.0, 3.0, 4.0));
        let p2 = r.at(10.0);
        assert_eq!(p2, point3::new(11.0, 12.0, 13.0));
    }
}