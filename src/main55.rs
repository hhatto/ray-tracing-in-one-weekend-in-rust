mod utils;

use utils::vec3::{Vec3, unit_vector};
use utils::ray::Ray;
use utils::hitable::{Hitable, HitableList, HitRecord};
use utils::sphere::Sphere;

fn color(r: &Ray, world: &Hitable) -> Vec3 {
    let mut rec = HitRecord { ..HitRecord::default() };
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
    println!("P3\n{} {}\n255", nx, ny);

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let s1 = Sphere::new(Vec3::new(0., 0., -1.), 0.5);
    let s2 = Sphere::new(Vec3::new(0., -100.5, -1.), 100.);
    let list: Vec<&Hitable> = vec![&s1, &s2];
    let world = HitableList::new(list);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u: f32 = i as f32 / nx as f32;
            let v: f32 = j as f32 / ny as f32;
            let hu: Vec3 = horizontal.clone() * u;
            let vv: Vec3 = vertical.clone() * v;
            let llc = lower_left_corner.clone() + hu + vv;
            let r = Ray::new(&origin, &llc);

            // let p = r.point_at_parameter(2.); // NOTE: not use?

            let col = color(&r, &world);
            let ir = (255.99 * col.e.0) as i32;
            let ig = (255.99 * col.e.1) as i32;
            let ib = (255.99 * col.e.2) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
