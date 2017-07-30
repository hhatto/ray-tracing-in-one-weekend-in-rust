use utils::hitable::{Hitable, HitRecord};
use utils::ray::Ray;
use utils::vec3::{dot, Vec3};
use utils::random::drand48;
use utils::material::Material;

#[allow(dead_code)]
pub struct Sphere {
    pub center: Vec3,
    radius: f32,
    mat: Box<Material>,
}

#[allow(dead_code)]
impl Sphere {
    pub fn new(cen: Vec3, r: f32, m: Box<Material>) -> Self {
        Self {
            center: cen,
            radius: r,
            mat: m,
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
                rec.mat = self.mat.clone();
                return true;
            }
            temp = (-b + discriminaun.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p.clone() - self.center.clone()) / self.radius;
                rec.mat = self.mat.clone();
                return true;
            }
        }
        return false;
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::new(drand48(), drand48(), drand48()) * 2.0 - Vec3::new(1., 1., 1.);
        if p.squared_len() >= 1.0 {
            return p;
        }
    }
}
