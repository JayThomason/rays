use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3, reflect};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

#[derive(Debug, Copy, Clone)]
pub struct Lambertian {
    pub albedo: Color,
}

unsafe impl Send for Lambertian {}
unsafe impl Sync for Lambertian {}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian{albedo}
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        let scattered = Ray::new(rec.p, scatter_direction);
        let attenuation = self.albedo;
        Some((scattered, attenuation))
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Metal {
    pub albedo: Color,
}

unsafe impl Send for Metal {}
unsafe impl Sync for Metal {}

impl Metal {
    pub fn new(albedo: Color) -> Metal {
        Metal{albedo}
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = reflect(&r_in.dir.unit_vec(), &rec.normal);
        let scattered = Ray::new(rec.p, reflected);
        let attenuation = self.albedo;
        if scattered.dir.dot(&rec.normal) > 0. {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}
