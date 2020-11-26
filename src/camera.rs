use crate::ray::Ray;
use crate::vec3::{Point, Vec3};

pub struct Camera {
    origin: Point,
    upper_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(vertical_fov: f64, aspect_ratio: f64) -> Camera {
        // Calculate viewport width and height using vertical
        let theta = vertical_fov * 180./std::f64::consts::PI;
        let h = (theta/2.).tan().abs();
        let viewport_height = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;

        let focal_length = 1.;
        let origin = Point::zeros();
        let horizontal = Vec3::new(viewport_width, 0., 0.);
        let vertical = Vec3::new(0., viewport_height, 0.);
        let upper_left_corner = origin - horizontal/2. + vertical/2. - Vec3::new(0., 0., focal_length);
        Camera{origin, horizontal, vertical, upper_left_corner}
    }
    
    pub fn get_ray(&self, x: f64, y: f64) -> Ray {
        Ray::new(self.origin, self.upper_left_corner + self.horizontal*x - self.vertical*y)
    }

    pub fn shift(&mut self, movement: &Vec3) {
        self.origin += *movement;
    }
}

