use crate::rtweekend::random_cosine_direction;
use crate::rtweekend::random_f64;
use crate::Hittable;
use crate::Onb;
use crate::Vec3;
use std::f64::consts::PI;

pub trait Pdf {
    fn value(&self, direction: Vec3) -> f64;
    fn generate(&self) -> Vec3;
}
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct CosinePdf {
    pub uvw: Onb,
}
impl CosinePdf {
    pub fn new(w: Vec3) -> Self {
        Self { uvw: Onb::new(w) }
    }
}
impl Pdf for CosinePdf {
    fn value(&self, direction: Vec3) -> f64 {
        let cosine = direction.unit() * self.uvw.axis[2];
        if cosine < 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }
    fn generate(&self) -> Vec3 {
        self.uvw.localbyvector(random_cosine_direction())
    }
}
#[derive(Clone)]
pub struct HittablePdf<'a, T: Hittable> {
    pub o: Vec3,
    pub ptr: &'a T,
}
impl<'a, T: Hittable> HittablePdf<'a, T> {
    pub fn new(p: &'a T, origin: Vec3) -> Self {
        Self { ptr: p, o: origin }
    }
}
impl<'a, T: Hittable> Pdf for HittablePdf<'a, T> {
    fn value(&self, direction: Vec3) -> f64 {
        self.ptr.pdf_value(self.o, direction)
    }
    fn generate(&self) -> Vec3 {
        self.ptr.random(self.o)
    }
}
#[derive(Clone)]
pub struct MixturePdf<'a> {
    pub p0: &'a dyn Pdf,
    pub p1: &'a dyn Pdf,
}
impl<'a> MixturePdf<'a> {
    pub fn new(p0: &'a dyn Pdf, p1: &'a dyn Pdf) -> Self {
        Self { p0: p0, p1: p1 }
    }
}
impl<'a> Pdf for MixturePdf<'a> {
    fn value(&self, direction: Vec3) -> f64 {
        0.5 * self.p0.value(direction) + 0.5 * self.p1.value(direction)
    }
    fn generate(&self) -> Vec3 {
        if random_f64(0.0, 1.0) < 0.5 {
            return self.p0.generate();
        } else {
            self.p1.generate()
        }
    }
}
