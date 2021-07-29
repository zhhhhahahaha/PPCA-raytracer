use crate::Vec3;
use rand::{thread_rng, Rng};
use std::f64::consts::PI;

pub const INFINITY: f64 = f64::INFINITY;
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
pub fn random_f64(min: f64, max: f64) -> f64 {
    let mut rng = thread_rng();
    let random: f64 = rng.gen();
    min + (max - min) * random
}
pub fn fmin(left: f64, right: f64) -> f64 {
    if left < right {
        return left;
    } else {
        right
    }
}
pub fn fmax(left: f64, right: f64) -> f64 {
    if left > right {
        return left;
    } else {
        right
    }
}
pub fn random_i32(min: i32, max: i32) -> i32 {
    random_f64(min as f64, (max + 1) as f64) as i32
}
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}
pub fn random_cosine_direction() -> Vec3 {
    let r1 = random_f64(0.0, 1.0);
    let r2 = random_f64(0.0, 1.0);
    let z = (1.0 - r2).sqrt();
    let phi = 2.0 * PI * r1;
    let x = f64::cos(phi) * r2.sqrt();
    let y = f64::sin(phi) * r2.sqrt();
    Vec3::new(x, y, z)
}
