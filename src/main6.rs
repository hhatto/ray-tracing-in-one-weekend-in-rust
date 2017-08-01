mod utils;

use utils::vec3::{Vec3, unit_vector};
use utils::ray::Ray;
use utils::hitable::{Hitable, HitableList, HitRecord};
use utils::sphere::Sphere;
use utils::camera::Camera;
use utils::material::DummyMat;
use utils::random::drand48;

fn color(r: &Ray, world: &Hitable) -> Vec3 {
    let mut rec = HitRecord::new(Box::new(DummyMat::new()));
    if world.hit(r, 0., std::f32::MAX, &mut rec) {
        return Vec3::new(rec.normal.x() + 1., rec.normal.y() + 1., rec.normal.z() + 1.) * 0.5;
    }
    let unit_direction = unit_vector(r.direction().clone());
    let t: f32 = 0.5 * (unit_direction.y() + 1.);
    Vec3::new(1., 1., 1.) * (1. - t) + Vec3::new(0.5, 0.7, 1.) * t
}

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;
    println!("P3\n{} {}\n255", nx, ny);

    let s1 = Box::new(Sphere::new(Vec3::new(0., 0., -1.), 0.5, Box::new(DummyMat::new())));
    let s2 = Box::new(Sphere::new(Vec3::new(0., -100.5, -1.), 100., Box::new(DummyMat::new())));
    let world = HitableList::new(vec![s1, s2]);
    let cam = Camera::new();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0., 0., 0.);
            for _ in 0..ns {
                let u: f32 = (i as f32 + drand48()) / nx as f32;
                let v: f32 = (j as f32 + drand48()) / ny as f32;
                let r = cam.get_ray(u, v);

                // let p = r.point_at_parameter(2.); // NOTE: not use?

                col = col + color(&r, &world);
            }
            col = col / ns as f32;
            let ir = (255.99 * col.e.0) as i32;
            let ig = (255.99 * col.e.1) as i32;
            let ib = (255.99 * col.e.2) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
