use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Default, Debug)]
pub struct Vec3 {
    pub e: (f32, f32, f32),
}

#[allow(dead_code)]
impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Self {
        Self { e: (e0, e1, e2) }
    }

    pub fn x(&self) -> f32 {
        self.e.0
    }
    pub fn y(&self) -> f32 {
        self.e.1
    }
    pub fn z(&self) -> f32 {
        self.e.2
    }

    pub fn len(&self) -> f32 {
        (self.e.0 * self.e.0 + self.e.1 * self.e.1 + self.e.2 * self.e.2).sqrt()
    }

    pub fn squared_len(&self) -> f32 {
        self.e.0 * self.e.0 + self.e.1 * self.e.1 + self.e.2 * self.e.2
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.e.0 + other.e.0, self.e.1 + other.e.1, self.e.2 + other.e.2)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.e.0 - other.e.0, self.e.1 - other.e.1, self.e.2 - other.e.2)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self.e.0 * other.e.0, self.e.1 * other.e.1, self.e.2 * other.e.2)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Vec3 {
        Vec3::new(self.e.0 * other, self.e.1 * other, self.e.2 * other)
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3::new(self.e.0 / other.e.0, self.e.1 / other.e.1, self.e.2 / other.e.2)
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f32) -> Vec3 {
        Vec3::new(self.e.0 / other, self.e.1 / other, self.e.2 / other)
    }
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    let l = v.len();
    v / l
}

pub fn dot(v1: &Vec3, v2: &Vec3) -> f32 {
    v1.e.0 * v2.e.0 + v1.e.1 * v2.e.1 + v1.e.2 * v2.e.2
}

#[allow(dead_code)]
pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
    Vec3::new(v1.e.1 * v2.e.2 - v1.e.2 * v2.e.1,
              (v1.e.0 * v2.e.2 - v1.e.2 * v2.e.0) * -1.,
              v1.e.0 * v2.e.1 - v1.e.1 * v2.e.0)
}
