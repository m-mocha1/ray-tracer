use std::{clone, ops};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3 {
    e: [f32; 3],
}
// pub struct Vec3 {
//     pub x: f64,
//     pub y: f64,
//     pub z: f64,
// }

impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Self {
        Vec3 { e: [e0, e1, e2] }
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rs: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] + rs.e[0],
                self.e[1] + rs.e[1],
                self.e[2] + rs.e[2],
            ],
        }
    }
}