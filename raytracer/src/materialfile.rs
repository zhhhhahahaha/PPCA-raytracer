use crate::rtweekend::{random_cosine_direction, random_f64};
use crate::vec3::random_in_hemisphere;
use crate::vec3::random_in_unit_sphere;
use crate::vec3::random_unit_vector;
use crate::vec3::reflect;
use crate::vec3::refract;
use crate::CosinePdf;
use crate::HitRecord;
use crate::Onb;
use crate::Pdf;
use crate::Ray;
use crate::SolidColor;
use crate::Texture;
use crate::Vec3;
use std::f64::consts::PI;
use std::ptr::NonNull;
use std::sync::Arc;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        false
    }
    fn emitted(&self, r_in: Ray, rec: &HitRecord, u: f64, v: f64, p: Vec3) -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
    fn scattering_pdf(&self, r_in: Ray, rec: HitRecord, scattered: Ray) -> f64 {
        0.0
    }
}

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Arc<dyn Texture>,
}
impl Lambertian {
    pub fn new1(a: Arc<dyn Texture>) -> Self {
        Self { albedo: a }
    }
    pub fn new2(a: &Vec3) -> Self {
        Self {
            albedo: Arc::new(SolidColor::new1(*a)),
        }
    }
}
impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.is_specular = false;
        srec.attenuation = self.albedo.value(rec.u, rec.v, rec.p);
        srec.pdf_ptr = Arc::new(CosinePdf::new(rec.normal));
        true
    }
    fn scattering_pdf(&self, r_in: Ray, rec: HitRecord, scattered: Ray) -> f64 {
        let cosine = rec.normal * scattered.dir.unit();
        if cosine < 0.0 {
            0.0
        } else {
            cosine / PI
        }
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        let reflected: Vec3 = reflect(&r_in.dir.unit(), &rec.normal);
        srec.specular_ray = Ray::new(rec.p, reflected + random_in_unit_sphere() * self.fuzz, 0.0);
        srec.attenuation = self.albedo;
        srec.is_specular = true;
        srec.pdf_ptr = Arc::new(CosinePdf::new(Vec3::zero())); //这里应该把指针制空的
        true
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.is_specular = true;
        srec.pdf_ptr = Arc::new(CosinePdf::new(Vec3::zero())); //这里其实要把指针制空
        srec.attenuation = Vec3::new(1.0, 1.0, 1.0);
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
            srec.specular_ray = Ray::new(rec.p, reflected, r_in.tm);
            return true;
        }
        let reflect_prob: f64 = schlick(&cos_theta, &etai_over_etat);
        if random_f64(0.0, 1.0) < reflect_prob {
            let reflected: Vec3 = reflect(&unit_direction, &rec.normal);
            srec.specular_ray = Ray::new(rec.p, reflected, r_in.tm);
            return true;
        }
        let refracted: Vec3 = refract(&unit_direction, &rec.normal, &etai_over_etat);
        srec.specular_ray = Ray::new(rec.p, refracted, r_in.tm);
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
    emit: Arc<dyn Texture>,
}
impl DiffuseLight {
    pub fn new1(a: Arc<dyn Texture>) -> Self {
        Self { emit: a }
    }
    pub fn new2(c: Vec3) -> Self {
        Self {
            emit: Arc::new(SolidColor::new1(c)),
        }
    }
}
impl Material for DiffuseLight {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        false
    }
    fn emitted(&self, r_in: Ray, rec: &HitRecord, u: f64, v: f64, p: Vec3) -> Vec3 {
        if rec.front_face {
            self.emit.value(u, v, p)
        } else {
            Vec3::new(0.0, 0.0, 0.0)
        }
    }
}
#[derive(Clone)]
pub struct Isotropic {
    albedo: Arc<dyn Texture>,
}
impl Isotropic {
    pub fn new1(c: Vec3) -> Self {
        Self {
            albedo: Arc::new(SolidColor::new1(c)),
        }
    }
    pub fn new2(a: Arc<dyn Texture>) -> Self {
        Self { albedo: a }
    }
}
impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.specular_ray = Ray::new(rec.p, random_in_unit_sphere(), r_in.tm);
        srec.attenuation = self.albedo.value(rec.u, rec.v, rec.p);
        true
    }
}
#[derive(Clone)]
pub struct ScatterRecord {
    pub specular_ray: Ray,
    pub is_specular: bool,
    pub attenuation: Vec3,
    pub pdf_ptr: Arc<dyn Pdf>,
}
