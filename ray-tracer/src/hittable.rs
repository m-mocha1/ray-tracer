use std::default;
use std::fmt::Debug;

use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
}

impl HitRecord {
    pub fn p(&self) -> Vec3 {
        self.p
    }
    pub fn t(&self) -> f32 {
        self.t
    }
    pub fn normal(&self) -> Vec3 {
        self.normal
    }
    pub fn set_p(&mut self, val: Vec3) {
        self.p = val
    }
    pub fn set_t(&mut self, val: f32) {
        self.t = val
    }
    pub fn set_normal(&mut self, val: Vec3) {
        self.normal = val
    }
}

pub trait Hittable: Debug {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        false
    }
}
