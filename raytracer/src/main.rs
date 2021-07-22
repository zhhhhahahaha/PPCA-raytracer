mod aabb;
mod camerafile;
mod hittable_listfile;
mod hittablefile;
mod materialfile;
mod moving_sphere;
mod ray;
mod boxfile;
#[allow(clippy::float_cmp)]
mod rtweekend;
mod spherefile;
mod texture;
mod vec3;
mod perlin;
mod aarect;

use aabb::AABB;
use camerafile::Camera;
use hittable_listfile::HittableList;
use hittablefile::HitRecord;
use hittablefile::Hittable;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use materialfile::{Dielectric,Metal,Lambertian, DiffuseLight,Material};
use moving_sphere::MovingSphere;
use ray::Ray;
use rtweekend::INFINITY;
use spherefile::Sphere;
use std::rc::Rc;
use texture::CheckerTexture;
use texture::{SolidColor, Texture,NoiseTexture};
use vec3::Vec3;
use perlin::Perlin;
use aarect::{XYRect,XZRect, YZRect};
use boxfile::Box;


use crate::rtweekend::random_f64;

fn cornell_box() -> HittableList {
    let mut objects = HittableList::new();
    let red = Rc::new(Lambertian::new2(&Vec3::new(0.65, 0.05, 0.05)));
    let white = Rc::new(Lambertian::new2(&Vec3::new(0.73, 0.73, 0.73)));
    let green = Rc::new(Lambertian::new2(&Vec3::new(0.12, 0.45, 0.15)));
    let light = Rc::new(DiffuseLight::new2(Vec3::new(3.0, 3.0, 3.0)));
    
    objects.add(Rc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    objects.add(Rc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    objects.add(Rc::new(XZRect::new(213.0, 343.0, 227.0, 332.0, 554.0, light)));
    objects.add(Rc::new(XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone())));
    objects.add(Rc::new(XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));
    objects.add(Rc::new(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));
    objects.add(Rc::new(Box::new(Vec3::new(130.0, 0.0, 65.0), Vec3::new(295.0, 165.0, 230.0), white.clone())));
    objects.add(Rc::new(Box::new(Vec3::new(265.0, 0.0, 295.0), Vec3::new(430.0, 330.0, 460.0), white.clone())));
    objects
}

fn simple_light () -> HittableList {
    let mut objects = HittableList::new();
    let pertext = Rc::new(NoiseTexture::new(4.0));
    objects.add(Rc::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Rc::new(Lambertian::new1(pertext.clone())))));
    objects.add(Rc::new(Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, Rc::new(Lambertian::new1(pertext.clone())))));
    let difflight = Rc::new(DiffuseLight::new2(Vec3::new(4.0, 4.0, 4.0)));
    objects.add(Rc::new(XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight)));
    objects
}

fn two_perlin_sphere() -> HittableList{
    let mut objects = HittableList::new();
    let pertext = Rc::new(NoiseTexture::new(4.0));
    objects.add(Rc::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Rc::new(Lambertian::new1(pertext.clone())))));
    objects.add(Rc::new(Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, Rc::new(Lambertian::new1(pertext.clone())))));
    objects
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();
    let checker = Rc::new(CheckerTexture::new2(
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));
    world.add(Rc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::new1(checker.clone())),
    )));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = random_f64(0.0, 1.0);
            let center: Vec3 = Vec3::new(
                a as f64 + 0.9 * random_f64(0.0, 1.0),
                0.2,
                b as f64 + 0.9 * random_f64(0.0, 1.0),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo: Vec3 = Vec3::elemul(Vec3::random(0.0, 1.0), Vec3::random(0.0, 1.0));
                    let sphere_material = Rc::new(Lambertian::new2(&albedo));
                    let center2: Vec3 = center + Vec3::new(0.0, random_f64(0.0, 0.5), 0.0);
                    world.add(Rc::new(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        sphere_material.clone(),
                    )));
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random(0.5, 1.0);
                    let fuzz: f64 = random_f64(0.0, 1.0);
                    let sphere_material = Rc::new(Metal::new(&albedo, fuzz));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material.clone())));
                } else {
                    let sphere_material = Rc::new(Dielectric::new(&1.5));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material.clone())));
                }
            }
        }
    }
    let material1 = Rc::new(Dielectric::new(&1.5));
    world.add(Rc::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Rc::new(Lambertian::new2(&Vec3::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::new(&Vec3::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));
    world
}

fn ray_color(r: &Ray,background: Vec3, world: &impl Hittable, depth: i32) -> Vec3 {
    let mut rec = HitRecord {
        p: Vec3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, 0.0),
        mat_ptr: Rc::new(Lambertian::new2(&Vec3::new(0.0, 0.0, 0.0))),
        t: 0.0,
        u: 0.0,
        v: 0.0,
        front_face: false,
    };
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }
    if !world.hit(*r, &0.001, &INFINITY, &mut rec) {
        return background;
    }
    //let target: Vec3 = rec.p + rec.normal + random_unit_vector();
    //return ray_color(&Ray::new(rec.p, target - rec.p), world, depth -1) * 0.5;
    let mut scattered: Ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0), 0.0);
    let mut attenuation: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let emitted: Vec3 = rec.mat_ptr.emitted(rec.u, rec.v, rec.p);
    if !rec
        .mat_ptr
        .scatter(&r, &rec, &mut attenuation, &mut scattered)
    {
        return emitted;
    }
    emitted + attenuation * ray_color(&scattered, background, world, depth -1)
    
}

fn main() {
    let x = Vec3::new(1.0, 1.0, 1.0);
    println!("{:?}", x);

    //image
    let mut img: RgbImage = ImageBuffer::new(600, 600);
    let bar = ProgressBar::new(1024);
    let aspect_ratio: f64 = 1.0;
    const IMAGE_WIDTH: i32 = 600;
    const IMAGE_HEIGHT: i32 = 600; //IMAGE_WIDTH / aspect_ratio
    let samples_per_pixel: i32 = 200;
    //world
    let world = cornell_box();

    //Camera
    let lookfrom: Vec3 = Vec3::new(278.0, 278.0, -800.0);
    let lookat: Vec3 = Vec3::new(278.0, 278.0, 0.0);
    let vup: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus: f64 = 10.0;
    let aperture: f64 = 0.0;
    let background: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let cam: Camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        &40.0,
        &aspect_ratio,
        &aperture,
        &dist_to_focus,
        0.0,
        1.0,
    );

    for x in 0..IMAGE_WIDTH {
        for y in 0..IMAGE_HEIGHT {
            let pixel = img.get_pixel_mut(x as u32, y as u32);
            let x1 = x as f64;
            let y1 = (IMAGE_HEIGHT - 1 - y) as f64;
            let mut color: Vec3 = Vec3::new(0.0, 0.0, 0.0);
            for _s in 0..samples_per_pixel {
                let u: f64 = (x1 + random_f64(0.0, 1.0)) / (IMAGE_WIDTH as f64 - 1.0);
                let v: f64 = (y1 + random_f64(0.0, 1.0)) / (IMAGE_HEIGHT as f64 - 1.0);
                let r: Ray = cam.get_ray(&u, &v);
                color += ray_color(&r, background, &world, 50);
            }
            let samples_per_pixel: f64 = 200.0;
            let red = (255.999 * ((color.x / samples_per_pixel).sqrt())) as u8;
            let green = (255.999 * ((color.y / samples_per_pixel).sqrt())) as u8;
            let blue = (255.999 * ((color.z / samples_per_pixel).sqrt())) as u8;
            //println!("{}, {}, {}", red, green, blue);
            *pixel = image::Rgb([red, green, blue]);
        }
        bar.inc(1);
    }

    img.save("output/test.png").unwrap();
    bar.finish();
}
