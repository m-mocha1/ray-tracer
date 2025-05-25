mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;

fn color(r: &Ray, list: &HittableList) -> Vec3 {
    let mut rec = HitRecord::default();

    if list.hit(r, 0.0, std::f32::MAX, &mut rec) {
        0.5 * Vec3::new(
            rec.normal.x() + 1.0,
            rec.normal.y() + 1.0,
            rec.normal.z() + 1.0,
        )
    } else {
        let unit_dir = r.direction().unit_vector();
        let t = 0.5 * (unit_dir.y() + 1.0);
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}

fn main() {
    let w = 800;
    let h = 600;
    // let w = 200;
    // let h = 100;
    let max_value = 255;

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let mut list = HittableList::new();
    list.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));
    list.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));

    println!("P3\n{} {}\n{}", w, h, max_value);

    for j in (0..h).rev() {
        for i in 0..w {
            let u = i as f32 / w as f32;
            let v = j as f32 / h as f32;

            let direction = lower_left_corner + horizontal * u + vertical * v;
            let r = Ray::ray(origin, direction);
            let color = color(&r, &list);

            let ir = (255.99 * color.r()) as i32;
            let ig = (255.99 * color.g()) as i32;
            let ib = (255.99 * color.b()) as i32;
            println!("{} {} {}", ir, ig, ib)
        }
    }
}
