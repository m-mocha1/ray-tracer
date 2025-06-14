use crate::camera;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}
impl Camera {
    pub fn new() -> Vec<Camera> {
        vec![
            Camera {
                lower_left_corner: Vec3::new(-4.0, -1.0, -1.0),
                horizontal: Vec3::new(8.0, 0.0, 0.0),
                vertical: Vec3::new(0.0, 6.0, 0.0),
                origin: Vec3::new(0.0, 1.0, 6.0),
            }, // //top-view camera
            Camera {
                origin: Vec3::new(0.0, 100.0, 1.0),
                lower_left_corner: Vec3::new(-3.0, 50.0, -3.0),
                horizontal: Vec3::new(6.0, 0.0, 0.0),
                vertical: Vec3::new(0.0, 0.0, 6.0),
            },
            // right side
            Camera {
                origin: Vec3::new(5.0, 3.0, -1.0),
                lower_left_corner: Vec3::new(1.0, -4.0, -4.0),
                horizontal: Vec3::new(0.0, 0.0, 6.0),
                vertical: Vec3::new(0.0, 6.0, 0.0),
            },
            //left side
            Camera {
                origin: Vec3::new(-6.0, 3.0, -1.0),
                lower_left_corner: Vec3::new(-1.0, -4.0, -4.0),
                horizontal: Vec3::new(0.0, 0.0, 6.0),
                vertical: Vec3::new(0.0, 6.0, 0.0),
            },
        ]
    }
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::ray(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
