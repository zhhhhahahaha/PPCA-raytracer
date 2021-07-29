use crate::aabb::surrounding_box;
use crate::rtweekend::random_i32;
use crate::HitRecord;
use crate::Hittable;
use crate::Metal;
use crate::Ray;
use crate::Vec3;
use crate::AABB;
use std::sync::Arc;
use std::vec::Vec;


#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
    pub fn new() -> Self {
        let objects: Vec<Arc<dyn Hittable>> = Vec::new();
        Self { objects }
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: Ray, t_min: &f64, t_max: &f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord {
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            mat_ptr: Arc::new(Metal::new(Vec3::new(0.0, 0.0, 0.0), 0.5)),
            t: 0.0,
            u: 0.0,
            v: 0.0,
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
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut crate::aabb::AABB) -> bool {
        if self.objects.is_empty() {
            return false;
        }
        let mut temp_box: AABB = AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
        let mut first_box: bool = true;
        for object in &self.objects {
            if !object.bounding_box(time0, time1, &mut temp_box) {
                return false;
            }
            *output_box = if first_box {
                temp_box
            } else {
                surrounding_box(*output_box, temp_box)
            };
            first_box = false;
        }
        true
    }
    fn pdf_value(&self, o: Vec3, v: Vec3) -> f64 {
        let weight = 1.0 / (self.objects.len() as f64);
        let mut sum = 0.0;
        for object in &self.objects {
            sum += weight * object.pdf_value(o, v);
        }
        sum
    }
    fn random(&self, o: Vec3) -> Vec3 {
        let int_size = self.objects.len() as i32;
        self.objects[random_i32(0, int_size - 1) as usize].random(o)
    }
}
unsafe impl Sync for HittableList{}
unsafe impl Send for HittableList{}