use crate::rtweekend::random_f64;
use crate::HitRecord;
use crate::Hittable;
use crate::Material;
use crate::Ray;
use crate::Vec3;
use crate::AABB;

#[derive(Clone)]
pub struct XYRect<T:Material> {
    pub mp: T,
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub k: f64,
}
impl<T:Material> XYRect<T> {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, mp: T) -> Self {
        Self {
            x0,
            x1,
            y0,
            y1,
            k,
            mp,
        }
    }
}

impl<T:Material> Hittable for XYRect<T> {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t: f64 = (self.k - r.orig.z) / r.dir.z;
        if t < t_min || t > t_max {
            return None;
        }
        let x: f64 = r.orig.x + t * r.dir.x;
        let y: f64 = r.orig.y + t * r.dir.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }
        let mut rec = HitRecord{
            u: (x - self.x0) / (self.x1 - self.x0),
            v: (y - self.y0) / (self.y1 - self.y0),
            t,
            normal: Vec3::zero(),
            mat_ptr: &self.mp,
            p: r.at(t),
            front_face: true,
        };
        rec.set_face_normal(r, Vec3::new(0.0,0.0,1.0));
        Some(rec)
        
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            Vec3::new(self.x0, self.y0, self.k - 0.0001),
            Vec3::new(self.x1, self.y1, self.k + 0.0001),
        );
        true
    }
}

#[derive(Clone)]
pub struct XZRect<T:Material> {
    pub mp: T,
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
}
impl<T:Material> XZRect<T> {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, mat: T) -> Self {
        Self {
            x0,
            x1,
            z0,
            z1,
            k,
            mp: mat,
        }
    }
}
impl<T:Material> Hittable for XZRect<T> {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t: f64 = (self.k - r.orig.y) / r.dir.y;
        if t < t_min || t > t_max {
            return None;
        }
        let x: f64 = r.orig.x + t * r.dir.x;
        let z: f64 = r.orig.z + t * r.dir.z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let mut rec = HitRecord{
            u:(x - self.x0) / (self.x1 - self.x0),
            v:(z - self.z0) / (self.z1 - self.z0),
            t,
            mat_ptr:&self.mp,
            p:r.at(t),
            front_face:true,
            normal:Vec3::zero(),
        };
        rec.set_face_normal(r, Vec3::new(0.0, 1.0, 0.0));
        Some(rec)
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            Vec3::new(self.x0, self.k - 0.0001, self.z0),
            Vec3::new(self.x1, self.k + 0.0001, self.z1),
        );
        true
    }
    fn pdf_value(&self, o: Vec3, v: Vec3) -> f64 {
        if let None = self.hit(Ray::new(o, v, 0.0), 0.001, f64::INFINITY) {
            return 0.0;
        }
        let rec =  self.hit(Ray::new(o, v, 0.0), 0.001, f64::INFINITY).unwrap();
        let area = (self.x1 - self.x0) * (self.z1 - self.z0);
        let distance_squared = rec.t * rec.t * v.squared_length();
        let cosine = (v * rec.normal / v.length()).abs();
        distance_squared / (cosine * area)
    }
    fn random(&self, o: Vec3) -> Vec3 {
        let random_point = Vec3::new(
            random_f64(self.x0, self.x1),
            self.k,
            random_f64(self.z0, self.z1),
        );
        random_point - o
    }
}

#[derive(Clone)]
pub struct YZRect<T> {
    pub mp: T,
    pub y0: f64,
    pub y1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
}
impl<T:Material> YZRect<T> {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, mat: T) -> Self {
        Self {
            y0,
            y1,
            z0,
            z1,
            k,
            mp: mat,
        }
    }
}
impl<T:Material> Hittable for YZRect<T> {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t: f64 = (self.k - r.orig.x) / r.dir.x;
        if t < t_min || t > t_max {
            return None;
        }
        let y: f64 = r.orig.y + t * r.dir.y;
        let z: f64 = r.orig.z + t * r.dir.z;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let mut rec = HitRecord{
            u:(y - self.y0) / (self.y1 - self.y0),
            v:(z - self.z0) / (self.z1 - self.z0),
            t, 
            mat_ptr:&self.mp,
            p:r.at(t),
            normal: Vec3::zero(),
            front_face: true
        };
        rec.set_face_normal(r, Vec3::new(1.0, 0.0, 0.0));
        Some(rec)
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            Vec3::new(self.k - 0.0001, self.y0, self.z0),
            Vec3::new(self.k + 0.0001, self.y1, self.z1),
        );
        true
    }
}
