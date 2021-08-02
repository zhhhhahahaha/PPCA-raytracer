use crate::aabb::surrounding_box;
use crate::rtweekend::random_i32;
use crate::HitRecord;
use crate::Hittable;
use crate::Metal;
use crate::Ray;
use crate::Vec3;
use crate::AABB;
use std::vec::Vec;
use std::boxed::Box;

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
    pub fn new() -> Self {
        let objects: Vec<Box<dyn Hittable>> = Vec::new();
        Self { objects }
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_anything: bool = false;
        let mut closest_so_far: f64 = t_max;
        let mut rec = self.objects[0].hit(r, t_min, t_max).unwrap();
        for object in &self.objects {
            if let None = object.hit(r, t_min, closest_so_far) {
                continue;
            }
            else{    
                let mut temp_rec = object.hit(r, t_min, closest_so_far).unwrap();
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec = temp_rec.clone();
            }
        }
        if hit_anything {
            return Some(rec);
        }
        else {None}
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