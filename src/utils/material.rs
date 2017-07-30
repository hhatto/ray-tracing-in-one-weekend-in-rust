use utils::vec3::{dot, unit_vector, Vec3};
use utils::ray::Ray;
use utils::sphere::random_in_unit_sphere;
use utils::hitable::HitRecord;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
    fn box_clone(&self) -> Box<Material>;
    fn name(&self) -> String;
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct DummyMat {
    pub albedo: Vec3,
}

impl Clone for Box<Material> {
    fn clone(&self) -> Box<Material> {
        self.box_clone()
    }
}

#[allow(dead_code)]
impl DummyMat {
    pub fn new() -> Self {
        Self { albedo: Vec3::new(0., 0., 0.) }
    }
}

impl Material for DummyMat {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord, _attenuation: &mut Vec3, _scattered: &mut Ray) -> bool {
        true
    }
    fn box_clone(&self) -> Box<Material> {
        Box::new((*self).clone())
    }
    fn name(&self) -> String {
        "dummy".to_string()
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Vec3,
}

#[allow(dead_code)]
impl Lambertian {
    pub fn new(a: Vec3) -> Self {
        Self { albedo: a }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let target = rec.normal.clone() + random_in_unit_sphere();
        let s_ray = Ray::new(&rec.p, &target);
        *scattered = s_ray.clone();
        *attenuation = self.albedo.clone();
        true
    }
    fn box_clone(&self) -> Box<Material> {
        Box::new((*self).clone())
    }
    fn name(&self) -> String {
        "lambertian".to_string()
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

#[allow(dead_code)]
impl Metal {
    pub fn new(a: Vec3, f: f32) -> Self {
        Self {
            albedo: a,
            fuzz: if f < 1. {
                f
            } else {
                1.
            },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let uv = unit_vector(r_in.direction().clone());
        let reflected = reflect(&uv, &rec.normal) + random_in_unit_sphere() * self.fuzz;
        let s_ray = Ray::new(&rec.p, &reflected);
        *scattered = s_ray.clone();
        *attenuation = self.albedo.clone();
        dot(scattered.direction(), &rec.normal) > 0.
    }
    fn box_clone(&self) -> Box<Material> {
        Box::new((*self).clone())
    }
    fn name(&self) -> String {
        "metal".to_string()
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v.clone() - n.clone() * dot(v, n) * 2.
}
