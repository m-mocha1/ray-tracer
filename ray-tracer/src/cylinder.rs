use crate::hittable::*;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Cylinder {
    pub center: Vec3,
    pub radius: f32,
    pub height: f32,
}

impl Cylinder {
    pub fn new(center: Vec3, radius: f32, height: f32) -> Self {
        Self {
            center,
            radius,
            height,
        }
    }
}

impl Hittable for Cylinder {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = ray.origin() - self.center;
        let dir = ray.direction();

        let a = dir.x() * dir.x() + dir.z() * dir.z();
        let b = 2.0 * (oc.x() * dir.x() + oc.z() * dir.z());
        let c = oc.x() * oc.x() + oc.z() * oc.z() - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;

        let mut hit_anything = false;

        if discriminant >= 0.0 {
            let sqrt_disc = discriminant.sqrt();
            let mut t = (-b - sqrt_disc) / (2.0 * a);

            for _ in 0..2 {
                if t < t_max && t > t_min {
                    let y = ray.origin().y() + t * dir.y();
                    if y >= self.center.y() && y <= self.center.y() + self.height {
                        rec.t = t;
                        rec.p = ray.point_at_parameter(t);
                        let normal = Vec3::new(
                            (rec.p.x() - self.center.x()) / self.radius,
                            0.0,
                            (rec.p.z() - self.center.z()) / self.radius,
                        );
                        rec.normal = normal.unit_vector();
                        hit_anything = true;
                        break;
                    }
                }
                t = (-b + sqrt_disc) / (2.0 * a);
            }
        }

        // ✅ إضافة تقاطع القاعدتين (caps)
        // Bottom cap
        // let t1 = (self.center.y() - ray.origin().y()) / ray.direction().y();
        // if t1 > t_min && t1 < t_max {
        //     let p = ray.point_at_parameter(t1);
        //     let dx = p.x() - self.center.x();
        //     let dz = p.z() - self.center.z();
        //     if dx * dx + dz * dz <= self.radius * self.radius {
        //         rec.t = t1;
        //         rec.p = p;
        //         rec.normal = Vec3::new(0.0, -1.0, 0.0);
        //         hit_anything = true;
        //     }
        // }

        // Top cap
        // let top_y = self.center.y() + self.height;
        // let t2 = (top_y - ray.origin().y()) / ray.direction().y();
        // if t2 > t_min && t2 < t_max {
        //     let p = ray.point_at_parameter(t2);
        //     let dx = p.x() - self.center.x();
        //     let dz = p.z() - self.center.z();
        //     if dx * dx + dz * dz <= self.radius * self.radius {
        //         rec.t = t2;
        //         rec.p = p;
        //         rec.normal = Vec3::new(0.0, 0.0, 0.0);
        //         hit_anything = true;
        //     }
        // }

        hit_anything
    }
}
