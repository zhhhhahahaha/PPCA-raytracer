use crate::materialfile::Material;
use crate::HitRecord;
use crate::Hittable;
use crate::Ray;
use crate::Vec3;
use crate::AABB;
use crate::Onb;
use crate::Lambertian;
use crate::rtweekend::random_f64;
use std::f64::consts::PI;
use std::rc::Rc;

#[derive(Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub mat_ptr: Rc<dyn Material>,
}
impl Sphere {
    pub fn new(center: Vec3, radius: f64, m: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            mat_ptr: m,
        }
    }
    fn get_sphere_uv(p: Vec3, u: &mut f64, v: &mut f64) {
        let theta: f64 = (-p.y).acos();
        let phi: f64 = f64::atan2(-p.z, p.x) + PI;
        *u = phi / (2.0 * PI);
        *v = theta / PI;
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: &f64, t_max: &f64, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = r.orig - self.center;
        let a: f64 = r.dir * r.dir;
        let half_b: f64 = r.dir * oc;
        let c: f64 = oc * oc - self.radius * self.radius;
        let discriminant: f64 = f64::powf(half_b, 2.0) - a * c;
        if discriminant < 0.0 {
            return false;
        } else {
            let root: f64 = discriminant.sqrt();
            let t: f64 = (-half_b - root) / a;
            if t > *t_min && t < *t_max {
                rec.t = t;
                rec.p = r.at(t);
                let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, outward_normal);
                Sphere::get_sphere_uv(outward_normal, &mut rec.u, &mut rec.v);
                rec.mat_ptr = self.mat_ptr.clone();
                return true;
            }
            let t: f64 = (-half_b + root) / a;
            if t > *t_min && t < *t_max {
                rec.t = t;
                rec.p = r.at(t);
                let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, outward_normal);
                Sphere::get_sphere_uv(outward_normal, &mut rec.u, &mut rec.v);
                rec.mat_ptr = self.mat_ptr.clone();
                return true;
            }
        }
        false
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        );
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
        if !self.hit(Ray::new(o, v, 0.0), &0.001, &f64::INFINITY, &mut rec){
            return 0.0;
        }
        let cos_theta_max = (1.0 - self.radius * self.radius / (self.center - o).squared_length()).sqrt();
        let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);
        1.0 / solid_angle
    }
    fn random(&self, o: Vec3) -> Vec3 {
        let direction = self.center - o;
        let distance_squared = direction.squared_length();
        let uvw = Onb::new(direction);
        uvw.localbyvector(random_to_sphere(self.radius, distance_squared))
    }
}
pub fn random_to_sphere(radius:f64, distance_squared:f64) -> Vec3 {
    let r1 = random_f64(0.0, 1.0);
    let r2 = random_f64(0.0, 1.0);
    let z = 1.0 + r2 * ((1.0 - radius * radius / distance_squared).sqrt() - 1.0);

    let phi = 2.0 * PI * r1;
    let x = f64::cos(phi) * (1.0 - z * z).sqrt();
    let y = f64::sin(phi) * (1.0 - z * z).sqrt();

    Vec3::new(x, y, z)
}
