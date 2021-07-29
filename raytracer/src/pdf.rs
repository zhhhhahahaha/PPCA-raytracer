use crate::rtweekend::random_cosine_direction;
use crate::rtweekend::random_f64;
use crate::Hittable;
use crate::Onb;
use crate::Vec3;
use std::f64::consts::PI;
use std::sync::Arc;

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
        if (cosine < 0.0) {
            0.0
        } else {
            cosine / PI
        }
    }
    fn generate(&self) -> Vec3 {
        self.uvw.localbyvector(random_cosine_direction())
    }
}
pub struct HittablePdf {
    pub o: Vec3,
    pub ptr: Arc<dyn Hittable>,
}
impl HittablePdf {
    pub fn new(p: Arc<dyn Hittable>, origin: Vec3) -> Self {
        Self { ptr: p, o: origin }
    }
}
impl Pdf for HittablePdf {
    fn value(&self, direction: Vec3) -> f64 {
        self.ptr.pdf_value(self.o, direction)
    }
    fn generate(&self) -> Vec3 {
        self.ptr.random(self.o)
    }
}
#[derive(Clone)]
pub struct MixturePdf {
    pub p: [Arc<dyn Pdf>; 2],
}
impl MixturePdf {
    pub fn new(p0: Arc<dyn Pdf>, p1: Arc<dyn Pdf>) -> Self {
        let p: [Arc<dyn Pdf>; 2] = [p0, p1];
        Self { p }
    }
}
impl Pdf for MixturePdf {
    fn value(&self, direction: Vec3) -> f64 {
        0.5 * self.p[0].value(direction) + 0.5 * self.p[1].value(direction)
    }
    fn generate(&self) -> Vec3 {
        if random_f64(0.0, 1.0) < 0.5 {
            return self.p[0].generate();
        } else {
            self.p[1].generate()
        }
    }
}
