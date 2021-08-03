use crate::materialfile::Material;
use crate::rtweekend::{degrees_to_radians, fmax, fmin};
use crate::Ray;
use crate::Vec3;
use crate::AABB;

#[derive(Clone)]
pub struct HitRecord<'a> {
    pub p: Vec3,
    pub normal: Vec3,
    pub mat_ptr: &'a dyn Material,
    pub t: f64,
    pub u: f64, //u和v应该是不用的时候默认是0
    pub v: f64, //
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.front_face = r.dir * outward_normal < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool;
    fn pdf_value(&self, o: Vec3, v: Vec3) -> f64 {
        0.0
    }
    fn random(&self, o: Vec3) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
}
#[derive(Clone)]
pub struct Translate<T: Hittable> {
    ptr: T,
    offset: Vec3,
}
impl<T: Hittable> Translate<T> {
    pub fn new(p: T, displacement: Vec3) -> Self {
        Self {
            ptr: p,
            offset: displacement,
        }
    }
}
impl<T: Hittable> Hittable for Translate<T> {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let move_r: Ray = Ray::new(r.orig - self.offset, r.dir, r.tm);
        if let None = self.ptr.hit(move_r, t_min, t_max) {
            return None;
        }
        let mut rec = self.ptr.hit(move_r, t_min, t_max).unwrap();
        rec.p += self.offset;
        rec.set_face_normal(move_r, rec.normal);
        Some(rec)
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        if !self.ptr.bounding_box(time0, time1, output_box) {
            return false;
        }
        *output_box = AABB::new(
            output_box.minimum + self.offset,
            output_box.maximum + self.offset,
        );
        true
    }
}
#[derive(Clone)]
pub struct Rotatey<T: Hittable> {
    ptr: T,
    sin_theta: f64,
    cos_theta: f64,
    hasbox: bool,
    bbox: AABB,
}
impl<T: Hittable> Rotatey<T> {
    pub fn new(p: T, angle: f64) -> Self {
        let radians: f64 = degrees_to_radians(angle);
        let sin_theta = f64::sin(radians);
        let cos_theta = f64::cos(radians);
        let mut bbox = AABB::new(Vec3::zero(), Vec3::zero());
        let hasbox = p.bounding_box(0.0, 1.0, &mut bbox);

        let mut min: Vec3 = Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max: Vec3 = Vec3::new(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.maximum.x + (1.0 - i as f64) * bbox.minimum.x;
                    let y = j as f64 * bbox.maximum.y + (1.0 - j as f64) * bbox.minimum.y;
                    let z = k as f64 * bbox.maximum.z + (1.0 - k as f64) * bbox.minimum.z;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);
                    for c in 0..3 {
                        if c == 0 {
                            min.x = fmin(min.getcoordinate(c), tester.getcoordinate(c));
                            max.x = fmax(max.getcoordinate(c), tester.getcoordinate(c));
                        }
                        if c == 1 {
                            min.y = fmin(min.getcoordinate(c), tester.getcoordinate(c));
                            max.y = fmax(max.getcoordinate(c), tester.getcoordinate(c));
                        }
                        if c == 2 {
                            min.z = fmin(min.getcoordinate(c), tester.getcoordinate(c));
                            max.z = fmax(max.getcoordinate(c), tester.getcoordinate(c));
                        }
                    }
                }
            }
        }
        bbox = AABB::new(min, max);
        Self {
            ptr: p,
            sin_theta,
            cos_theta,
            hasbox,
            bbox,
        }
    }
}
impl<T: Hittable> Hittable for Rotatey<T> {
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.bbox;
        self.hasbox
    }
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = r.orig;
        let mut direction = r.dir;
        origin.x = self.cos_theta * r.orig.x - self.sin_theta * r.orig.z;
        origin.z = self.sin_theta * r.orig.x + self.cos_theta * r.orig.z;
        direction.x = self.cos_theta * r.dir.x - self.sin_theta * r.dir.z;
        direction.z = self.sin_theta * r.dir.x + self.cos_theta * r.dir.z;
        let rotated_r = Ray::new(origin, direction, r.tm);
        if let None = self.ptr.hit(rotated_r, t_min, t_max) {
            return None;
        }
        let mut rec = self.ptr.hit(rotated_r, t_min, t_max).unwrap();
        let mut p = rec.p;
        let mut normal = rec.normal;
        p.x = self.cos_theta * rec.p.x + self.sin_theta * rec.p.z;
        p.z = -self.sin_theta * rec.p.x + self.cos_theta * rec.p.z;
        normal.x = self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.z;
        normal.z = -self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.z;
        rec.p = p;
        rec.set_face_normal(rotated_r, normal);
        Some(rec)
    }
}

#[derive(Clone)]
pub struct FlipFace<T: Hittable> {
    pub ptr: T,
}
impl<T: Hittable> FlipFace<T> {
    pub fn new(p: T) -> Self {
        Self { ptr: p }
    }
}
impl<T: Hittable> Hittable for FlipFace<T> {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if let None = self.ptr.hit(r, t_min, t_max) {
            return None;
        }
        let mut rec = self.ptr.hit(r, t_min, t_max).unwrap();
        rec.front_face = !rec.front_face;
        Some(rec)
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        self.ptr.bounding_box(time0, time1, output_box)
    }
}
