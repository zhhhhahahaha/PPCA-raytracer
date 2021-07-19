use rand::{thread_rng, Rng};
use std::f64::consts::PI;

pub const INFINITY: f64 = f64::INFINITY;
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
pub fn random_f64(min: &f64, max: &f64) -> f64 {
    let mut rng = thread_rng();
    let random: f64 = rng.gen();
    min + (max - min) * random
}
/*pub fn clamp(x: &f64, min: &f64, max: &f64) -> f64 {
    if x < min {
        return *min;
    }
    if x > max {
        return *max;
    }
    return *x;
}*/
