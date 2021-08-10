pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min
    } else if x > max{
        return max
    }
    x
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_degrees_to_radians() {
        assert_eq!(degrees_to_radians(90.0), std::f64::consts::FRAC_PI_2);
    }
    #[test]
    pub fn test_clamp() {
        assert_eq!(clamp(0.5, 0.0, 1.0), 0.5);
        assert_eq!(clamp(-1.0, 0.0, 1.0), 0.0);
        assert_eq!(clamp(1.1, 0.0, 1.0), 1.0);
    }
}
