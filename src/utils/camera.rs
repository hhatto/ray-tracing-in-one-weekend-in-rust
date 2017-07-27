use utils::vec3::Vec3;
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
            origin: Vec3::new(0., 0., 0.),
            ..Self::default()
        }
    }

    pub fn get_ray(&mut self, u: f32, v: f32) -> Ray {
        self.ray_vec = self.lower_left_corner.clone() + self.horizontal.clone() * u + self.vertical.clone() * v -
                       self.origin.clone();
        Ray::new(&self.origin, &self.ray_vec)
    }
}
