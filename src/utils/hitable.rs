use std::vec::Vec;
use utils::{vec3, ray};

#[allow(dead_code)]
#[derive(Clone, Default)]
pub struct HitRecord {
    pub t: f32,
    pub p: vec3::Vec3,
    pub normal: vec3::Vec3,
}

pub trait Hitable {
    fn hit(&self, r: &ray::Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}

#[allow(dead_code)]
pub struct HitableList<'a> {
    pub list: Vec<&'a Hitable>,
}

#[allow(dead_code)]
impl<'a> HitableList<'a> {
    pub fn new(hitable: Vec<&'a Hitable>) -> Self {
        Self { list: hitable }
    }
}

impl<'a> Hitable for HitableList<'a> {
    fn hit(&self, r: &ray::Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord { ..HitRecord::default() };
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
