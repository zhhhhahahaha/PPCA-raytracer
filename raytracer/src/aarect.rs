use crate::Hittable;
use crate::Material;
use crate::Ray;
use crate::Vec3;
use crate::HitRecord;
use crate::AABB;
use crate::rtweekend::random_f64;
use std::rc::Rc;
use crate::Lambertian;

#[derive(Clone)]
pub struct XYRect {
    pub mp: Rc<dyn Material>,
    pub x0:f64,
    pub x1:f64,
    pub y0:f64,
    pub y1:f64,
    pub k:f64,
}
impl XYRect {
    pub fn new(x0:f64, x1:f64, y0:f64, y1:f64, k:f64, mp:Rc<dyn Material>) -> Self {
        Self{x0, x1, y0, y1, k, mp}
    }
}

impl Hittable for XYRect {
    fn hit(&self, r: Ray, t_min: &f64, t_max: &f64, rec: &mut HitRecord) -> bool {
        let t:f64 = (self.k - r.orig.z) / r.dir.z;
        if t < *t_min || t > *t_max {
            return false;
        }
        let x:f64 = r.orig.x + t * r.dir.x;
        let y:f64 = r.orig.y + t * r.dir.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return false;
        }
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (y - self.y0) / (self.y1 - self.y0);
        rec.t = t;
        let outward_normal: Vec3 = Vec3::new(0.0, 0.0, 1.0);
        rec.set_face_normal(r, outward_normal);
        rec.mat_ptr = self.mp.clone();
        rec.p = r.at(t);
        true
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(Vec3::new(self.x0, self.y0, self.k - 0.0001), Vec3::new(self.x1, self.y1, self.k + 0.0001));
        true
    }
}

#[derive(Clone)]
pub struct XZRect {
    pub mp: Rc<dyn Material>,
    pub x0:f64,
    pub x1:f64,
    pub z0:f64,
    pub z1:f64,
    pub k:f64,
}
impl XZRect {
    pub fn new(x0:f64, x1:f64, z0:f64, z1:f64, k:f64, mat:Rc<dyn Material>) -> Self {
        Self {x0, x1, z0, z1, k, mp:mat}
    }
}
impl Hittable for XZRect {
    fn hit(&self, r: Ray, t_min: &f64, t_max: &f64, rec: &mut HitRecord) -> bool {
        let t:f64 = (self.k - r.orig.y) / r.dir.y;
        if t < *t_min || t > *t_max {
            return false;
        }
        let x:f64 = r.orig.x + t * r.dir.x;
        let z:f64 = r.orig.z + t * r.dir.z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return false;
        }
        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        let outward_normal: Vec3 = Vec3::new(0.0, 1.0, 0.0);
        rec.set_face_normal(r, outward_normal);
        rec.mat_ptr = self.mp.clone();
        rec.p = r.at(t);
        true
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(Vec3::new(self.x0, self.k - 0.0001, self.z0), Vec3::new(self.x1, self.k + 0.0001, self.z1));
        true
    }
    fn pdf_value(&self, o: Vec3, v: Vec3) -> f64 {
        let mut rec = HitRecord {
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            mat_ptr: Rc::new(Lambertian::new2(&Vec3::new(0.0, 0.0, 0.0))),
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: false,
        };
        if !self.hit(Ray::new(o, v, 0.0), &0.001, &f64::INFINITY, &mut rec) {
            return 0.0;
        }
        let area = (self.x1 - self.x0) * (self.z1 - self.z0);
        let distance_squared = rec.t * rec.t * v.squared_length();
        let cosine = (v * rec.normal / v.length()).abs();
        distance_squared / (cosine * area)
    }
    fn random(&self, o: Vec3) -> Vec3 {
        let random_point = Vec3::new(random_f64(self.x0, self.x1), self.k, random_f64(self.z0, self.z1));
        random_point - o
    }
}


#[derive(Clone)]
pub struct YZRect {
    pub mp: Rc<dyn Material>,
    pub y0:f64,
    pub y1:f64,
    pub z0:f64,
    pub z1:f64,
    pub k:f64,
}
impl YZRect {
    pub fn new(y0:f64, y1:f64, z0:f64, z1:f64, k:f64, mat:Rc<dyn Material>) -> Self {
        Self{y0, y1, z0, z1, k, mp:mat}
    }
}
impl Hittable for YZRect {
    fn hit(&self, r: Ray, t_min: &f64, t_max: &f64, rec: &mut HitRecord) -> bool {
        let t:f64 = (self.k - r.orig.x) / r.dir.x;
        if t < *t_min || t > *t_max {
            return false;
        }
        let y:f64 = r.orig.y + t * r.dir.y;
        let z:f64 = r.orig.z + t * r.dir.z;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return false;
        }
        rec.u = (y - self.y0) / (self.y1 - self.y0);
        rec.v = (z - self.z0) / (self.z1 - self.z0);
        rec.t = t;
        let outward_normal: Vec3 = Vec3::new(1.0, 0.0, 0.0);
        rec.set_face_normal(r, outward_normal);
        rec.mat_ptr = self.mp.clone();
        rec.p = r.at(t);
        true
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(Vec3::new(self.k - 0.0001, self.y0, self.z0), Vec3::new(self.k + 0.0001, self.y1, self.z1));
        true
    }
}