use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3, reflect, refract};
use rand::Rng;

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
    pub fuzz: f64
}

unsafe impl Send for Metal {}
unsafe impl Sync for Metal {}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal{albedo, fuzz}
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected: Vec3 = reflect(&r_in.dir.unit_vec(), &rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz*Vec3::random_in_unit_sphere());
        let attenuation = self.albedo;
        if scattered.dir.dot(&rec.normal) > 0. {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub ir: f64,
}

unsafe impl Send for Dielectric {}
unsafe impl Sync for Dielectric {}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let mut r0 = (1. - ref_idx) / (1. + ref_idx);
    r0 = r0 * r0;
    r0 + (1. - r0) * (1. - cosine).powf(5.)
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Color::new(1., 1., 1.);
        let refraction_ratio = if rec.front_face { 1./self.ir } else { self.ir };
        let unit_direction = r_in.dir.unit_vec();

        let cos_theta = (-unit_direction).dot(&rec.normal).min(1.);
        let sin_theta = (1. - cos_theta*cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let mut rng  = rand::thread_rng();
        let direction = if cannot_refract || reflectance(cos_theta, refraction_ratio) > rng.gen_range(0.0, 1.0) {
            reflect(&unit_direction, &rec.normal)
        } else {
            refract(&unit_direction, &rec.normal, refraction_ratio)
        };

        let scattered = Ray::new(rec.p, direction);
        Some((scattered, attenuation))
    }
}
