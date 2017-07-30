use utils::vec3::{dot, unit_vector, Vec3};
use utils::ray::Ray;
use utils::sphere::random_in_unit_sphere;
use utils::hitable::HitRecord;
use utils::random::drand48;

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
            fuzz: if f < 1. { f } else { 1. },
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

#[allow(dead_code)]
#[derive(Clone)]
pub struct Dielectric {
    pub ref_idx: f32,
}

#[allow(dead_code)]
impl Dielectric {
    pub fn new(ri: f32) -> Self {
        Self { ref_idx: ri }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let outward_normal: Vec3;
        let reflected = reflect(r_in.direction(), &rec.normal);
        let ni_over_nt: f32;
        let mut refracted: Vec3 = Vec3::new(0., 0., 0.);
        let reflect_prob: f32;
        let cosine: f32;
        *attenuation = Vec3::new(1., 1., 1.);

        if dot(r_in.direction(), &rec.normal) > 0. {
            outward_normal = rec.normal.clone() * -1.;
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * dot(r_in.direction(), &rec.normal) / r_in.direction().len()
        } else {
            outward_normal = rec.normal.clone();
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = dot(r_in.direction(), &rec.normal) / r_in.direction().len() * -1.;
        }

        if refract(r_in.direction(), &outward_normal, ni_over_nt, &mut refracted) {
            reflect_prob = schlick(cosine, self.ref_idx);
        } else {
            reflect_prob = 1.;
        }

        if drand48() < reflect_prob {
            let s_ray = Ray::new(&rec.p, &reflected);
            *scattered = s_ray.clone();
        } else {
            let s_ray = Ray::new(&rec.p, &refracted);
            *scattered = s_ray.clone();
        }

        true
    }
    fn box_clone(&self) -> Box<Material> {
        Box::new((*self).clone())
    }
    fn name(&self) -> String {
        "dielectric".to_string()
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v.clone() - n.clone() * dot(v, n) * 2.
}

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32, refracted: &mut Vec3) -> bool {
    let uv = unit_vector(v.clone());
    let dt: f32 = dot(&uv, n);
    let discriminant: f32 = 1.0 - ni_over_nt * ni_over_nt * (1. - dt * dt);
    if discriminant > 0. {
        *refracted = (uv - n.clone() * dt) * ni_over_nt - n.clone() * discriminant.sqrt();
        return true;
    }
    false
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0: f32 = (1. - ref_idx) / (1. + ref_idx);
    r0 = r0 * r0;
    r0 + (1. - r0) * (1. - cosine).powi(5)
}
