use crate::vec3::Vec3;

pub struct Ray {
    A: Vec3,
    B: Vec3,
}

impl Ray {
    pub fn ray(a: Vec3, b: Vec3) -> Ray {
        Ray { A: a, B: b }
    }

    fn origin(self) -> Vec3 {
        self.A
    }
    fn direction(self) -> Vec3 {
        self.B
    }
    fn point_at_parameter(self, t: f32) -> Vec3 {
        self.A + self.B * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ray_origin() {}
    #[test]
    fn test_ray_direction() {}
    #[test]
    fn test_ray_point() {}
}
