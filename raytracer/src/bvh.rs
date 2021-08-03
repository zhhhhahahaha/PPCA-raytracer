use crate::aabb::surrounding_box;
use crate::rtweekend::random_i32;
use crate::HitRecord;
use crate::Hittable;
use crate::Ray;
use crate::Vec3;
use crate::AABB;
use std::cmp::Ordering;
use std::boxed::Box;
use std::vec::Vec;

pub struct BvhNode {
    pub left: Option<Box<dyn Hittable>>,
    pub right: Option<Box<dyn Hittable>>,
    pub boxb: AABB,
}
impl BvhNode {
    pub fn new(
        mut objects: Vec<Box<dyn Hittable>>,
        time0: f64,
        time1: f64,
    ) -> Self {
        let axis: i32 = random_i32(0, 2);
        let comparator = if axis == 0 {
            box_x_compare
        } else if axis == 1 {
            box_y_compare
        } else {
            box_z_compare
        };
        let object_span = objects.len();
        let mut left:Option<Box<dyn Hittable>> = None;
        let mut right:Option<Box<dyn Hittable>> = None;
        if object_span == 1 {
            left = objects.pop();
        } else if object_span == 2 {
            if comparator(&objects[0], &objects[1]) == Ordering::Less {
                right = objects.pop();
                left = objects.pop();
            } else {
                left = objects.pop();
                right = objects.pop();
            }
        } else {
            objects.sort_by(|a, b| comparator(a, b));
            let mid = object_span / 2;
            let a = objects.split_off(mid);
            left = Some(Box::new(BvhNode::new(objects, time0, time1)));
            right = Some(Box::new(BvhNode::new(a, time0, time1)));
        }
        let box_left = AABB::new(Vec3::zero(), Vec3::zero());
        let box_right = AABB::new(Vec3::zero(), Vec3::zero());
            /*if !left.bounding_box(time0, time1, &mut box_left)
                || !right.bounding_box(time0, time1, &mut box_right)
            {
                print!("No bounding box in bvh_node constructor");
            }*/
            let boxb = surrounding_box(box_left, box_right);
            Self { left, right, boxb }
    }
}

impl Hittable for BvhNode {
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.boxb;
        true
    }
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.boxb.hit(r, t_min, t_max) {
            return None;
        }
        let mut hit_left: Option<HitRecord> = None;
        let mut hit_right: Option<HitRecord> = None;
        let left = self.left.as_ref().unwrap();
        let right = self.right.as_ref().unwrap();
        if let None = left.hit(r, t_min, t_max) {
           hit_right = right.hit(r, t_min, t_max);
           return hit_right;
        }
        else {
            hit_left = left.hit(r, t_min, t_max);
            let tem = hit_left.unwrap();
            let t = tem.t;
            if let None = right.hit(r, t_min, t) {
                return Some(tem);
            }
            else {
                hit_right = right.hit(r, t_min, t);
                return hit_right;
            }
        }
    }
}
pub fn box_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>, axis: i32) -> Ordering {
    let mut box_a: AABB = AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
    let mut box_b: AABB = AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
    if !a.bounding_box(0.0, 0.0, &mut box_a) || !b.bounding_box(0.0, 0.0, &mut box_b) {
        println!("No bounding box in bvh_node constructor");
    }
    if box_a.minimum.getcoordinate(axis) < box_b.minimum.getcoordinate(axis) {
        return Ordering::Less;
    }
    if box_a.minimum.getcoordinate(axis) == box_b.minimum.getcoordinate(axis) {
        return Ordering::Equal;
    } else {
        return Ordering::Greater;
    }
}
pub fn box_x_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> Ordering {
    box_compare(a, b, 0)
}
pub fn box_y_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> Ordering {
    box_compare(a, b, 1)
}
pub fn box_z_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> Ordering {
    box_compare(a, b, 2)
}

