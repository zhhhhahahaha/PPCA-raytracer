use crate::Vec3;
use crate::Ray;
use crate::rtweekend::degrees_to_radians;
use crate::vec3::random_in_unit_disk;

#[derive(Clone, Debug, PartialEq,Copy)]
pub struct camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u:Vec3,
    v:Vec3,
    w:Vec3,
    lens_radius:f64,
}

impl camera {
    pub fn new(lookfrom: Vec3, lookat:Vec3, vup: Vec3, vfov: &f64, aspect_ratio: &f64, aperture: &f64, focus_dist: &f64) -> Self {
        let theta:f64 = degrees_to_radians(vfov.clone());
        let h: f64 = f64::tan(theta.clone() / 2.0);
        let viewport_height: f64 = 2.0 * h;
        let viewport_width: f64 = aspect_ratio * viewport_height;
        let w:Vec3 = (lookfrom - lookat).unit();
        let u:Vec3 = Vec3::cross(vup, w).unit();
        let v:Vec3 = Vec3::cross(w, u);
        Self{
            origin: lookfrom,
            horizontal: u * viewport_width * *focus_dist,
            vertical: v * viewport_height * *focus_dist,
            lower_left_corner: lookfrom - u * viewport_width * *focus_dist / 2.0 - v * viewport_height * *focus_dist / 2.0 - w * *focus_dist,
            w,u,v,lens_radius: aperture / 2.0,
        }
    }
    pub fn get_ray(&self, s: &f64, t: &f64) -> Ray {
        let rd:Vec3 = random_in_unit_disk() * self.lens_radius;
        let offset: Vec3 = self.u * rd.x + self.v * rd.y;
        return Ray::new(self.origin + offset, self.lower_left_corner + self.horizontal * *s + self.vertical * *t - self.origin - offset);
    }

}