use crate::hit_record;
use crate::hittable;
use crate::materialfile::material;
use crate::Ray;
use crate::Vec3;
use std::rc::Rc;

#[derive(Clone)]
pub struct sphere {
    pub center: Vec3,
    pub radius: f64,
    pub mat_ptr: Rc<dyn material>,
}
impl sphere {
    pub fn new(center: Vec3, radius: f64, m: Rc<dyn material>) -> Self {
        Self {
            center,
            radius,
            mat_ptr: m,
        }
    }
}

impl hittable for sphere {
    fn hit(&self, r: Ray, t_min: &f64, t_max: &f64, rec: &mut hit_record) -> bool {
        let oc: Vec3 = r.orig - self.center;
        let a: f64 = r.dir * r.dir;
        let half_b: f64 = r.dir * oc;
        let c: f64 = oc * oc - self.radius * self.radius;
        let discriminant: f64 = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        } else {
            let root: f64 = discriminant.sqrt();
            let t: f64 = (-half_b - root) / a;
            if t > *t_min && t < *t_max {
                rec.t = t;
                rec.p = r.at(t);
                let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, outward_normal);
                rec.mat_ptr = self.mat_ptr.clone();
                return true;
            }
            let t: f64 = (-half_b + root) / a;
            if t > *t_min && t < *t_max {
                rec.t = t;
                rec.p = r.at(t);
                let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, outward_normal);
                rec.mat_ptr = self.mat_ptr.clone();
                return true;
            }
        }
        return false;
    }
}
