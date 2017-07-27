extern crate rand;

mod utils;

use rand::{thread_rng, Rng};
use utils::vec3::{Vec3, unit_vector};
use utils::ray::Ray;
use utils::hitable::{Hitable, HitableList, HitRecord};
use utils::sphere::Sphere;
use utils::camera::Camera;

fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::new(drand48(), drand48(), drand48()) * 2.0 - Vec3::new(1., 1., 1.);
        if p.squared_len() >= 1.0 {
            return p;
        }
    }
}

fn color(r: &Ray, world: &Hitable) -> Vec3 {
    let mut rec = HitRecord { ..HitRecord::default() };
    if world.hit(r, 0.001, std::f32::MAX, &mut rec) {
        let target = rec.normal + random_in_unit_sphere();
        return color(&Ray::new(&rec.p, &target), world) * 0.5;
    }
    let unit_direction = unit_vector(r.direction().clone());
    let t: f32 = 0.5 * (unit_direction.y() + 1.);
    Vec3::new(1., 1., 1.) * (1. - t) + Vec3::new(0.5, 0.7, 1.) * t
}

fn drand48() -> f32 {
    let mut rng = thread_rng();
    rng.gen_range(0., 1.)
}

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;
    println!("P3\n{} {}\n255", nx, ny);

    let s1 = Sphere::new(Vec3::new(0., 0., -1.), 0.5);
    let s2 = Sphere::new(Vec3::new(0., -100.5, -1.), 100.);
    let list: Vec<&Hitable> = vec![&s1, &s2];
    let world = HitableList::new(list);
    let mut cam = Camera::new();

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
            col = Vec3::new(col.e.0.sqrt(), col.e.1.sqrt(), col.e.2.sqrt());
            let ir = (255.99 * col.e.0) as i32;
            let ig = (255.99 * col.e.1) as i32;
            let ib = (255.99 * col.e.2) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
