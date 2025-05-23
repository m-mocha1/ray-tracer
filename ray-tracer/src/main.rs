mod ray;
mod vec3;
use ray::Ray;
use vec3::Vec3;

fn color(r: &Ray) -> Vec3 {
    let unit_dir = Vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_dir.y() + 1.0);

    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 0.1) * t
}

fn main() {
    let w = 200;
    let h = 100;
    let max_value = 250;

    let lower_left_cornar = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    println!("P3\n{} {}\n{}", w, h, max_value);

    for j in (0..h).rev() {
        for i in 0..w {
            let u = i as f32 / w as f32;
            let v = j as f32 / h as f32;
            let r = Ray::ray(origin, lower_left_cornar + horizontal * u + vertical * v);
            let color = color(&r);

            let ir = (255.99 * color.r()) as i32;
            let ig = (255.99 * color.g()) as i32;
            let ib = (255.99 * color.b()) as i32;
            println!("{} {} {}", ir, ig, ib)
        }
    }
}
