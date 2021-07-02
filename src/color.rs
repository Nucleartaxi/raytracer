#[derive(Debug)]
#[derive(PartialEq)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    pub fn new_empty() -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color {
            r,
            g,
            b,
        }
    }
    pub fn r(&self) -> f64 {
        self.r
    }
    pub fn g(&self) -> f64 {
        self.g
    }
    pub fn b(&self) -> f64 {
        self.b
    }
    pub fn write_color(&self, v: &mut Vec<u8>) {
        v.push((255.999 * self.r) as u8); 
        v.push((255.999 * self.g) as u8); 
        v.push((255.999 * self.b) as u8); 
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
        let c2 = Color {r: 0.0, g: 0.0, b: 0.0};
        assert_eq!(c1, c2);
    }
    #[test]
    fn test_write_color() {
        let c1 = Color::new(1.0, 0.5, 0.25);
        let mut v1: Vec<u8> = Vec::new();
        c1.write_color(&mut v1);
        let v2: Vec<u8> = vec![255, 127, 63];
        assert_eq!(v1, v2);
    }
}