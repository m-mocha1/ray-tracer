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
use std::fs::File;
use std::io::BufWriter;
use std::io::{self, Write};
use vec3::Vec3;

fn color(r: &Ray, list: &HittableList, depth: u32) -> Vec3 {
    let max_depth = 20;
    let mut rec = HitRecord::default();

    if list.hit(r, 0.0, std::f32::MAX, &mut rec) {
        if depth >= max_depth {
            return Vec3::new(0.0, 0.0, 0.0);
        }
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
    let arr = ui();
    println!("img {:?}", arr);
    let width = arr[0];
    let height = arr[1];
    let samples = 100;
    let max_value = 255;
    // let width = 200;
    // let height = 100;

    let mut list = HittableList::new();
    list.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));
    list.add(Box::new(Sphere::new(Vec3::new(-0.2, 0.0, -1.0), 0.4)));
    list.add(Box::new(Sphere::new(Vec3::new(0.7, 0.0, -1.0), 0.4)));

    let cam = Camera::camera();
    let mut rng = rand::rng();

    let file = File::create("img.ppm").expect("Could not create file");
    let mut writer = BufWriter::new(file);
    writeln!(writer, "P3\n{} {}\n{}", width, height, max_value).unwrap();
    // println!("P3\n{} {}\n{}", width, height, max_value);

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
            writeln!(writer, "{} {} {}", ir, ig, ib).unwrap();
            // println!("{} {} {}", ir, ig, ib)
        }
    }
}
fn ui() -> Vec<i32> {
    print!("Enter Width, hight for example 200x100 : ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let input = input.trim();

    input
        .split('x')
        .map(|s| s.trim().parse().expect(""))
        .collect()
}
