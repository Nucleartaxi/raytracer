use super::vec3;
use super::utils;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Color { //internally stores colors as values between 0.0 and 1.0. Writes colors as u8 values from 0 to 255
    v: vec3::Vec3,
}

impl Color {
    pub fn new_empty() -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color {
            v: vec3::Vec3::new(r, g, b)
        }
    }
    pub fn r(&self) -> f64 {
        self.v.x()
    }
    pub fn g(&self) -> f64 {
        self.v.y()
    }
    pub fn b(&self) -> f64 {
        self.v.z()
    }
    pub fn add(&self, c: Color) -> Color {
        Color {
            v: vec3::Vec3::new(self.r() + c.r(), self.g() + c.g(), self.b() + c.b()),
        }
    }
    pub fn multiply_by(&self, x: f64) -> Color {
        Color {
            v: self.v.multiply_by(x),
        }
    }
    pub fn write_color(&self, v: &mut Vec<u8>, samlpes_per_pixel: u32) {
        let mut r = self.r();
        let mut g = self.g();
        let mut b = self.b();

        //divide the color by the number of samples
        let scale = 1.0 / samlpes_per_pixel as f64;
        r = (r * scale).sqrt();
        g = (g * scale).sqrt();
        b = (b * scale).sqrt();

        //write the translated [0, 255] value of each color component
        v.push((256.0 * utils::clamp(r, 0.0, 0.999)) as u8); 
        v.push((256.0 * utils::clamp(g, 0.0, 0.999)) as u8); 
        v.push((256.0 * utils::clamp(b, 0.0, 0.999)) as u8); 
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_rgb() {
        let c1 = Color::new(0.1, 0.2, 0.3);
        assert_eq!(c1.r(), 0.1);
        assert_eq!(c1.g(), 0.2);
        assert_eq!(c1.b(), 0.3);
    }
    #[test]
    fn test_new_empty() {
        let c1 = Color::new_empty();
        let c2 = Color {v: vec3::Vec3::new_empty()};
        assert_eq!(c1, c2);
    }
    #[test]
    fn test_write_color() {
        let c1 = Color::new(1.0, 0.5, 0.25);
        let mut v1: Vec<u8> = Vec::new();
        c1.write_color(&mut v1, 1);
        let v2: Vec<u8> = vec![255, 127, 63];
        assert_eq!(v1, v2);
    }
}