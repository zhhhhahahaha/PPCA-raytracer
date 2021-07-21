use crate::materialfile::Material;
use crate::HitRecord;
use crate::Hittable;
use crate::Ray;
use crate::Vec3;
use crate::AABB;
use std::f64::consts::PI;
use std::rc::Rc;

#[derive(Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub mat_ptr: Rc<dyn Material>,
}
impl Sphere {
    pub fn new(center: Vec3, radius: f64, m: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            mat_ptr: m,
        }
    }
    fn get_sphere_uv(p: Vec3, u: &mut f64, v: &mut f64) {
        let theta: f64 = -p.y.acos();
        let phi: f64 = f64::atan2(-p.z, p.x) + PI;
        *u = phi / (2.0 * PI);
        *v = theta / PI;
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: &f64, t_max: &f64, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = r.orig - self.center;
        let a: f64 = r.dir * r.dir;
        let half_b: f64 = r.dir * oc;
        let c: f64 = oc * oc - self.radius * self.radius;
        let discriminant: f64 = f64::powf(half_b, 2.0) - a * c;
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
                Sphere::get_sphere_uv(outward_normal, &mut rec.u, &mut rec.v);
                rec.mat_ptr = self.mat_ptr.clone();
                return true;
            }
            let t: f64 = (-half_b + root) / a;
            if t > *t_min && t < *t_max {
                rec.t = t;
                rec.p = r.at(t);
                let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, outward_normal);
                Sphere::get_sphere_uv(outward_normal, &mut rec.u, &mut rec.v);
                rec.mat_ptr = self.mat_ptr.clone();
                return true;
            }
        }
        false
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        );
        true
    }
}
