use crate::materialfile::material;
use crate::Ray;
use crate::Vec3;
use std::rc::Rc;

#[derive(Clone)]
pub struct hit_record {
    pub p: Vec3,
    pub normal: Vec3,
    pub mat_ptr: Rc<dyn material>,
    pub t: f64,
    pub front_face: bool,
}

impl hit_record {
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.front_face = r.dir * outward_normal < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
pub trait hittable {
    fn hit(&self, r: Ray, t_min: &f64, t_max: &f64, rec: &mut hit_record) -> bool;
}
