use super::vec3;
use super::ray;

pub struct Camera {
    origin: vec3::Vec3,
    lower_left_corner: vec3::Vec3,
    horizontal: vec3::Vec3,
    vertical: vec3::Vec3,
}
impl Camera {
    pub fn new() -> Camera {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = vec3::Vec3::new_empty();
        let horizontal = vec3::Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = vec3::Vec3::new(0.0, viewport_height, 0.0);
        Camera {
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: origin
                .subtract(&horizontal.divide_by(2.0))
                .subtract(&vertical.divide_by(2.0))
                .subtract(&vec3::Vec3::new(0.0, 0.0, focal_length)),
        }
    }
    pub fn get_ray(&self, u: f64, v: f64) -> ray::Ray {
        ray::Ray::new(self.origin, self.lower_left_corner
            .add(&self.horizontal.multiply_by(u))
            .add(&self.vertical.multiply_by(v))
            .subtract(&self.origin)
        )
    }
}