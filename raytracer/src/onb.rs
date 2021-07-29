use crate::Vec3;

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Onb {
    pub axis: [Vec3; 3],
}
impl Onb {
    pub fn localbynum(&self, a: f64, b: f64, c: f64) -> Vec3 {
        self.axis[0] * a + self.axis[1] * b + self.axis[2] * c
    }
    pub fn localbyvector(&self, a: Vec3) -> Vec3 {
        self.axis[0] * a.x + self.axis[1] * a.y + self.axis[2] * a.z
    }
    pub fn new(n: Vec3) -> Self {
        let mut axis = [Vec3::zero(); 3];
        axis[2] = n.unit();
        let a = if axis[2].x.abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        axis[1] = Vec3::cross(axis[2], a).unit();
        axis[0] = Vec3::cross(axis[2], axis[1]);
        Self { axis }
    }
}
