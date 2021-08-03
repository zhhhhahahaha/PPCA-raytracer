use crate::aabb::surrounding_box;
use crate::materialfile::Material;
use crate::HitRecord;
use crate::Hittable;
use crate::Ray;
use crate::Vec3;
use crate::AABB;

#[derive(Clone)]
pub struct MovingSphere<T: Material> {
    pub center0: Vec3,
    pub center1: Vec3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub mat_ptr: T,
}
impl<T: Material> MovingSphere<T> {
    pub fn new(center0: Vec3, center1: Vec3, time0: f64, time1: f64, radius: f64, m: T) -> Self {
        Self {
            center0,
            center1,
            time0,
            time1,
            radius,
            mat_ptr: m,
        }
    }
    pub fn center(&self, time: f64) -> Vec3 {
        self.center0
            + (self.center1 - self.center0) * (time - self.time0) / (self.time1 - self.time0)
    }
}

impl<T: Material> Hittable for MovingSphere<T> {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vec3 = r.orig - self.center(r.tm);
        let a: f64 = r.dir.squared_length();
        let half_b: f64 = oc * r.dir;
        let c: f64 = oc.squared_length() - self.radius * self.radius;

        let discriminant: f64 = f64::powf(half_b, 2.0) - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd: f64 = discriminant.sqrt();
        let mut root: f64 = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }
        let mut rec = HitRecord {
            t: root,
            p: r.at(root),
            mat_ptr: &self.mat_ptr,
            u: 0.0,
            v: 0.0,
            front_face: true,
            normal: Vec3::zero(),
        };
        let outward_normal: Vec3 = (rec.p - self.center(r.tm)) / self.radius;
        rec.set_face_normal(r, outward_normal);
        Some(rec)
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        let box0: AABB = AABB::new(
            self.center(time0) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(time0) + Vec3::new(self.radius, self.radius, self.radius),
        );
        let box1: AABB = AABB::new(
            self.center(time1) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(time1) + Vec3::new(self.radius, self.radius, self.radius),
        );
        *output_box = surrounding_box(box0, box1);
        true
    }
}
