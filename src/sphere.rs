use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::material::Material;
use crate::vec3::{Point,Vec3};

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub material: Box<dyn Material + Send + Sync>
}

impl Sphere {
    pub fn new(x: f64, y: f64, z: f64, r: f64, m: Box<dyn Material + Send + Sync>) -> Sphere {
        Sphere{center: Point::new(x, y, z), radius: r, material: m} 
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vec3 = r.origin - self.center;
        let a = r.dir.length_squared();
        let half_b = oc.dot(&r.dir);
        let c = oc.length_squared() - self.radius*self.radius;
        let discriminant = half_b*half_b - a*c;
        if discriminant < 0. {
            None
        } else {
            let sqrt_d = discriminant.sqrt();

            let root = (-half_b - sqrt_d) / a;
            if root < t_min || root > t_max {
                let root = (-half_b + sqrt_d) / a;
                if root < t_min || root > t_max {
                    return None
                }
            }
            let t = root;
            let p = r.at(t);
            let normal = (p - self.center) / self.radius;
            let front_face = false; // placeholder value
            let mut rec = HitRecord{p, normal, t, front_face, material: &*self.material};
            let outward_normal = (rec.p - self.center) / self.radius;
            rec.set_face_normal(r, &outward_normal);
            Some(rec)
        }
    }
}
