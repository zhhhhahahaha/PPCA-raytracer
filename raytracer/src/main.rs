#[allow(clippy::float_cmp)]
mod aabb;
mod camerafile;
mod hittable_listfile;
mod hittablefile;
mod materialfile;
mod moving_sphere;
mod ray;
mod boxfile;
mod rtweekend;
mod spherefile;
mod texture;
mod vec3;
mod perlin;
mod aarect;
mod constant_medium;
mod bvh;
mod onb;
mod pdf;

use aabb::AABB;
use camerafile::Camera;
use hittable_listfile::HittableList;
use hittablefile::{HitRecord,Hittable,Translate, Rotatey, FlipFace};
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use materialfile::{Dielectric,Lambertian, DiffuseLight,Material,Isotropic,ScatterRecord};
use ray::Ray;
use rtweekend::INFINITY;
use spherefile::Sphere;
use texture::{SolidColor, Texture};
use vec3::Vec3;
use perlin::Perlin;
use aarect::{XYRect,XZRect, YZRect};
use boxfile::RealBox;
use onb::Onb;
use pdf::{Pdf,CosinePdf,HittablePdf,MixturePdf};
use crate::rtweekend::random_f64;
use std::sync::mpsc::channel;
use std::sync::Arc;
use threadpool::ThreadPool;
use std::boxed::Box;

/* 
fn final_scene() -> HittableList {
    let mut boxes1 = HittableList::new();
    let ground = Arc::new(Lambertian::new2(&Vec3::new(0.48, 0.83, 0.53)));
    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w =100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_f64(1.0, 101.0);
            let z1 = z0 + w;
            boxes1.add(Arc::new(Box::new(Vec3::new(x0, y0, z0), Vec3::new(x1, y1, z1), ground.clone())));
        }
    }
    let mut objects = HittableList::new();
    objects.add(Arc::new(BvhNode::new(&boxes1.objects, 0, boxes1.objects.len(), 0.0, 1.0)));

    let light = Arc::new(DiffuseLight::new2(Vec3::new(7.0, 7.0, 7.0)));
    objects.add(Arc::new(XZRect::new(123.0, 423.0, 147.0, 412.0, 554.0, light.clone())));
    let center1: Vec3 = Vec3::new(400.0, 400.0, 200.0);
    let center2: Vec3 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_material = Arc::new(Lambertian::new2(&Vec3::new(0.7, 0.3, 0.1)));
    objects.add(Arc::new(MovingSphere::new(center1, center2, 0.0, 1.0, 50.0, moving_sphere_material.clone())));

    objects.add(Arc::new(Sphere::new(Vec3::new(260.0, 150.0, 45.0), 50.0, Arc::new(Dielectric::new(1.5)))));
    objects.add(Arc::new(Sphere::new(Vec3::new(0.0, 150.0, 145.0), 50.0, Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.9), 1.0)))));

    let mut boundary = Arc::new(Sphere::new(Vec3::new(360.0, 150.0, 145.0), 70.0, Arc::new(Dielectric::new(1.5))));
    objects.add(boundary.clone());
    objects.add(Arc::new(ConstantMedium::new2(boundary.clone(), 0.2, Vec3::new(0.2, 0.4, 0.9))));
    boundary = Arc::new(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 5000.0, Arc::new(Dielectric::new(1.5))));
    objects.add(Arc::new(ConstantMedium::new2(boundary.clone(), 0.0001, Vec3::new(1.0, 1.0, 1.0))));

    let emat = Arc::new(Lambertian::new1(Arc::new(ImageTexture::new2("input/earthmap.jpg"))));
    objects.add(Arc::new(Sphere::new(Vec3::new(400.0, 200.0, 400.0), 100.0, emat.clone())));
    let pertext = Arc::new(NoiseTexture::new(0.1));
    objects.add(Arc::new(Sphere::new(Vec3::new(220.0, 280.0, 300.0), 80.0, Arc::new(Lambertian::new1(pertext.clone())))));

    let mut boxes2 = HittableList::new();
    let white = Arc::new(Lambertian::new2(&Vec3::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for j in 0..ns {
        boxes2.add(Arc::new(Sphere::new(Vec3::random(0.0, 165.0), 10.0, white.clone())));
    }
    objects.add(Arc::new(Translate::new(Arc::new(Rotatey::new(Arc::new(BvhNode::new(&boxes2.objects, 0, boxes2.objects.len(), 0.0, 1.0)), 15.0)), Vec3::new(-100.0, 270.0, 395.0))));
    objects

}


fn cornell_smoke() -> HittableList {
    let mut objects = HittableList::new();
    let red = Arc::new(Lambertian::new2(&Vec3::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new2(&Vec3::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new2(&Vec3::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new2(Vec3::new(7.0, 7.0, 7.0)));
    
    objects.add(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    objects.add(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    objects.add(Arc::new(XZRect::new(113.0, 443.0, 127.0, 432.0, 554.0, light)));
    objects.add(Arc::new(XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone())));
    objects.add(Arc::new(XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));
    objects.add(Arc::new(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));
    let mut box1: Arc<dyn Hittable> = Arc::new(Box::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(165.0, 330.0, 165.0), white.clone()));
    box1 = Arc::new(Rotatey::new(box1, 15.0));
    box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    let mut box2: Arc<dyn Hittable> = Arc::new(Box::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(165.0, 165.0, 165.0), white.clone()));
    box2 = Arc::new(Rotatey::new(box2, -18.0));
    box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    objects.add(Arc::new(ConstantMedium::new2(box1, 0.01, Vec3::zero())));
    objects.add(Arc::new(ConstantMedium::new2(box2, 0.01, Vec3::new(1.0, 1.0, 1.0))));
    objects
}
*/
/* 
fn simple_light () -> HittableList {
    let mut objects = HittableList::new();
    let pertext = Arc::new(NoiseTexture::new(4.0));
    objects.add(Arc::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Arc::new(Lambertian::new1(pertext.clone())))));
    objects.add(Arc::new(Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, Arc::new(Lambertian::new1(pertext.clone())))));
    let difflight = Arc::new(DiffuseLight::new2(Vec3::new(4.0, 4.0, 4.0)));
    objects.add(Arc::new(XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight)));
    objects
}

fn two_perlin_sphere() -> HittableList{
    let mut objects = HittableList::new();
    let pertext = Arc::new(NoiseTexture::new(4.0));
    objects.add(Arc::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Arc::new(Lambertian::new1(pertext.clone())))));
    objects.add(Arc::new(Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, Arc::new(Lambertian::new1(pertext.clone())))));
    objects
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();
    let checker = Arc::new(CheckerTexture::new2(
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new1(checker.clone())),
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
                    let sphere_material = Arc::new(Lambertian::new2(&albedo));
                    let center2: Vec3 = center + Vec3::new(0.0, random_f64(0.0, 0.5), 0.0);
                    world.add(Arc::new(MovingSphere::new(
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
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material.clone())));
                } else {
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material.clone())));
                }
            }
        }
    }
    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new2(&Vec3::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));
    world
}
*/
fn cornell_box() -> HittableList {
    let mut objects = HittableList::new();
    let red = Lambertian::new2(Vec3::new(0.65, 0.05, 0.05));
    let white = Lambertian::new2(Vec3::new(0.73, 0.73, 0.73));
    let green = Lambertian::new2(Vec3::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::new2(Vec3::new(15.0, 15.0, 15.0));
    
    objects.add(Box::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    objects.add(Box::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    objects.add(Box::new(FlipFace::new(XZRect::new(213.0, 343.0, 227.0, 332.0, 554.0, light))));
    objects.add(Box::new(XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone())));
    objects.add(Box::new(XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));
    objects.add(Box::new(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));
    let box1 = RealBox::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(165.0, 330.0, 165.0), white.clone());
    let box1 = Rotatey::new(box1, 15.0);
    let box1 = Translate::new(box1, Vec3::new(265.0, 0.0, 295.0));
    let box1 = Box::new(box1);
    objects.add(box1);

    let glass = Dielectric::new(1.5);
    objects.add(Box::new(Sphere::new(Vec3::new(190.0, 90.0, 190.0), 90.0, glass.clone())));

    objects
}
fn ray_color(r: Ray,background: Vec3, world: &Arc<HittableList>,lights: &Arc<HittableList>, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }
    if let None = world.hit(r, 0.001, INFINITY) {
        return background;
    }
    let rec = world.hit(r, 0.001, INFINITY).unwrap();
    let mut srec = ScatterRecord {specular_ray:Ray::new(Vec3::zero(), Vec3::zero(), 0.0),
                                           is_specular:true,
                                           attenuation: Vec3::zero(),
                                           pdf_ptr: Box::new(CosinePdf::new(Vec3::zero())),};
    let emitted = rec.mat_ptr.emitted(r, &rec, rec.u, rec.v, rec.p);
    if !rec.mat_ptr.scatter(r, &rec, &mut srec) {
        return emitted;
    }
    if srec.is_specular {
        //let a = ray_color(srec.specular_ray, background, world, lights, depth - 1);
        //println!("{}, {}, {}", a.x, a.y, a.z);
        return Vec3::elemul(srec.attenuation, ray_color(srec.specular_ray, background, world, lights, depth - 1));
    }
    let tem1pdf = &**lights;
    let light_ptr = HittablePdf::new(tem1pdf, rec.p);
    let tem2pdf = &*srec.pdf_ptr;
    let p = MixturePdf::new(&light_ptr, tem2pdf);
    let scattered = Ray::new(rec.p, p.generate(), r.tm);
    let pdf_val = p.value(scattered.dir);
    emitted + Vec3::elemul(srec.attenuation, ray_color(scattered, background, world, lights, depth - 1)) * rec.mat_ptr.scattering_pdf(r, &rec, scattered) / pdf_val
}

fn main() {
    let (tx, rx) = channel();
    let n_jobs: usize = 32;
    let n_workers = 4;
    let pool = ThreadPool::new(n_workers);
    
    let bar = ProgressBar::new(n_jobs as u64);

    //image
    let aspect_ratio: f64 = 1.0;
    const IMAGE_WIDTH: i32 = 600;
    const IMAGE_HEIGHT: i32 = 600; //IMAGE_WIDTH / aspect_ratio
    let samples_per_pixel: i32 = 1000;
    //world
    let mut lights =HittableList::new();
    lights.add(Box::new(XZRect::new(213.0, 343.0, 227.0, 332.0, 554.0, DiffuseLight::new2(Vec3::new(7.0, 7.0, 7.0)))));
    lights.add(Box::new(Sphere::new(Vec3::new(190.0, 90.0, 190.0), 90.0,DiffuseLight::new2(Vec3::new(7.0, 7.0, 7.0)))));
    let world = Arc::new(cornell_box());
    let lights = Arc::new(lights);

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
    
    for i in 0..n_jobs {
        let tx = tx.clone();
        let world_ptr = world.clone();
        let lightscolne =  lights.clone(); 
        pool.execute(move || {
        let row_begin = IMAGE_HEIGHT as usize * i / n_jobs;
        let row_end = IMAGE_HEIGHT as usize * (i + 1) / n_jobs;
        let render_height = row_end - row_begin;
        let mut img: RgbImage = ImageBuffer::new(IMAGE_WIDTH as u32, render_height as u32);
        for x in 0..IMAGE_WIDTH {
            for(img_y, y) in (row_begin..row_end).enumerate() {
                let pixel = img.get_pixel_mut(x as u32, img_y as u32);
                let x1 = x as f64;
                let y1 = (IMAGE_HEIGHT - 1 - y as i32) as f64;
                let mut color: Vec3 = Vec3::new(0.0, 0.0, 0.0);
                for _s in 0..samples_per_pixel {
                    let u: f64 = (x1 + random_f64(0.0, 1.0)) / (IMAGE_WIDTH as f64 - 1.0);
                    let v: f64 = (y1 + random_f64(0.0, 1.0)) / (IMAGE_HEIGHT as f64 - 1.0);
                    let r: Ray = cam.get_ray(&u, &v);
                    color += ray_color(r, background, &world_ptr,&lightscolne , 50);
                }
                let red = (255.999 * ((color.x / samples_per_pixel as f64).sqrt())) as u8;
                let green = (255.999 * ((color.y / samples_per_pixel as f64).sqrt())) as u8;
                let blue = (255.999 * ((color.z / samples_per_pixel as f64).sqrt())) as u8;
                *pixel = image::Rgb([red, green, blue]);
            }
        }
        tx.send((row_begin..row_end, img)).expect("failed to send result");
        }
        )
    }
    let mut img: RgbImage = ImageBuffer::new(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);
    for (rows, data) in rx.iter().take(n_jobs) {
        for (idx, row) in rows.enumerate() {
            for col in 0..IMAGE_WIDTH {
                let row = row as u32;
                let idx = idx as u32;
                *img.get_pixel_mut(col as u32, row) = *data.get_pixel(col as u32, idx);
            }
        }
        bar.inc(1);
    }
    /*for x in 0..IMAGE_WIDTH {
        for y in 0..IMAGE_HEIGHT {
            let pixel = img.get_pixel_mut(x as u32, y as u32);
            let x1 = x as f64;
            let y1 = (IMAGE_HEIGHT - 1 - y) as f64;
            let mut color: Vec3 = Vec3::new(0.0, 0.0, 0.0);
            for _s in 0..samples_per_pixel {
                let u: f64 = (x1 + random_f64(0.0, 1.0)) / (IMAGE_WIDTH as f64 - 1.0);
                let v: f64 = (y1 + random_f64(0.0, 1.0)) / (IMAGE_HEIGHT as f64 - 1.0);
                let r: Ray = cam.get_ray(&u, &v);
                color += ray_color(r, background, &world,&lights , 50);
            }
            let r = color.x;
            let g = color.y;
            let b = color.z;
            if r!=r {color.x = 0.0}
            if g!=g {color.y = 0.0}
            if b!=b {color.z = 0.0}
            let samples_per_pixel: f64 = 10.0;
            let red = (255.999 * ((color.x / samples_per_pixel).sqrt())) as u8;
            let green = (255.999 * ((color.y / samples_per_pixel).sqrt())) as u8;
            let blue = (255.999 * ((color.z / samples_per_pixel).sqrt())) as u8;
            //println!("{}, {}, {}", red, green, blue);
            *pixel = image::Rgb([red, green, blue]);
        }
        bar.inc(1);
    }*/
    img.save("output/test.png").unwrap();
    bar.finish();
}
