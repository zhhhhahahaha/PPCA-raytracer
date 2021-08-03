use crate::rtweekend::random_f64;
use crate::HitRecord;
use crate::Hittable;
use crate::Isotropic;
use crate::Ray;
use crate::Texture;
use crate::Vec3;
use crate::AABB;

#[derive(Clone)]
pub struct ConstantMedium<T: Hittable, U: Texture> {
    boundary: T,
    phase_function: Isotropic<U>,
    neg_inv_density: f64,
}
/* 
impl ConstantMedium<SolidColor, T:Hittable> {
    pub fn new2(b: T, d: f64, c: Vec3) -> Self {
        Self {
            boundary: b,
            neg_inv_density: -1.0 / d,
            phase_function: Isotropic::<SolidColor>::new1(c),
        }
    }
}
*/
impl <T:Hittable, U:Texture> ConstantMedium<T,U> {
    pub fn new1(b: T, d: f64, a: U) -> Self {
        Self {
            boundary: b,
            neg_inv_density: -1.0 / d,
            phase_function: Isotropic::<U>::new2(a),
        }
    }
}
impl<T: Hittable, U: Texture> Hittable for ConstantMedium<T, U> {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if let None = self.boundary.hit(r, -f64::INFINITY, f64::INFINITY) {
            return None;
        }
        let mut rec1 = self.boundary.hit(r, -f64::INFINITY, f64::INFINITY).unwrap();
        if let None = self.boundary.hit(r, rec1.t + 0.0001, f64::INFINITY) {
            return None;
        }
        let mut rec2 = self
            .boundary
            .hit(r, rec1.t + 0.0001, f64::INFINITY)
            .unwrap();
        //if(debugging) cout<<"t_min"<<rec1.t<<"t_max"<<rec2.t;
        if rec1.t < t_min {
            rec1.t = t_min;
        }
        if rec2.t > t_max {
            rec2.t = t_max;
        }
        if rec1.t >= rec2.t {
            return None;
        }
        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }
        let ray_length = r.dir.length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * f64::log(random_f64(0.0, 1.0), f64::exp(1.0));
        if hit_distance > distance_inside_boundary {
            return None;
        }
        let rec = HitRecord {
            t: rec1.t + hit_distance / ray_length,
            p: r.at(rec1.t + hit_distance / ray_length),
            u: 0.0,
            v: 0.0,
            normal: Vec3::new(1.0, 0.0, 0.0),
            front_face: true,
            mat_ptr: &self.phase_function,
        };
        //if debugging{cout<<hit_distance<<rec.t<<rec.p}
        Some(rec)
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        self.boundary.bounding_box(time0, time1, output_box)
    }
}
