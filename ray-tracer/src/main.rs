mod ray;
mod vec3;
use std::mem::Discriminant;

use ray::Ray;
use vec3::Vec3;

fn color(r: &Ray) -> Vec3 {
    if hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Vec3::new(1.0, 0.0, 0.0);
    }

    let unit_dir = Vec3::unit_vector(&r.direction());
    let t = 0.5 * (unit_dir.y() + 1.0);

    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 0.1) * t
}

fn hit_sphere(center: &Vec3, radius: f32, r: &Ray) -> bool {
    let oc = r.origin() - *center;
    let a = Vec3::dot(&r.direction(), &r.direction());
    let b = 2.0 * Vec3::dot(&oc, &r.direction());
    let c = Vec3::dot(&oc, &oc) - radius * radius;

    let discriminant = b * b - 4.0 * a * c;

    discriminant > 0.0
}

fn main() {
    // let w = 800;
    // let h = 600;
    let w = 200;
    let h = 100;
    let max_value = 255;

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    println!("P3\n{} {}\n{}", w, h, max_value);

    for j in (0..h).rev() {
        for i in 0..w {
            let u = i as f32 / w as f32;
            let v = j as f32 / h as f32;

            let direction = lower_left_corner + horizontal * u + vertical * v;
            let r = Ray::ray(origin, direction);
            let color = color(&r);

            let ir = (255.99 * color.r()) as i32;
            let ig = (255.99 * color.g()) as i32;
            let ib = (255.99 * color.b()) as i32;
            println!("{} {} {}", ir, ig, ib)
        }
    }
}
