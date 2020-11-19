use crate::ray::Ray;
use crate::vec3::Point;
use crate::vec3::Vec3;
use crate::material::Material;

pub struct HitRecord<'a> {
   pub p: Point,
   pub normal: Vec3,
   pub t: f64,
   pub front_face: bool,
   pub material: &'a dyn Material,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl<'a> HitRecord<'a> {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.dir.dot(&outward_normal) < 0.;
        self.normal = if self.front_face { outward_normal.clone() } else { -outward_normal.clone() };
    }
}

pub struct HittableList {
    objects: std::vec::Vec<Box<dyn Hittable + Send + Sync>>
}

impl HittableList {
    pub fn new(objects: Vec<Box<dyn Hittable + Send  +Sync>>) -> HittableList {
        HittableList{objects}
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec: Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        for object in self.objects.iter() {
            if let Some(new_hit) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = new_hit.t;
                rec = Some(new_hit);
            }
        }
        return rec;
    }
}
