use std::vec::Vec;
use std::rc::Rc;
use crate::Ray;
use crate::hittable;
use crate::hit_record;
use crate::Vec3;
use crate::metal;
#[derive(Clone)]
pub struct hittable_list {
    pub objects: Vec<Rc<dyn hittable>>,
}

impl hittable_list {
    pub fn add(&mut self, object: Rc<dyn hittable>) {
        self.objects.push(object);
    }
    pub fn new() -> Self {
        let mut objects: Vec<Rc<dyn hittable>> = Vec::new();
        Self { objects }
    }
}

impl hittable for hittable_list {
    fn hit(&self, r:Ray, t_min:&f64, t_max:&f64, rec:&mut hit_record) -> bool{
        let mut temp_rec = hit_record {p:Vec3::new(0.0, 0.0, 0.0), normal:Vec3::new(0.0, 0.0, 0.0), mat_ptr:Rc::new(metal::new(&Vec3::new(0.0, 0.0, 0.0), 0.5)), t:0.0, front_face:false,};
        let mut hit_anything: bool = false;
        let mut closest_so_far: f64 = *t_max;
        for object in &self.objects {
            if object.hit(r, &t_min, &closest_so_far, &mut temp_rec) {
                hit_anything = true;
                 closest_so_far = temp_rec.t.clone();
                *rec = temp_rec.clone();
            }
        }
        hit_anything
    }
}