use crate::HitRecord;
use crate::Hittable;
use crate::Metal;
use crate::Ray;
use crate::Vec3;
use std::rc::Rc;
use std::vec::Vec;
#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
    pub fn new() -> Self {
        let objects: Vec<Rc<dyn Hittable>> = Vec::new();
        Self { objects }
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: Ray, t_min: &f64, t_max: &f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord {
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            mat_ptr: Rc::new(Metal::new(&Vec3::new(0.0, 0.0, 0.0), 0.5)),
            t: 0.0,
            front_face: false,
        };
        let mut hit_anything: bool = false;
        let mut closest_so_far: f64 = *t_max;
        for object in &self.objects {
            if object.hit(r, &t_min, &closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
        hit_anything
    }
}
