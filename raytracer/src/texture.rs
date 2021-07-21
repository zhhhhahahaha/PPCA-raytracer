use crate::Vec3;
use std::{rc::Rc, str};
use crate::Perlin;
use std::vec::Vec;
use crate::rtweekend::clamp;
use image;
use imageproc::drawing::Canvas;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3;
}
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct SolidColor {
    color_value: Vec3,
}
impl SolidColor {
    pub fn new1(c: Vec3) -> Self {
        Self { color_value: c }
    }
    pub fn new2(red: f64, green: f64, blue: f64) -> Self {
        Self {
            color_value: Vec3::new(red, green, blue),
        }
    }
}
impl Texture for SolidColor {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        self.color_value
    }
}
#[derive(Clone)]
pub struct CheckerTexture {
    odd: Rc<dyn Texture>,
    even: Rc<dyn Texture>,
}
impl CheckerTexture {
    pub fn new1(odd: Rc<dyn Texture>, even: Rc<dyn Texture>) -> Self {
        Self { odd, even }
    }
    pub fn new2(c1: Vec3, c2: Vec3) -> Self {
        Self {
            even: Rc::new(SolidColor::new1(c1)),
            odd: Rc::new(SolidColor::new1(c2)),
        }
    }
}
impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        let sines: f64 = f64::sin(10.0 * p.x) * f64::sin(10.0 * p.y) * f64::sin(10.0 * p.z);
        if sines < 0.0 {
            return self.odd.value(u, v, p);
        } else {
            self.even.value(u, v, p)
        }
    }
}
#[derive(Clone)]
pub struct NoiseTexture {
    pub noise: Perlin,
    pub scale: f64,
}
impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
       Self{noise:Perlin::new(),scale}
    }
}
impl Texture for NoiseTexture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        Vec3::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + f64::sin(self.scale * p.z + 10.0 * self.noise.turb(p, 7)))
    }
}

#[derive(Clone)]
pub struct ImageTexture {
    data: Vec<u8>,
    width: i32,
    height: i32,
    bytes_per_scanline: i32,
}
impl ImageTexture {
    pub fn new1() -> Self {
        Self{data: Vec::new(),
             width:0,
             height:0,
             bytes_per_scanline:0,
            }
    }
    pub fn new2(path: &str) -> Self {
        let img = image::open(path).unwrap();
        Self {
            data: img.clone().into_bytes(),
            width: img.clone().width() as i32,
            height: img.clone().height() as i32,
            bytes_per_scanline: img.clone().width() as i32 * 3
        }
    }
}
impl Texture for ImageTexture {
    fn value(&self, mut u: f64, mut v: f64, p: Vec3) -> Vec3 {
        if self.data.is_empty() {
            return Vec3::new(0.0, 1.0, 1.0);
        }
        u = clamp(u, 0.0, 1.0);
        v = 1.0 - clamp(v,0.0, 1.0);
        let mut i :i32 = (u * self.width as f64) as i32;
        let mut j: i32 = (v * self.height as f64) as i32;
        if i >= self.width {i = self.width - 1;}
        if j >=self.height {j = self.height - 1;}
        let color_scale: f64 = 1.0 / 255.0;
        let coordinate: usize = (1 + j * self.bytes_per_scanline + i * 3) as usize;
        Vec3::new(self.data[coordinate] as f64 * color_scale, self.data[coordinate + 1] as f64 * color_scale, self.data[coordinate + 2] as f64 * color_scale)
    }
}
