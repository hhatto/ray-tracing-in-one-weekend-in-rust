use std::vec::Vec;
use utils::{vec3, ray, material};

#[allow(dead_code)]
#[derive(Clone)]
pub struct HitRecord {
    pub t: f32,
    pub p: vec3::Vec3,
    pub normal: vec3::Vec3,
    pub mat: Box<material::Material>,
}

#[allow(dead_code)]
impl HitRecord {
    pub fn new(m: Box<material::Material>) -> Self {
        Self {
            t: 0.,
            p: vec3::Vec3::new(0., 0., 0.),
            normal: vec3::Vec3::new(0., 0., 0.),
            mat: m,
        }
    }
}

pub trait Hitable {
    fn hit(&self, r: &ray::Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}

#[allow(dead_code)]
pub struct HitableList {
    pub list: Vec<Box<Hitable>>,
}

#[cfg(feature="concurrency")]
unsafe impl Sync for HitableList {}
#[cfg(feature="concurrency")]
unsafe impl Send for HitableList {}

#[allow(dead_code)]
impl HitableList {
    pub fn new(hitable: Vec<Box<Hitable>>) -> Self {
        Self { list: hitable }
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &ray::Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new(rec.mat.clone());
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for h in self.list.iter() {
            if h.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
        hit_anything
    }
}
