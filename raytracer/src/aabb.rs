use crate::rtweekend::fmax;
use crate::rtweekend::fmin;
use crate::Ray;
use crate::Vec3;

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct AABB {
    pub minimum: Vec3,
    pub maximum: Vec3,
}
impl AABB {
    pub fn new(minimum: Vec3, maximum: Vec3) -> Self {
        Self { minimum, maximum }
    }
    pub fn hit(&self, r: Ray, mut t_min: f64, mut t_max: f64) -> bool {
        for a in 0..3 {
            let t0: f64 = fmin(
                (self.minimum.getcoordinate(a) - r.orig.getcoordinate(a)) / r.dir.getcoordinate(a),
                (self.maximum.getcoordinate(a) - r.orig.getcoordinate(a)) / r.dir.getcoordinate(a),
            );
            let t1: f64 = fmax(
                (self.minimum.getcoordinate(a) - r.orig.getcoordinate(a)) / r.dir.getcoordinate(a),
                (self.maximum.getcoordinate(a) - r.orig.getcoordinate(a)) / r.dir.getcoordinate(a),
            );
            t_min = fmax(t0, t_min);
            t_max = fmin(t1, t_max);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }
}
pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
    let small: Vec3 = Vec3::new(
        fmin(box0.minimum.x, box1.minimum.x),
        fmin(box0.minimum.y, box1.minimum.y),
        fmin(box0.minimum.z, box1.minimum.z),
    );
    let big: Vec3 = Vec3::new(
        fmax(box0.maximum.x, box1.maximum.x),
        fmax(box0.maximum.y, box1.maximum.y),
        fmax(box0.maximum.z, box1.maximum.z),
    );
    AABB::new(small, big)
}
