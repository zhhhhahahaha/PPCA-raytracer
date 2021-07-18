use crate::Ray;
use crate::hit_record;
use crate::Vec3;
use crate::rtweekend::random_f64;
use crate::vec3;
use crate::vec3::random_unit_vector;
use crate::vec3::reflect;
use crate::vec3::random_in_unit_sphere;
use crate::vec3::refract;
pub trait material {
    fn scatter(&self, r_in: &Ray, rec: &hit_record, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}

#[derive(Clone, Debug, PartialEq,Copy)]
pub struct lambertian {
    pub albedo: Vec3,
}
impl lambertian {
    pub fn new(a: &Vec3) -> Self {
        Self { albedo: *a}
    }
}
impl material for lambertian {
    fn scatter(&self, r_in: &Ray, rec: &hit_record, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let scatter_direction: Vec3 = rec.normal + random_unit_vector();
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        return true;
    }
}
#[derive(Clone, Debug, PartialEq,Copy)]
pub struct metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}
impl metal {
     pub fn new(a: &Vec3, f: f64) -> Self {
          Self { albedo: *a,
                 fuzz: if f < 1.0 {f} else {1.0},
          }
      }
}
impl material for metal {
     fn scatter(&self, r_in: &Ray, rec: &hit_record, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
         let reflected: Vec3 = reflect(&r_in.dir.unit(), &rec.normal);
         *scattered = Ray::new(rec.p, reflected + random_in_unit_sphere() * self.fuzz);
         *attenuation = self.albedo;
         return (scattered.dir * rec.normal) > 0.0;
     }
}
#[derive(Clone, Debug, PartialEq,Copy)]
pub struct dielectric {
    pub ref_idx: f64,
}
impl dielectric {
    pub fn new(ref_idx: &f64) -> Self{
        Self {ref_idx: *ref_idx}
    }
}
impl material for dielectric {
    fn scatter(&self, r_in: &Ray, rec: &hit_record, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
    *attenuation = Vec3::new(1.0, 1.0, 1.0);
    let etai_over_etat:f64 = if rec.front_face {1.0 / self.ref_idx} else {self.ref_idx};
    let unit_direction: Vec3 = r_in.dir.unit();
    let cos_theta: f64 = if -unit_direction * rec.normal > 1.0 {1.0} else {-unit_direction * rec.normal};
    let sin_theta: f64 = (1.0 - cos_theta * cos_theta).sqrt();
    if etai_over_etat * sin_theta > 1.0 {
        let reflected: Vec3 = reflect(&unit_direction, &rec.normal);
        *scattered = Ray::new(rec.p, reflected);
        return true;
    }
    let reflect_prob: f64 = schlick(&cos_theta, &etai_over_etat);
    if random_f64(&0.0, &1.0) < reflect_prob {
        let reflected: Vec3 = reflect(&unit_direction, &rec.normal);
        *scattered = Ray::new(rec.p, reflected);
        return true;
    }
    let refracted: Vec3 = refract(&unit_direction, &rec.normal, &etai_over_etat);
    *scattered = Ray::new(rec.p, refracted);
    return true;
    }
}
pub fn schlick(cosine: &f64, ref_idx: &f64) -> f64 {
     let mut r0: f64 = (1.0 - *ref_idx) / (1.0 + *ref_idx);
     r0 *= r0;
     return r0 + (1.0 - r0) * f64::powf(1.0 - *cosine, 5.0);
}