use crate::hittable::{Hittable, HitRecord};
use crate::ray::{Ray};
use crate::interval::Interval;
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: Vec::new() }
    }

    pub fn new_with(object: Box<dyn Hittable>) -> HittableList {
        let mut list = HittableList::new();
        list.add(object);
        list
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) -> () {
        self.objects.push(object);
    }


}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut closest_so_far = ray_t.max;
        let mut result_hit_record: Option<HitRecord> = None;

        for object in self.objects.iter() {
            match object.hit(r, Interval::new(ray_t.min, closest_so_far)) {
                None => {},
                Some(hitrecord) => {
                    closest_so_far = hitrecord.t;
                    result_hit_record = Some(hitrecord);
                }
            }
        }

        return result_hit_record;
    }
}