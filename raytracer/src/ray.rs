use crate::Vec3;

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Ray {
    pub orig: Vec3,
    pub dir: Vec3,
    pub tm: f64,//默认是0.0
}

impl Ray {
    pub fn at(&self, t: f64) -> Vec3 {
        self.orig + self.dir * t
    }
    pub fn new(orig: Vec3, dir: Vec3, tm: f64) -> Self {
        Self { orig, dir, tm }
    }
}
