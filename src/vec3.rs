#[derive(Debug)]
#[derive(PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    //constructors
    pub fn new_empty() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
    pub fn new(x: f64, y: f64, z:f64) -> Vec3 {
        Vec3 {
            x,
            y,
            z,
        }
    }
    //getters
    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }
    //setters
    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }
    pub fn set_z(&mut self, z: f64) {
        self.z = z;
    }
    //operations
    pub fn add(&self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
    pub fn subtract(&self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
    pub fn multiply(&self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
    pub fn divide(&self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }
    pub fn multiply_by(&self, f: f64) -> Vec3 {
        Vec3::new(self.x * f, self.y * f, self.z * f)
    }
    pub fn divide_by(&self, d: f64) -> Vec3 {
        Vec3::new(self.x / d, self.y / d, self.z / d)
    }
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn unit_vector(&self) -> Vec3 {
        self.divide_by(self.length())
    }
    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(self.y * other.z - self.z * other.y, self.z * other.x - self.x * other.z, self.x * other.y - self.y * other.x)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_new() {
        let v1 = Vec3 {x: 1.0, y: 2.0, z: 0.0};
        let v2 = Vec3::new(1.0, 2.0, 0.0);
        assert_eq!(v1, v2);
    }
    #[test]
    fn test_new_empty() {
        let v1 = Vec3 {x: 0.0, y: 0.0, z: 0.0};
        let v2 = Vec3::new_empty();
        assert_eq!(v1, v2);
    }
    #[test]
    fn test_set_x() {
        let mut v1 = Vec3::new_empty();
        let v2 = Vec3::new(1.0, 0.0, 0.0);
        v1.set_x(1.0);
        assert_eq!(v1, v2);
    }
    #[test]
    fn test_set_y() {
        let mut v1 = Vec3::new_empty();
        let v2 = Vec3::new(0.0, 1.0, 0.0);
        v1.set_y(1.0);
        assert_eq!(v1, v2);
    }
    #[test]
    fn test_set_z() {
        let mut v1 = Vec3::new_empty();
        let v2 = Vec3::new(0.0, 0.0, 1.0);
        v1.set_z(1.0);
        assert_eq!(v1, v2);
    }
    #[test]
    fn test_x_y_z() {
        let v1 = Vec3::new(1.0, -2.0, 3.0);
        assert_eq!(v1.x(), 1.0);
        assert_eq!(v1.y(), -2.0);
        assert_eq!(v1.z(), 3.0);
    }
    #[test]
    fn test_add() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let v3 = v1.add(&v2);
        let answer = Vec3::new(5.0, 7.0, 9.0);
        assert_eq!(v3, answer);
    }
    #[test]
    fn test_subtract() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 1.0);
        let v3 = v1.subtract(&v2);
        let answer = Vec3::new(-3.0, -3.0, 2.0);
        assert_eq!(v3, answer);
    }
    #[test]
    fn test_multiply() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 1.0);
        let v3 = v1.multiply(&v2);
        let answer = Vec3::new(4.0, 10.0, 3.0);
        assert_eq!(v3, answer);
    }
    #[test]
    fn test_divide() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 1.0);
        let v3 = v1.divide(&v2);
        let answer = Vec3::new(0.25, 0.4, 3.0);
        assert_eq!(v3, answer);
    }
    #[test]
    fn test_length_squared() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v1.length_squared(), 14.0);
    }
    #[test]
    fn test_length() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v1.length(), 3.7416573867739413);
        let v2 = Vec3::new_empty();
        assert_eq!(v2.length(), 0.0);
    }
    #[test]
    fn test_multiply_by() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let answer = Vec3::new(3.0, 6.0, 9.0);
        assert_eq!(v1.multiply_by(3.0), answer);
    }
    #[test]
    fn test_divide_by() {
        let v1 = Vec3::new(2.0, 4.0, 6.0);
        let answer = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v1.divide_by(2.0), answer);
    }
    #[test]
    fn test_unit_vector() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let answer = Vec3::new(1.0 / (14.0_f64).sqrt(), (2.0 / 7.0_f64).sqrt(), 3.0 / (14.0_f64).sqrt());
        assert_eq!(v1.unit_vector(), answer);
    }
    #[test]
    fn test_dot() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(v1.dot(&v2), 32.0);
    }
    #[test]
    fn test_cross() {
        let v1 = Vec3::new(1.0, 2.0, 4.0);
        let v2 = Vec3::new(5.0, 6.0, 7.0);
        let answer = Vec3::new(-10.0, 13.0, -4.0);
        assert_eq!(v1.cross(&v2), answer);
    }
}