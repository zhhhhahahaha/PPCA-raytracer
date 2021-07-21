use crate::Vec3;
use crate::HittableList;
use std::rc::Rc;
use crate::Material;
use crate::Hittable;
use crate::AABB;
use crate::{XYRect,XZRect,YZRect};

#[derive(Clone)]
pub struct Box {
    box_min: Vec3,
    box_max: Vec3,
    sides: HittableList,
}
impl Box {
    pub fn new(p0: Vec3, p1: Vec3, ptr: Rc<dyn Material>) -> Self {
        let mut sides = HittableList::new();
        sides.add(Rc::new(XYRect::new(p0.x, p1.x, p0.y, p1.y, p1.z, ptr.clone())));
        sides.add(Rc::new(XYRect::new(p0.x, p1.x, p0.y, p1.y, p0.z, ptr.clone())));

        sides.add(Rc::new(XZRect::new(p0.x, p1.x, p0.z, p1.z, p1.y, ptr.clone())));
        sides.add(Rc::new(XZRect::new(p0.x, p1.x, p0.z, p1.z, p0.y, ptr.clone())));

        sides.add(Rc::new(YZRect::new(p0.y, p1.y, p0.z, p1.z, p1.x, ptr.clone())));
        sides.add(Rc::new(YZRect::new(p0.y, p1.y, p0.z, p1.z, p0.x, ptr.clone())));
        Self{
            box_min: p0,
            box_max: p1,
            sides,
        }
    }
}
impl Hittable for Box {
    fn hit(&self, r: crate::ray::Ray, t_min: &f64, t_max: &f64, rec: &mut crate::hittablefile::HitRecord) -> bool {
        self.sides.hit(r, &t_min, &t_max, rec)
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut crate::aabb::AABB) -> bool {
        *output_box = AABB::new(self.box_min, self.box_max);
        true
    }
}
