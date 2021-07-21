use std::vec::Vec;
use crate::rtweekend::random_i32;
use crate::random_f64;
use crate::Vec3;
use std::usize;

pub const POINT_COUNT: usize = 256;
#[derive(Clone)]
pub struct Perlin {
     ranvec: Vec<Vec3>,
     perm_x: Vec<i32>,
     perm_y: Vec<i32>,
     perm_z: Vec<i32>,
}
impl Perlin {
    fn permute(p:&mut Vec<i32>, n: i32) {
        let mut i:usize = (n - 1) as usize;
        while i > 0 {
            let target:usize = random_i32(0, i as i32) as usize;
            let tmp:i32 = p[i];
            p[i] = p[target];
            p[target] = tmp;
            i-=1;
        }
    }
    fn perlin_generate_perm() -> Vec<i32> {
        let mut p:Vec<i32> = vec![0;POINT_COUNT];
        for i in 0..POINT_COUNT {
            p[i] = i as i32;
        }
        Perlin::permute(&mut p, POINT_COUNT as i32);
        p
    }
    pub fn new() -> Self {
        let mut rf: Vec<Vec3> = vec![Vec3::new(0.0, 0.0, 0.0);POINT_COUNT];
        for i in 0..POINT_COUNT {
            rf[i] = Vec3::random(-1.0, 1.0).unit();
        }
        Self{ranvec: rf.clone(),
             perm_x: Perlin::perlin_generate_perm(),
             perm_y: Perlin::perlin_generate_perm(),
             perm_z: Perlin::perlin_generate_perm(),
            }
    }
    pub fn noise(&self, p: &Vec3) -> f64 {
        let u:f64 = p.x - p.x.floor();
        let v:f64 = p.y - p.y.floor();
        let w:f64 = p.z - p.z.floor();
        let i:i32 = p.x.floor() as i32;
        let j:i32 = p.y.floor() as i32;
        let k:i32 = p.z.floor() as i32;
        let mut c = [[[Vec3::new(0.0, 0.0, 0.0);2];2];2];
        for di in 0..2{
            for dj in 0..2{
                for dk in 0..2{
                    c[di][dj][dk] = self.ranvec[
                        (self.perm_x[((i + di as i32) & 255) as usize] ^
                        self.perm_y[((j + dj as i32) & 255) as usize] ^
                        self.perm_z[((k + dk as i32) & 255) as usize]) as usize
                    ];
                }
            }
        }
        Perlin::perlin_interp(c, u, v, w)
    }
    fn perlin_interp(c:[[[Vec3;2];2];2], u:f64, v:f64, w:f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum: f64 = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v:Vec3 = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu)) *
                             (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv)) *
                             (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww)) * 
                             (c[i as usize][j as usize][k as usize] * weight_v);
                }
            }
        }
        accum
    }
    pub fn turb(&self, p: Vec3, depth:i32) -> f64 {
        let mut accum:f64 = 0.0;
        let mut temp_p: Vec3 = p;
        let mut weight: f64 = 1.0;

        for i in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }
        accum.abs()
    }
}