use crate::rtweekend::random_f64;
use crate::HitRecord;
use crate::Hittable;
use crate::Isotropic;
use crate::Lambertian;
use crate::Material;
use crate::Texture;
use crate::Vec3;
use crate::AABB;
use std::sync::Arc;

#[derive(Clone)]
pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    phase_function: Arc<dyn Material>,
    neg_inv_density: f64,
}
impl ConstantMedium {
    pub fn new1(b: Arc<dyn Hittable>, d: f64, a: Arc<dyn Texture>) -> Self {
        Self {
            boundary: b,
            neg_inv_density: -1.0 / d,
            phase_function: Arc::new(Isotropic::new2(a)),
        }
    }
    pub fn new2(b: Arc<dyn Hittable>, d: f64, c: Vec3) -> Self {
        Self {
            boundary: b,
            neg_inv_density: -1.0 / d,
            phase_function: Arc::new(Isotropic::new1(c)),
        }
    }
}
impl Hittable for ConstantMedium {
    fn hit(&self, r: crate::ray::Ray, t_min: &f64, t_max: &f64, rec: &mut HitRecord) -> bool {
        let enableDebug: bool = false;
        let debugging = enableDebug && random_f64(0.0, 1.0) < 0.00001;
        let mut rec1: HitRecord = HitRecord {
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            mat_ptr: Arc::new(Lambertian::new2(&Vec3::new(0.0, 0.0, 0.0))),
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: false,
        };
        let mut rec2: HitRecord = HitRecord {
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            mat_ptr: Arc::new(Lambertian::new2(&Vec3::new(0.0, 0.0, 0.0))),
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: false,
        };
        if !self
            .boundary
            .hit(r, &-f64::INFINITY, &f64::INFINITY, &mut rec1)
        {
            return false;
        }
        if !self
            .boundary
            .hit(r, &(rec1.t + 0.0001), &f64::INFINITY, &mut rec2)
        {
            return false;
        }
        //if(debugging) cout<<"t_min"<<rec1.t<<"t_max"<<rec2.t;
        if rec1.t < *t_min {
            rec1.t = *t_min;
        }
        if rec2.t > *t_max {
            rec2.t = *t_max;
        }
        if rec1.t >= rec2.t {
            return false;
        }
        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }
        let ray_length = r.dir.length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * f64::log(random_f64(0.0, 1.0), f64::exp(1.0));
        if hit_distance > distance_inside_boundary {
            return false;
        }
        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);
        //if debugging{cout<<hit_distance<<rec.t<<rec.p}
        rec.normal = Vec3::new(1.0, 0.0, 0.0);
        rec.front_face = true;
        rec.mat_ptr = self.phase_function.clone();
        true
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        self.boundary.bounding_box(time0, time1, output_box)
    }
}
