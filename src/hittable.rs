use crate::interval::Interval;
use crate::material::Material;
use crate::vec3::{Point3, Vec3};
use crate::ray::{Ray};
use std::sync::Arc;


pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Arc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, normal: Vec3, t: f64, mat: Arc<dyn Material>, front_face: bool) -> HitRecord {
        HitRecord { p, normal, t, mat, front_face }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) -> () {
        self.front_face = Vec3::dot(r.direction(), *outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { -*outward_normal };
    }
}

pub trait Hittable { 
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}