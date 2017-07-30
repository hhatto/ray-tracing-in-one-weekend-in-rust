use std::f32::consts::PI;
use utils::vec3::{Vec3, unit_vector, cross};
use utils::ray::Ray;

#[derive(Default)]
pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,

    #[allow(dead_code)]
    ray_vec: Vec3,
}

#[allow(dead_code)]
impl Camera {
    pub fn new() -> Self {
        Self {
            lower_left_corner: Vec3::new(-2., -1., -1.),
            horizontal: Vec3::new(4., 0., 0.),
            vertical: Vec3::new(0., 2., 0.),
            ..Self::default()
        }
    }

    pub fn with_fov(vfov: f32, aspect: f32) -> Self {
        let theta = vfov * PI / 180.;
        let half_height = (theta / 2.).tan();
        let half_width = aspect * half_height;
        Self {
            lower_left_corner: Vec3::new(-half_width, -half_height, -1.),
            horizontal: Vec3::new(2. * half_width, 0., 0.),
            vertical: Vec3::new(0., 2. * half_height, 0.),
            ..Self::default()
        }
    }

    pub fn with_lookat(lookfrom: &Vec3, lookat: &Vec3, vup: &Vec3, vfov: f32, aspect: f32) -> Self {
        let w: Vec3 = unit_vector(lookfrom.clone() - lookat.clone());
        let u: Vec3 = unit_vector(cross(vup, &w));
        let v: Vec3 = cross(&w, &u);
        let theta = vfov * PI / 180.;
        let half_height = (theta / 2.).tan();
        let half_width = aspect * half_height;
        let origin = lookfrom.clone();
        Self {
            origin: origin,
            lower_left_corner: lookfrom.clone() - u.clone() * half_width - v.clone() * half_height - w,
            horizontal: u * 2. * half_width,
            vertical: v * 2. * half_height,
            ..Self::default()
        }
    }

    pub fn get_ray(&mut self, u: f32, v: f32) -> Ray {
        self.ray_vec = self.lower_left_corner.clone() + self.horizontal.clone() * u + self.vertical.clone() * v -
                       self.origin.clone();
        Ray::new(&self.origin, &self.ray_vec)
    }
}
