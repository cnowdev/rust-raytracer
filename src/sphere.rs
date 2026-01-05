use crate::hittable::{Hittable, HitRecord};
use crate::interval::Interval;
use crate::vec3::{Point3, Vec3};
use crate::ray::{Ray};
use crate::material::Material;
use std::sync::Arc;

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Arc<dyn Material>) -> Sphere {
        Sphere {center, radius, mat}
    }
}

impl Hittable for Sphere {

    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = self.center - r.origin();
        let a = r.direction().length_squared();
        let h = Vec3::dot(r.direction(), oc);
        let c = oc.length_squared() - self.radius*self.radius;

        let discriminant = h*h - a*c;

        if discriminant < 0.0 {
            // ray dosent hit the sphere
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;

        //if both roots are not in range, return None
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None
            }
        }

        let t = root;
        let p = r.at(t);
        let outward_normal = (p - self.center) / self.radius;

        let mut rec = HitRecord::new(p, outward_normal, t, self.mat.clone(), false);
        rec.set_face_normal(r, &outward_normal);

        return Some(rec);


    }
}