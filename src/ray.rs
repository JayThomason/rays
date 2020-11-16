use crate::vec3::Point;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Ray {
    pub origin: Point,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point, dir: Vec3) -> Ray {
        Ray{origin, dir}
    }

    pub fn at(&self, t: f64) -> Point {
        self.origin + self.dir*t
    }
}

#[cfg(test)]
mod test {
    use super::*;

     #[test]
    fn test_at() {
        let origin = Point::zeros();
        let dir = Vec3::new(1., 2., 3.);
        let r = Ray::new(origin, dir);
        assert_eq!(r.at(1.0), Point::new(1., 2., 3.));
    }
}
