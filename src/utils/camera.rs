use std::f32::consts::PI;
use utils::vec3::{Vec3, unit_vector, cross, dot};
use utils::ray::Ray;
use utils::random::drand48;

#[derive(Default)]
pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f32,
}

#[allow(dead_code)]
impl Camera {
    pub fn new() -> Self {
        Self {
            lower_left_corner: Vec3::new(-2., -1., -1.),
            horizontal: Vec3::new(4., 0., 0.),
            vertical: Vec3::new(0., 2., 0.),
            lens_radius: 1.,
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
            lens_radius: 1.,
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

    pub fn with_focus(lookfrom: &Vec3,
                      lookat: &Vec3,
                      vup: &Vec3,
                      vfov: f32,
                      aspect: f32,
                      aperture: f32,
                      focus_dist: f32)
                      -> Self {
        let lens_radius = aperture / 2.;
        let w: Vec3 = unit_vector(lookfrom.clone() - lookat.clone());
        let u: Vec3 = unit_vector(cross(vup, &w));
        let v: Vec3 = cross(&w, &u);
        let theta = vfov * PI / 180.;
        let half_height = (theta / 2.).tan();
        let half_width = aspect * half_height;
        let origin = lookfrom.clone();
        Self {
            origin: origin.clone(),
            lower_left_corner: origin - u.clone() * focus_dist * half_width - v.clone() * focus_dist * half_height -
                               w.clone() * focus_dist,
            horizontal: u.clone() * 2. * focus_dist * half_width,
            vertical: v.clone() * 2. * focus_dist * half_height,
            w: w,
            u: u,
            v: v,
            lens_radius: lens_radius,
            ..Self::default()
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        // let ray_vec = self.lower_left_corner.clone() + self.horizontal.clone() * u + self.vertical.clone() * v -
        //               self.origin.clone();
        // Ray::new(&self.origin, &ray_vec)

        let rd = random_in_unit_disk() * self.lens_radius;
        let offset: Vec3 = self.u.clone() * rd.x() + self.v.clone() * rd.y();
        let ray_vec = self.lower_left_corner.clone() + self.horizontal.clone() * u + self.vertical.clone() * v -
                       self.origin.clone() - offset.clone();
        let origin = self.origin.clone() + offset.clone();
        Ray::new(&origin, &ray_vec)
    }
}

#[allow(dead_code)]
pub fn random_in_unit_disk() -> Vec3 {
    let mut p: Vec3;
    loop {
        p = Vec3::new(drand48(), drand48(), 0.) * 2.0 - Vec3::new(1., 1., 0.);
        if !(dot(&p, &p) >= 1.) {
            break;
        }
    }
    p
}
