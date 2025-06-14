mod camera;
mod cube;
mod cylinder;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use cube::Cube;
use cylinder::Cylinder;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use rand::prelude::*;
use ray::Ray;
use sphere::Sphere;
use std::fs::File;
use std::io::BufWriter;
use std::io::{self, Write};
use vec3::Vec3;
/*
    to run use (cargo run)
    enter width x height like the example
    don't use (cargo run > img.ppm)
*/
fn color(r: &Ray, list: &HittableList, depth: u32) -> Vec3 {
    let max_depth = 20;
    let mut rec = HitRecord::default();
    if list.hit(r, 0.0, std::f32::MAX, &mut rec) {
        if depth >= max_depth {
            return Vec3::new(0.0, 0.0, 0.0);
        }
        // Check if the hit object is the sphere under the objects

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
    let samples = 100;
    let max_value = 255;
    let width = 800;
    let height = 600;

    let mut list = HittableList::new();
    let objects = ui();
    list.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, 0.0), 100.0))); // Ground sphere
    for (obj_type, position, size) in objects {
        match obj_type.as_str() {
            "sphere" => list.add(Box::new(Sphere::new(position, size))),
            "cylinder" => list.add(Box::new(Cylinder::new(position, size, size * 2.0))),
            "cube" => list.add(Box::new(Cube::new(
                position - Vec3::new(size, size, size),
                position + Vec3::new(size, size, size),
            ))),
            _ => println!("Unknown object type: {}", obj_type),
        }
    }

    let cams = Camera::new();
    let mut rng = rand::rng();
    let mut i = 0;

    for cam in cams {
        let filename = format!("img{}.ppm", i);
        let file = File::create(filename).expect("Could not create file");
        let mut writer = BufWriter::new(file);
        writeln!(writer, "P3\n{} {}\n{}", width, height, max_value).unwrap();

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
            }
        }
        println!("imgReady {}", i);
        i += 1;
    }
}
fn ui() -> Vec<(String, Vec3, f32)> {
    println!(
        "type x,y,z size (sphere 0,0,-1 0.5) (cube 2.0,0.0,-1 0.5) (cylinder -2.0,-0.5,-2.0 0.5)"
    );
    println!("done when finished.");

    let mut objects = Vec::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim();
        if input.eq_ignore_ascii_case("done") {
            break;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() != 3 {
            println!("Invalid format. Use: type x,y,z size");
            continue;
        }

        let obj_type = parts[0].to_string();
        let position_parts: Vec<f32> = parts[1]
            .split(',')
            .map(|s| s.trim().parse().expect("Invalid position format"))
            .collect();
        if position_parts.len() != 3 {
            println!("Invalid position format. Use: x,y,z");
            continue;
        }
        let position = Vec3::new(position_parts[0], position_parts[1], position_parts[2]);

        let size: f32 = parts[2].parse().expect("Invalid size format");

        objects.push((obj_type, position, size));
    }

    objects
}
