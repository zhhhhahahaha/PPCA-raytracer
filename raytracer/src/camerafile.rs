use crate::random_f64;
use crate::rtweekend::degrees_to_radians;
use crate::vec3::random_in_unit_disk;
use crate::Ray;
use crate::Vec3;

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
    time0: f64,
    time1: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: &f64,
        aspect_ratio: &f64,
        aperture: &f64,
        focus_dist: &f64,
        time0: f64,
        time1: f64,
    ) -> Self {
        let theta: f64 = degrees_to_radians(*vfov);
        let h: f64 = f64::tan(theta / 2.0);
        let viewport_height: f64 = 2.0 * h;
        let viewport_width: f64 = aspect_ratio * viewport_height;
        let w: Vec3 = (lookfrom - lookat).unit();
        let u: Vec3 = Vec3::cross(vup, w).unit();
        let v: Vec3 = Vec3::cross(w, u);
        Self {
            origin: lookfrom,
            horizontal: u * viewport_width * *focus_dist,
            vertical: v * viewport_height * *focus_dist,
            lower_left_corner: lookfrom
                - u * viewport_width * *focus_dist / 2.0
                - v * viewport_height * *focus_dist / 2.0
                - w * *focus_dist,
            w,
            u,
            v,
            lens_radius: aperture / 2.0,
            time0,
            time1,
        }
    }
    pub fn get_ray(&self, s: &f64, t: &f64) -> Ray {
        let rd: Vec3 = random_in_unit_disk() * self.lens_radius;
        let offset: Vec3 = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * *s + self.vertical * *t
                - self.origin
                - offset,
            random_f64(self.time0, self.time1),
        )
    }
}
