#[macro_use]
mod utils;

use std::thread;
use std::sync::Arc;
use std::sync::mpsc::{channel, Receiver, Sender};
use utils::vec3::{Vec3, unit_vector};
use utils::ray::Ray;
use utils::hitable::{Hitable, HitableList, HitRecord};
use utils::sphere::Sphere;
use utils::camera::Camera;
use utils::material::{DummyMat, Lambertian, Metal, Dielectric};
use utils::random::drand48;

const CONCURRENCY: usize = 4;

fn random_scene() -> HitableList {
    let mut list = HitableList::new(vec![]);
    list.list.push(get_sphere!(Lambertian, Vec3::new(0.5, 0.5, 0.5), Vec3::new(0., -1000., 0.), 1000.));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = drand48();
            let center = Vec3::new(a as f32 + 0.9 * drand48(), 0.2, b as f32 * 0.9 * drand48());
            if (center.clone() - Vec3::new(4., 0.2, 0.)).len() > 0.9 {
                if choose_mat < 0.8 {
                    list.list.push(get_sphere!(Lambertian, Vec3::new(drand48() * drand48(), drand48() * drand48(), drand48() * drand48()), center, 0.2));
                } else if choose_mat < 0.95 {
                    list.list.push(get_sphere!(Metal, Vec3::new(0.5 * (1. + drand48()), 0.5 * (1. + drand48()), 0.5 * (1. + drand48())), 0.5 * drand48(), center, 0.2));
                } else {
                    list.list.push(get_sphere!(Dielectric, 1.5, center, 0.2));
                }
            }
        }
    }

    list.list.push(get_sphere!(Dielectric, 1.5, Vec3::new(0., 1., 0.), 1.));
    list.list.push(get_sphere!(Lambertian, Vec3::new(0.4, 0.2, 0.1), Vec3::new(-4., 1., 0.), 1.));
    list.list.push(get_sphere!(Metal, Vec3::new(0.7, 0.6, 0.5), 0., Vec3::new(4., 1., 0.), 1.));

    list
}

fn color(r: &Ray, world: &Arc<HitableList>, depth: i32) -> Vec3 {
    let mut rec = HitRecord::new(Box::new(DummyMat::new()));
    if world.hit(r, 0.001, std::f32::MAX, &mut rec) {
        let v1 = Vec3::new(0., 0., 0.);
        let v2 = Vec3::new(0., 0., 0.);
        let mut scattered = Ray::new(&v1, &v2);
        let mut attenuation = Vec3::new(0., 0., 0.);
        if depth < 50 && rec.mat.as_ref().scatter(r, &rec, &mut attenuation, &mut scattered) {
            return attenuation * color(&scattered, world, depth + 1);
        }
        return Vec3::new(0., 0., 0.);
    }
    let unit_direction = unit_vector(r.direction().clone());
    let t: f32 = 0.5 * (unit_direction.y() + 1.);
    Vec3::new(1., 1., 1.) * (1. - t) + Vec3::new(0.5, 0.7, 1.) * t
}

fn get_color(i: f32, j: f32, nx: f32, ny: f32, cam: &Arc<Camera>, world: &Arc<HitableList>) -> Vec3 {
    let u: f32 = (i as f32 + drand48()) / nx as f32;
    let v: f32 = (j as f32 + drand48()) / ny as f32;
    let r = cam.get_ray(u, v);
    color(&r, world, 0)
}

fn exec_worker(cam: &Arc<Camera>, world: &Arc<HitableList>, rx: Receiver<Option<(f32, f32, f32, f32)>>, cx: Sender<Option<Vec3>>) {
    loop {
        match rx.recv().unwrap() {
            Some(arg) => {
                let r = get_color(arg.0, arg.1, arg.2, arg.3, cam, world);
                cx.send(Some(r)).unwrap();
            }
            None => {
                // kill thread
                return
            }
        }
    }
}

fn main() {
    let nx = 2400;
    let ny = 1600;
    let ns = 100;
    println!("P3\n{} {}\n255", nx, ny);

    let world: HitableList = random_scene();

    let lookfrom = Vec3::new(13., 2., 3.);
    let lookat = Vec3::new(0., 0., -1.);
    let dist_to_focus = 10.;
    let aperture: f32 = 0.1;
    let cam = Camera::with_focus(&lookfrom,
                                     &lookat,
                                     &Vec3::new(0., 1., 0.),
                                     30.,
                                     nx as f32 / ny as f32,
                                     aperture,
                                     dist_to_focus);
    let mut workers = vec![];
    let mut handles = vec![];
    let world_arc = Arc::new(world);
    let cam_arc = Arc::new(cam);
    let (calc_tx, calc_rx) = channel::<Option<Vec3>>();

    for _ in 0..CONCURRENCY {
        let world = world_arc.clone();
        let cam = cam_arc.clone();
        let (worker_tx, worker_rx) = channel::<Option<(f32, f32, f32, f32)>>();
        workers.push(worker_tx.clone());
        let c_tx = calc_tx.clone();
        handles.push(thread::spawn(move || { exec_worker(&cam, &world, worker_rx, c_tx) }));
    }

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0., 0., 0.);
            for cnt in 0..ns {
                let offset = cnt % CONCURRENCY;
                let req = workers[offset].clone();
                req.send(Some((i as f32, j as f32, nx as f32, ny as f32))).unwrap();
            }
            
            for _ in 0..ns {
                match calc_rx.recv().unwrap() {
                    Some(ret) => { col = col + ret },
                    None => break
                }
            }
            col = col / ns as f32;
            col = Vec3::new(col.e.0.sqrt(), col.e.1.sqrt(), col.e.2.sqrt());
            let ir = (255.99 * col.e.0) as i32;
            let ig = (255.99 * col.e.1) as i32;
            let ib = (255.99 * col.e.2) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }

    for worker in workers {
        let req = worker.clone();
        req.send(None).unwrap();
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
