/*use std::rc::Rc;
use std::vec::Vec;
use crate::Hittable;
use crate::AABB;
use crate::random_i32;

#[derive(Clone)]
pub struct BvhNode {
    pub left:Rc<Hittable>,
    pub right:Rc<Hittable>,
    pub boxb: AABB,
}
impl BvhNode {
    fn new(&mut self, src_objects: & mut Vec<Rc<dyn Hittable>>, start: u64, end: u64, time0: f64, time1: f64) -> Self {
        let objects: Vec<Rc<dyn Hittable>> = src_objects.clone();
        let axis: i32 = random_i32(0, 2);
        let comparator = if axis == 0 {box_x_compare}
                         else if axis == 1 {box_y_compare}
                         else {box_z_compare};

        let object_span:  i64 = end - start;
        if object_span == 1 {
            left = objects[start].clone();
            right = objects[start].clone();
        }
        else if object_span ==2 {
            if comparator(objects[start].clone(), objects[start+1].clone()) {
                left = obejects[start].clone();
                right = objects[start+1].clone();
            }
            else {
                left = objects[start+1].clone();
                right = objects[start].clone();
            }
        }
        else {
            objects.sort_by()

        }
    }
}

impl Hittable for BvhNode {
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        output_box = self.boxb;
        true
    }
    fn hit(&self, r: Ray, t_min: &f64, t_max: &f64, rec: &mut HitRecord) -> bool {
        if !self.boxb.hit(r, t_min, t_max) {return false;}
        let hit_left = self.left.hit(r, t_min, t_max, rec);
        let hit_right = self.right.hit(r, t_min, if hit_left {rec.t} else {t_max}, rec);
        hit_left||hit_right
    }
}
pub fn box_compare(a: Rc<Hittable>, b: Rc<Hittable>, axis: i32) -> bool {
    let mut box_a: AABB = AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
    let mut box_b: AABB = AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
    if !a.bounding_box(0.0, 0.0, box_a) || !b.bounding_box(0.0, 0.0, box_b) {
        println("No bounding box in bvh_node constructor");
        return box_a.minimum.getcoordinate(axis) < box_b.minimum.getcoordinate(axis);
    }
}
pub fn box_x_compare(a: Rc<Hittable>, b: Rc<Hittable>) -> bool {
    box_compare(a, b, 0)
}
pub fn box_y_compare(a: Rc<Hittable>, b: Rc<Hittable>) -> bool {
    box_compare(a, b, 1)
}
pub fn box_z_compare(a: Rc<Hittable>, b: Rc<Hittable>) -> bool {
    box_compare(a, b, 2)
}
*/