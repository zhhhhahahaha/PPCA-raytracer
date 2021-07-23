use crate::rtweekend::random_f64;
use crate::vec3::random_in_unit_sphere;
use crate::vec3::random_unit_vector;
use crate::vec3::reflect;
use crate::vec3::refract;
use crate::HitRecord;
use crate::Ray;
use crate::SolidColor;
use crate::Texture;
use crate::Vec3;
use std::rc::Rc;

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
    fn emitted(&self, u: f64, v:f64, p: Vec3) -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
}

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Rc<dyn Texture>,
}
impl Lambertian {
    pub fn new1(a: Rc<dyn Texture>) -> Self {
        Self { albedo: a }
    }
    pub fn new2(a: &Vec3) -> Self {
        Self {
            albedo: Rc::new(SolidColor::new1(*a)),
        }
    }
}
impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction: Vec3 = rec.normal + random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray::new(rec.p, scatter_direction, r_in.tm);
        *attenuation = self.albedo.value(rec.u, rec.v, rec.p);
        true
    }
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}
impl Metal {
    pub fn new(a: Vec3, f: f64) -> Self {
        Self {
            albedo: a,
            fuzz: if f < 1.0 { f } else { 1.0 },
        }
    }
}
impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected: Vec3 = reflect(&r_in.dir.unit(), &rec.normal);
        *scattered = Ray::new(
            rec.p,
            reflected + random_in_unit_sphere() * self.fuzz,
            r_in.tm,
        );
        *attenuation = self.albedo;
        (scattered.dir * rec.normal) > 0.0
    }
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Dielectric {
    pub ref_idx: f64,
}
impl Dielectric {
    pub fn new(ref_idx: f64) -> Self {
        Self { ref_idx: ref_idx }
    }
}
impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Vec3::new(1.0, 1.0, 1.0);
        let etai_over_etat: f64 = if rec.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };
        let unit_direction: Vec3 = r_in.dir.unit();
        let cos_theta: f64 = if -unit_direction * rec.normal > 1.0 {
            1.0
        } else {
            -unit_direction * rec.normal
        };
        let sin_theta: f64 = (1.0 - cos_theta * cos_theta).sqrt();
        if etai_over_etat * sin_theta > 1.0 {
            let reflected: Vec3 = reflect(&unit_direction, &rec.normal);
            *scattered = Ray::new(rec.p, reflected, r_in.tm);
            return true;
        }
        let reflect_prob: f64 = schlick(&cos_theta, &etai_over_etat);
        if random_f64(0.0, 1.0) < reflect_prob {
            let reflected: Vec3 = reflect(&unit_direction, &rec.normal);
            *scattered = Ray::new(rec.p, reflected, r_in.tm);
            return true;
        }
        let refracted: Vec3 = refract(&unit_direction, &rec.normal, &etai_over_etat);
        *scattered = Ray::new(rec.p, refracted, r_in.tm);
        true
    }
}
pub fn schlick(cosine: &f64, ref_idx: &f64) -> f64 {
    let mut r0: f64 = (1.0 - *ref_idx) / (1.0 + *ref_idx);
    r0 *= r0;
    r0 + (1.0 - r0) * f64::powf(1.0 - *cosine, 5.0)
}

#[derive(Clone)]
pub struct DiffuseLight {
    emit: Rc<dyn Texture>,
}
impl DiffuseLight {
    pub fn new1(a: Rc<dyn Texture>) -> Self {
        Self{ emit:a}
    }
    pub fn new2(c: Vec3) -> Self {
        Self{ emit: Rc::new(SolidColor::new1(c))}
    }
}
impl Material for DiffuseLight {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        false
    }
    fn emitted(&self, u: f64, v:f64, p: Vec3) -> Vec3 {
        self.emit.value(u, v, p)
    }
}
#[derive(Clone)]
pub struct Isotropic {
      albedo: Rc<dyn Texture>,
}
impl Isotropic {
    pub fn new1(c: Vec3) -> Self {
        Self{albedo: Rc::new(SolidColor::new1(c))}
    }
    pub fn new2(a: Rc<dyn Texture>) -> Self{
        Self{albedo:a}
    }
}
impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        *scattered = Ray::new(rec.p, random_in_unit_sphere(), r_in.tm);
        *attenuation = self.albedo.value(rec.u, rec.v, rec.p);
        true
    }
}