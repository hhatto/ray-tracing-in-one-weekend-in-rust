use utils::hitable::{Hitable, HitRecord};
use utils::ray::Ray;
use utils::vec3::{dot, Vec3};

#[allow(dead_code)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

#[allow(dead_code)]
impl Sphere {
    pub fn new(cen: Vec3, r: f32) -> Self {
        Self {
            center: cen,
            radius: r,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = r.origin().clone() - self.center.clone();
        let a: f32 = dot(r.direction(), r.direction());
        let b: f32 = dot(&oc, r.direction());
        let c: f32 = dot(&oc, &oc) - self.radius * self.radius;
        let discriminaun = b * b - a * c;
        if discriminaun > 0. {
            let mut temp: f32 = (-b - discriminaun.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p.clone() - self.center.clone()) / self.radius;
                return true;
            }
            temp = (-b + discriminaun.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p.clone() - self.center.clone()) / self.radius;
                return true;
            }
        }
        return false;
    }
}
