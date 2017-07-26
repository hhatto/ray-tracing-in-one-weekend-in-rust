use utils::vec3;

#[allow(dead_code)]
pub struct Ray<'a> {
    pub a: &'a vec3::Vec3,
    pub b: &'a vec3::Vec3,
}

#[allow(dead_code)]
impl<'a> Ray<'a> {
    pub fn new(a: &'a vec3::Vec3, b: &'a vec3::Vec3) -> Self {
        Self { a: a, b: b }
    }

    pub fn origin(&self) -> &'a vec3::Vec3 {
        self.a
    }

    pub fn direction(&self) -> &'a vec3::Vec3 {
        self.b
    }

    pub fn point_at_parameter(&self, t: f32) -> vec3::Vec3 {
        let z: vec3::Vec3 = self.a.clone() + self.b.clone() * t;
        z
    }
}
