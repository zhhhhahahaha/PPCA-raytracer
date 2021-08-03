use crate::HitRecord;
use crate::Hittable;
use crate::HittableList;
use crate::Material;
use crate::Ray;
use crate::Vec3;
use crate::AABB;
use crate::{XYRect, XZRect, YZRect};
use std::boxed::Box;

pub struct RealBox {
    box_min: Vec3,
    box_max: Vec3,
    pub sides: HittableList,
}
impl<'a> RealBox {
    pub fn new<T: 'static + Material + Clone>(p0: Vec3, p1: Vec3, ptr: T) -> Self {
        let mut sides = HittableList::new();
        sides.add(Box::new(XYRect::new(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p1.z,
            ptr.clone(),
        )));
        sides.add(Box::new(XYRect::new(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p0.z,
            ptr.clone(),
        )));

        sides.add(Box::new(XZRect::new(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p1.y,
            ptr.clone(),
        )));
        sides.add(Box::new(XZRect::new(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p0.y,
            ptr.clone(),
        )));

        sides.add(Box::new(YZRect::new(
            p0.y,
            p1.y,
            p0.z,
            p1.z,
            p1.x,
            ptr.clone(),
        )));
        sides.add(Box::new(YZRect::new(
            p0.y,
            p1.y,
            p0.z,
            p1.z,
            p0.x,
            ptr.clone(),
        )));
        Self {
            box_min: p0,
            box_max: p1,
            sides,
        }
    }
}
impl Hittable for RealBox {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if let None = self.sides.hit(r, t_min, t_max) {
            return None;
        }
        let rec = self.sides.hit(r, t_min, t_max).unwrap();
        Some(rec)
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut crate::aabb::AABB) -> bool {
        *output_box = AABB::new(self.box_min, self.box_max);
        true
    }
}
