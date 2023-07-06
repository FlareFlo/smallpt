use std::env::args;
use std::str::FromStr;
use std::fmt::Write;
use std::{env, fs, thread};
use std::process::exit;
use std::sync::{Arc, Mutex, Once};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread::sleep;
use std::time::{Duration, Instant};
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
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

fn to_int(x: f64) -> i32 { (clamp(x).powf(1.0 / 2.2) * 255.0 + 0.5) as i32 }

fn erand48() -> f64 {
    fastrand::i32(0..) as f64 / i32::MAX as f64
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
    let samps = if let Some(samp_string) = args().skip(1).next() {
        usize::from_str(&samp_string).unwrap()
    } else {
        64
    };

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
    let cx = Vec3::new(w as f64 * 0.5135 / h as f64, 0.0, 0.0);
    let cy = (cx % cam.d).norm().mul_f(0.5135);

    // cast buffer into mutex to access it in parallel
    let mut image_buffer = Mutex::new(vec![Vec3::ZEROES; w * h]);
    let completed_lines = Arc::new(AtomicUsize::new(0));

    // The progress thread receives a reference to the progress
    let progress_lines = completed_lines.clone();
    let progress_thread = thread::spawn({
        move ||{
            while progress_lines.load(Ordering::Relaxed) < h {
                let percentage = 100.0 * progress_lines.load(Ordering::Relaxed) as f64 / (h as f64 - 1.0);
                // let percentage_left = 100.0 - percentage;
                clearscreen::clear().unwrap();
                println!("Rendering at {} samples: {percentage:.1}%", samps * 4);
                sleep(Duration::from_millis(33)); // Only update progress every 30hz
            }
        }
    });

    let start = Instant::now();
     (0..h).into_par_iter().for_each(|y|
        {
            for x in 0..w {  // Loop cols
                let i = (h - y - 1) * w + x; // Current pixel index
                for sy in 0..2 { // 2x2 subpixel rows

                    for sx in 0..2{  // 2x2 subpixel cols
                        let mut r = Vec3::ZEROES; // Current radiance
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
                                cy.mul_f(((sy as f64 + 0.5 + dy) / 2.0 + y as f64) / h as f64 - 0.5 ) + cam.d;
                            r = r + radiance(spheres, Ray { o: cam.o + d.mul_f(140.0), d: d.norm() }, 0).mul_f(1.0 / samps as f64);
                        }
                        let mut image_buffer = image_buffer.lock().unwrap();
                        image_buffer[i] = image_buffer[i] + Vec3::new(clamp(r.x), clamp(r.y), clamp(r.z)).mul_f(0.25);
                    }
                }
            }
            completed_lines.fetch_add(1, Ordering::Relaxed);
        }
    );
    // Ensure the status thread terminates
    completed_lines.store(h, Ordering::Relaxed);
    progress_thread.join().unwrap();

    // A little bit of cheating,
    // as it is not guaranteed that the last thread prints its progress
    // before another has already finished, so we simply assert it has completed
    println!("Rendering at {} samples: 100%", samps * 4);
    println!("Took: {:?}", start.elapsed());

    // Pull buffer out of mutex
    let c = image_buffer.into_inner().unwrap();

    if env::var("NO_SAVE").is_ok() {
        exit(0);
    }

    let mut buf = String::new();
    buf.write_str(&format!("P3\n{} {}\n{}\n", w,h,255)).unwrap();
    for i in c {
        buf.write_str(&format!("{} {} {} ", to_int(i.x), to_int(i.y), to_int(i.z))).unwrap();
    }
    fs::write("image.ppm", buf.into_bytes()).unwrap();
}
