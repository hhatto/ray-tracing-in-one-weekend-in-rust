mod utils;

use utils::{vec3, ray};

fn color(r: &ray::Ray) -> vec3::Vec3 {
    let d = r.direction().clone();
    let unit_direction: vec3::Vec3 = vec3::unit_vector(d);
    let t: f32 = 0.5 * (unit_direction.y() + 1.0);
    vec3::Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + vec3::Vec3::new(0.5, 0.7, 1.0) * t
}

fn main() {
    let nx = 200;
    let ny = 100;
    println!("P3\n{} {}\n255", nx, ny);

    let lower_left_corner = vec3::Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = vec3::Vec3::new(4.0, 0.0, 0.0);
    let vertical = vec3::Vec3::new(0.0, 2.0, 0.0);
    let origin = vec3::Vec3::new(0.0, 0.0, 0.0);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u: f32 = i as f32 / nx as f32;
            let v: f32 = j as f32 / ny as f32;
            let hu: vec3::Vec3 = horizontal.clone() * u;
            let vv: vec3::Vec3 = vertical.clone() * v;
            let llc = lower_left_corner.clone() + hu + vv;
            let r = ray::Ray::new(&origin, &llc);

            let col = color(&r);
            let ir = (255.99 * col.e.0) as i32;
            let ig = (255.99 * col.e.1) as i32;
            let ib = (255.99 * col.e.2) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
