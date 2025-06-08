mod camera;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use rand::prelude::*;
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;
fn color(r: &Ray, list: &HittableList, depth: u32) -> Vec3 {
    let mut rec = HitRecord::default();

    if list.hit(r, 0.0, std::f32::MAX, &mut rec) {
        let target = rec.p() + rec.normal() + random_in_unit_sphere();
        return 0.5 * color(&Ray::ray(rec.p(), target - rec.p()), &list, depth + 1);
    } else {
        let unit_dir = r.direction().unit_vector();
        let t = 0.5 * (unit_dir.y() + 1.0);

        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let mut p = Vec3::default();
    let mut rng = rand::rng();

    loop {
        p =
            2.0 * Vec3::new(
                rng.random::<f32>(),
                rng.random::<f32>(),
                rng.random::<f32>(),
            ) - Vec3::new(1.0, 1.0, 1.0);

        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

fn main() {
    // let width = 500;
    // let height = 250;
    let samples = 100;
    let max_value = 255;
    let width = 200;
    let height = 100;

    let mut list = HittableList::new();
    list.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));
    list.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));

    let cam = Camera::camera();
    let mut rng = rand::rng();

    println!("P3\n{} {}\n{}", width, height, max_value);

    for j in (0..height).rev() {
        for i in 0..width {
            let mut col = Vec3::default();

            for _ in 0..samples {
                let u = (i as f32 + rng.random::<f32>()) / width as f32;
                let v = (j as f32 + rng.random::<f32>()) / height as f32;

                let r = &cam.get_ray(u, v);

                col = col + color(r, &list, 0);
            }
            col = col / samples as f32;

            let ir = (255.99 * col.r()) as i32;
            let ig = (255.99 * col.g()) as i32;
            let ib = (255.99 * col.b()) as i32;
            println!("{} {} {}", ir, ig, ib)
        }
    }
}
