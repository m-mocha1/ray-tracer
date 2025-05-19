mod vec3;
use vec3::Vec3;
fn main() {
    let w = 200;
    let h = 100;
    let max_value = 250;

    // write_ppm(w, h, max_value);

    let v = Vec3::new(1.0, 2.0, 6.0);
    let v2 = Vec3::new(1.0, 2.0, 6.0);

    let v3 = v + v2;
    println!("added value {:?}", v3)
}

fn write_ppm(w: i32, h: i32, max_value: i32) {
    println!("P3\n{} {}\n{}", w, h, max_value);

    for j in (0..h).rev() {
        for i in 0..w {
            let r = i as f32 / w as f32;
            let g = j as f32 / h as f32;
            let b = 0.2;

            let ir = (255.99 * r) as i32;
            let ig = (255.99 * g) as i32;
            let ib = (255.99 * b) as i32;
            println!("{} {} {}", ir, ig, ib)
        }
    }
}
