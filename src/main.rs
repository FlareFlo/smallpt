use std::sync::Once;
use crate::radiance::radiance;
use crate::ray::Ray;
use crate::sphere::{ReflectionType, Sphere};
use crate::vec3::Vec3;

mod vec3;
mod sphere;
mod ray;
mod radiance;

fn clamp(x: f64) -> f64 {
   x.clamp(0.0, 1.0)
}

fn to_int(x: f64) -> usize { (clamp(x).powf(1.0 / 2.2) * 255.0 + 0.5) as _ }

fn erand48() -> f64 {
    fastrand::i32(..) as f64 / i32::MAX as f64
}

fn intersect(spheres: &[Sphere], r: Ray, t: &mut f64, id: &mut usize) -> bool {
    let n = spheres.len();
    let inf = 1e20;
    *t = inf;
    for i in 0..n {
        let d = spheres[i].intersect(r);
        if d != 0.0 && d < *t {
            *t = d;
            *id = i as _;
        }
    }
    return *t < inf;
}

fn main() {
    let spheres = {&[
        Sphere::new(1e5, Vec3::new(1e5 +  1.0, 40.8, 81.6), Vec3::ZEROES, Vec3::new(0.75, 0.25, 0.25), ReflectionType::Diff),//Left
        Sphere::new(1e5, Vec3::new(-1e5 + 99.0, 40.8, 81.6), Vec3::ZEROES, Vec3::new(0.25, 0.25, 0.75), ReflectionType::Diff),//Rght
        Sphere::new(1e5, Vec3::new(50.0, 40.8, 1e5), Vec3::ZEROES, Vec3::new(0.75, 0.75, 0.75), ReflectionType::Diff),//Back
        Sphere::new(1e5, Vec3::new(50.0, 40.8, -1e5 + 170.0), Vec3::ZEROES, Vec3::ZEROES, ReflectionType::Diff),//Frnt
        Sphere::new(1e5, Vec3::new(50.0, 1e5, 81.6), Vec3::ZEROES, Vec3::new(0.75, 0.75, 0.75), ReflectionType::Diff),//Botm
        Sphere::new(1e5, Vec3::new(50.0, -1e5 + 81.6, 81.6), Vec3::ZEROES, Vec3::new(0.75, 0.75, 0.75), ReflectionType::Diff),//Top
        Sphere::new(16.5, Vec3::new(27.0, 16.5, 47.0), Vec3::ZEROES, Vec3::new(1.0, 1.0, 1.0).mul_f(  0.999), ReflectionType::Spec),//Mirr
        Sphere::new(16.5, Vec3::new(73.0, 16.5, 78.0), Vec3::ZEROES, Vec3::new(1.0, 1.0, 1.0).mul_f(0.999), ReflectionType::Refr),//Glas
        Sphere::new(600.0, Vec3::new(50.0, 681.6 - 0.27, 81.6), Vec3::new(12.0, 12.0, 12.0), Vec3::ZEROES, ReflectionType::Diff), //Lite
    ]};

    let w = 1024;
    let h = 768;
    let samps = 1; // TODO: Fetch argument

    let cam = Ray {
        o: Vec3 {
            x: 50.0,
            y: 52.0,
            z: 295.6,
        },
        d: Vec3 {
            x: 0.0,
            y: -0.042612,
            z: -1.0,
        }.norm()
    };
    let cx = Vec3::new(w as f64 * 0.6135 / h as f64, 0.0, 0.0);
    let cy = (cx % cam.d).norm().mul_f(0.5135);
    let mut r = Vec3::ZEROES;
    let mut c = vec![Vec3::ZEROES; w * h];
    let mut y = 0;
    while y < h {
        println!("Rendering at {} samples: {}%", samps * 4, 100 * y / (h - 1));

        let mut x = 0;
        while x < w {
            let mut sy = 0;
            while sy < 2 {
                let i = (h - y - 1) * w + x;

                let mut sx = 0;
                while sx < 2 {
                    r = Vec3::ZEROES;
                    for _ in 0..samps {
                        let r1 = 2.0 * erand48();
                        let dx = if r1 < 1.0 {
                            r1.sqrt() - 1.0
                        } else {
                            1.0 - (2.0 - r1).sqrt()
                        };
                        let r2 = 2.0 * erand48();
                        let dy = if r2 < 1.0 {
                            r2.sqrt() - 1.0
                        } else {
                            1.0 - (2.0 - r2).sqrt()
                        };
                        let d = cx.mul_f(((sx as f64 + 0.5 + dx) / 2.0 + x as f64) / w as f64 - 0.5) +
                                        cy.mul_f(((sy as f64 + 0.5 + dy as f64) / 2.0 + y as f64) / h as f64 - 0.5 ) + cam.d;
                        r = r + radiance(spheres, Ray { o: cam.o + d.mul_f(140.0), d: d.norm() }, 0).mul_f(1.0 / samps as f64);
                    }
                    c[i] = c[i] + Vec3::new(clamp(r.x), clamp(r.y), clamp(r.z)).mul_f(0.25);

                    sx += 1;
                }

                sy += 1;
            }


            x += 1;
        }

        y += 1;
    }

}
