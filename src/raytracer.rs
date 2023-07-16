use std::{env, fs, thread};
use std::fmt::Write as FmtWrite;
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::thread::sleep;
use std::time::{Duration, Instant};

use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

use crate::radiance::radiance;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::Spheres;
use crate::vec3::Vec3;

#[inline(always)]
pub fn clamp(x: f64) -> f64 {
	x.clamp(0.0, 1.0)
}

#[inline(always)]
pub fn to_int(x: f64) -> i32 { (clamp(x).powf(1.0 / 2.2) * 255.0 + 0.5) as i32 }

#[inline(always)]
pub fn erand48() -> f64 {
	fastrand::i32(..).abs() as f64 / i32::MAX as f64
}

#[inline(always)]
pub fn intersect(spheres: Spheres, r: Ray, t: &mut f64, id: &mut usize) -> bool {
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

pub fn render_scene() {
	let rng = erand48;
	let scene = Scene::default();


	// cast buffer into mutex to access it in parallel
	let image_buffer = Vec::from_iter((0..scene.h).map(|_| Mutex::new(vec![Vec3::ZEROES; scene.w])));
	let completed_lines = Arc::new(AtomicUsize::new(0));

	let super_sampling: usize = 2; // Non-zero multiple of two
	let super_sampling_area = super_sampling.pow(2);
	let super_sampling_brightness_factor = 1.0 / super_sampling_area as f64;
	let effective_samps = super_sampling_area * scene.samples;

	let total_sampled_pixels = scene.w * scene.h * super_sampling_area * scene.samples;

	// The progress thread receives a reference to the progress
	let progress_lines = completed_lines.clone();
	let start = Instant::now();
	let progress_thread = thread::spawn({
		move || {
			while progress_lines.load(Relaxed) < scene.h {
				let percentage = 100.0 * progress_lines.load(Relaxed) as f64 / (scene.h as f64 - 1.0);
				let actual_sample_progress = super_sampling_area * scene.samples * scene.w * progress_lines.load(Relaxed);
				let elapsed = start.elapsed().as_secs_f64();

				clearscreen::clear().unwrap();
				println!("Elapsed: {elapsed:.1}s\nTotal Samples required: {}m\nSamples-per-pixel: {effective_samps}\nResolution: {}x{}\nSuperSampling: {super_sampling}x",
						 scene.w,
						 scene.h,
						 total_sampled_pixels / 1_000_000);
				println!("Progress: {percentage:.2}% Samples: {}m  samples/sec: {:.2}m",
						 actual_sample_progress / 1_000_000,
						 actual_sample_progress as f64 / 1_000_000.0 / elapsed,
				);
				sleep(Duration::from_millis(33)); // Only update progress every 30hz
			}
		}
	});

	(0..scene.h).into_par_iter().for_each(|y|
		{
			let mut buf = image_buffer[y].lock().unwrap();
			for x in 0..scene.w {  // Loop cols
				for sy in 0..super_sampling { // 2x2 subpixel rows

					for sx in 0..super_sampling {  // 2x2 subpixel cols
						let mut r = Vec3::ZEROES; // Current radiance
						for _ in 0..scene.samples {
							let r1 = 2.0 * rng();
							let dx = if r1 < 1.0 {
								r1.sqrt() - 1.0
							} else {
								1.0 - (2.0 - r1).sqrt()
							};
							let r2 = 2.0 * rng();
							let dy = if r2 < 1.0 {
								r2.sqrt() - 1.0
							} else {
								1.0 - (2.0 - r2).sqrt()
							};
							let mut d = scene.cx.mul_f(((sx as f64 + 0.5 + dx) / 2.0 + x as f64) / scene.w as f64 - 0.5) +
								scene.cy.mul_f(((sy as f64 + 0.5 + dy) / 2.0 + y as f64) / scene.h as f64 - 0.5) + scene.cam.direction;
							d = d.norm();
							r = r + radiance(&scene.spheres, Ray { origin: scene.cam.origin + d.mul_f(140.0), direction: d }, 0, rng).mul_f(1.0 / scene.samples as f64);
						}
						buf[x] = buf[x] + Vec3::new(clamp(r.x), clamp(r.y), clamp(r.z)).mul_f(super_sampling_brightness_factor);
					}
				}
			}
			completed_lines.fetch_add(1, Relaxed);
		}
	);
	// Ensure the status thread terminates
	completed_lines.store(scene.h, Relaxed);
	progress_thread.join().unwrap();

	println!("Finished after: {:.1}s", start.elapsed().as_secs_f64());

	// Pull buffer out of mutex
	let c = image_buffer.into_iter().map(|e| e.into_inner().unwrap()).rev().flatten();

	if env::var("NO_SAVE").is_ok() {
		exit(0);
	}

	let mut buf = String::new();
	buf.write_str(&format!("P3\n{} {}\n{}\n", scene.w, scene.h, 255)).unwrap();
	for i in c {
		buf.write_str(&format!("{} {} {} ", to_int(i.x), to_int(i.y), to_int(i.z))).unwrap();
	}
	fs::write("image.ppm", buf.into_bytes()).unwrap();
}