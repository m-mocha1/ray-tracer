use crate::hittable::*;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Cube {
    pub min: Vec3,
    pub max: Vec3,
}

impl Cube {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut tmin = t_min;
        let mut tmax = t_max;

        for a in 0..3 {
            let inv_d = 1.0 / ray.direction()[a];
            let mut t0 = (self.min[a] - ray.origin()[a]) * inv_d;
            let mut t1 = (self.max[a] - ray.origin()[a]) * inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            tmin = if t0 > tmin { t0 } else { tmin };
            tmax = if t1 < tmax { t1 } else { tmax };

            if tmax <= tmin {
                return false;
            }
        }

        rec.t = tmin;
        rec.p = ray.point_at_parameter(tmin);

        let epsilon = 0.001;
        let p = rec.p;
        let min = self.min;
        let max = self.max;

        rec.normal = if (p.x() - min.x()).abs() < epsilon {
            Vec3::new(-1.0, 0.0, 0.0)
        } else if (p.x() - max.x()).abs() < epsilon {
            Vec3::new(1.0, 0.0, 0.0)
        } else if (p.y() - min.y()).abs() < epsilon {
            Vec3::new(0.0, -1.0, 0.0)
        } else if (p.y() - max.y()).abs() < epsilon {
            Vec3::new(0.0, 1.0, 0.0)
        } else if (p.z() - min.z()).abs() < epsilon {
            Vec3::new(0.0, 0.0, -1.0)
        } else {
            Vec3::new(0.0, 0.0, 1.0)
        };

        true
    }
}
