use std::rc::Rc;
use std::vec::Vec;
use std::cmp::Ordering;
use crate::Hittable;
use crate::AABB;
use crate::aabb::surrounding_box;
use crate::rtweekend::random_i32;
use crate::Vec3;
use crate::Ray;
use crate::HitRecord;

#[derive(Clone)]
pub struct BvhNode {
    pub left:Rc<dyn Hittable>,
    pub right:Rc<dyn Hittable>,
    pub boxb: AABB,
}
impl BvhNode {
    pub fn new(src_objects: & Vec<Rc<dyn Hittable>>, start: usize, end: usize, time0: f64, time1: f64) -> Self {
        let mut objects: Vec<Rc<dyn Hittable>> = src_objects.clone();
        let axis: i32 = random_i32(0, 2);
        let comparator = if axis == 0 {box_x_compare}
                         else if axis == 1 {box_y_compare}
                         else {box_z_compare};

        let object_span = end - start;
        let mut left:Rc<dyn Hittable> = objects[0].clone();
        let mut right:Rc<dyn Hittable> = objects[0].clone();
        if object_span == 1 {
            left = objects[start].clone();
            right = objects[start].clone();
        }
        else if object_span ==2 {
            if comparator(objects[start].clone(), objects[start+1].clone())==Ordering::Less {
               left = objects[start].clone();
               right = objects[start+1].clone();
            }
            else {
               left = objects[start+1].clone();
               right = objects[start].clone();
            }
        }
        else {
            let mut change:Vec<Rc<dyn Hittable>> = Vec::new();
            for i in start..end {
                 change.push(objects[i].clone());
            }
            change.sort_by(|a,b| comparator(a.clone(), b.clone()));
            for i in start..end {
                objects[i] = change[i - start].clone();
            }
            let mid = start + object_span/2;
            left = Rc::new(BvhNode::new(&objects.clone(), start, mid, time0, time1));
            right = Rc::new(BvhNode::new(&objects.clone(), mid, end, time0, time1));
        }
        let mut box_left = AABB::new(Vec3::zero(), Vec3::zero());
        let mut box_right = AABB::new(Vec3::zero(), Vec3::zero());
        if !left.bounding_box(time0, time1, &mut box_left)||!right.bounding_box(time0, time1, &mut box_right) {
            print!("No bounding box in bvh_node constructor");
        }
        let boxb = surrounding_box(box_left, box_right);
        Self{left,right,boxb}
    }
}

impl Hittable for BvhNode {
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.boxb;
        true
    }
    fn hit(&self, r: Ray, t_min: &f64, t_max: &f64, rec: &mut HitRecord) -> bool {
        if !self.boxb.hit(r, *t_min, *t_max) {return false;}
        let hit_left = self.left.hit(r, t_min, t_max, rec);
        let t = rec.t;
        let hit_right = self.right.hit(r, t_min, if hit_left {&t} else {t_max}, rec);
        hit_left||hit_right
    }
}
pub fn box_compare(a: Rc<dyn Hittable>, b: Rc<dyn Hittable>, axis: i32) -> Ordering {
    let mut box_a: AABB = AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
    let mut box_b: AABB = AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
    if !a.bounding_box(0.0, 0.0, & mut box_a) || !b.bounding_box(0.0, 0.0, &mut box_b) {
        println!("No bounding box in bvh_node constructor");
    }
    if box_a.minimum.getcoordinate(axis) < box_b.minimum.getcoordinate(axis) {
        return Ordering::Less;
    }
    if box_a.minimum.getcoordinate(axis) == box_b.minimum.getcoordinate(axis) {
        return Ordering::Equal;
    }
    else {
        return Ordering::Greater;
    }
}
pub fn box_x_compare(a: Rc<dyn Hittable>, b: Rc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 0)
}
pub fn box_y_compare(a: Rc<dyn Hittable>, b: Rc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 1)
}
pub fn box_z_compare(a: Rc<dyn Hittable>, b: Rc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 2)
}
