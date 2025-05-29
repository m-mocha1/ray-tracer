use std::{clone, ops};

#[derive(Debug, PartialEq, Clone, Copy, Default)]
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

    // getters for ray direction
    pub fn x(self) -> f32 {
        self.e[0]
    }
    pub fn y(self) -> f32 {
        self.e[1]
    }
    pub fn z(self) -> f32 {
        self.e[2]
    }
    // getters the collors
    pub fn r(self) -> f32 {
        self.e[0]
    }
    pub fn g(self) -> f32 {
        self.e[1]
    }
    pub fn b(self) -> f32 {
        self.e[2]
    }

    pub fn dot(v1: &Vec3, v2: &Vec3) -> f32 {
        v1.e[0] * v2.e[0] + v1.e[1] * v2.e[1] + v1.e[2] * v2.e[2]
    }

    pub fn length(self) -> f32 {
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt()
    }

    pub fn unit_vector(self) -> Self {
        self / self.length()
    }

    pub fn squared_length(self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rs: Vec3) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] + rs.e[0],
                self.e[1] + rs.e[1],
                self.e[2] + rs.e[2],
            ],
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] - rhs.e[0],
                self.e[1] - rhs.e[1],
                self.e[2] - rhs.e[2],
            ],
        }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs],
        }
    }
}
impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            e: [rhs.e[0] * self, rhs.e[1] * self, rhs.e[2] * self],
        }
    }
}
impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        let k = 1.0 / rhs;
        Vec3 {
            e: [self.e[0] * k, self.e[1] * k, self.e[2] * k],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_vec3_add() {
        assert_eq!(
            Vec3::new(2.0, 4.0, 6.0) + Vec3::new(1.0, 1.0, 4.0),
            Vec3::new(3.0, 5.0, 10.0)
        );
    }
    #[test]
    fn test_vec3_mul() {
        assert_eq!(Vec3::new(2.0, 4.0, 6.0) * 2.0, Vec3::new(4.0, 8.0, 12.0));
    }
    #[test]
    fn test_vec3_div() {
        assert_eq!(Vec3::new(8.0, 4.0, 2.0) / 2.0, Vec3::new(4.0, 2.0, 1.0));
    }
}
