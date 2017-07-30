mod utils;

use utils::vec3::{Vec3, unit_vector};
use utils::ray::Ray;
use utils::hitable::{Hitable, HitableList, HitRecord};
use utils::sphere::Sphere;
use utils::camera::Camera;
use utils::material::{DummyMat, Lambertian, Metal};
use utils::random::drand48;

fn color(r: &Ray, world: &Hitable, depth: i32) -> Vec3 {
    let mut rec = HitRecord::new(Box::new(DummyMat::new()));
    if world.hit(r, 0.001, std::f32::MAX, &mut rec) {
        //println!("mat={}", rec.mat.as_ref().name());
        let v1 = Vec3::new(0., 0., 0.);
        let v2 = Vec3::new(0., 0., 0.);
        let mut scattered = Ray::new(&v1, &v2);
        let mut attenuation = Vec3::new(0., 0., 0.);
        if depth < 50 && rec.mat.as_ref().scatter(r, &rec, &mut attenuation, &mut scattered) {
            let c = color(&scattered, world, depth + 1);
            //println!("mat={}, depth={}, att={:?}, scat={:?}, color={:?}", rec.mat.as_ref().name(), depth, attenuation.clone(), scattered.clone(), c);
            return attenuation * c;
        }
        return Vec3::new(0., 0., 0.);
    }
    let unit_direction = unit_vector(r.direction().clone());
    let t: f32 = 0.5 * (unit_direction.y() + 1.);
    Vec3::new(1., 1., 1.) * (1. - t) + Vec3::new(0.5, 0.7, 1.) * t
}

macro_rules! get_sphere {
    ( $m:ident, $x:expr, $y:expr, $r:expr ) => {
        {
            Box::new(Sphere::new($y, $r, Box::new($m::new($x))))
        }
    };
    ( $m:ident, $x:expr, $f:expr, $y:expr, $r:expr ) => {
        {
            Box::new(Sphere::new($y, $r, Box::new($m::new($x, $f))))
        }
    };
}

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;
    println!("P3\n{} {}\n255", nx, ny);

    let s1 = get_sphere!(Lambertian, Vec3::new(0.8, 0.3, 0.3), Vec3::new(0., 0., -1.), 0.5);
    let s2 = get_sphere!(Lambertian, Vec3::new(0.8, 0.8, 0.), Vec3::new(0., -100.5, -1.), 100.);
    let s3 = get_sphere!(Metal, Vec3::new(0.8, 0.6, 0.2), 1.0, Vec3::new(1., 0., -1.), 0.5);
    let s4 = get_sphere!(Metal, Vec3::new(0.8, 0.8, 0.8), 0.3, Vec3::new(-1., 0., -1.), 0.5);
    let world: HitableList = HitableList { list : vec![s1, s2, s3, s4] };
    let mut cam = Camera::new();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0., 0., 0.);
            for _ in 0..ns {
                let u: f32 = (i as f32 + drand48()) / nx as f32;
                let v: f32 = (j as f32 + drand48()) / ny as f32;
                let r = cam.get_ray(u, v);

                // let p = r.point_at_parameter(2.); // NOTE: not use?

                col = col + color(&r, &world, 0);
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
