use std::env::args;
use std::str::FromStr;
use crate::ray::Ray;
use crate::sphere::{ReflectionType, Sphere};
use crate::vec3::Vec3;

pub struct Scene {
	pub w: usize,
	pub h: usize,
	pub samples: usize,
	pub aliasing: usize,
	pub spheres: Vec<Sphere>,
	pub cam: Ray,
	pub cx: Vec3,
	pub cy: Vec3,
}

impl Default for Scene {
	fn default() -> Self {
		let cam = Ray {
			origin: Vec3 {
				x: 50.0,
				y: 52.0,
				z: 295.6,
			},
			direction: Vec3 {
				x: 0.0,
				y: -0.042612,
				z: -1.0,
			}.norm(),
		};
		let w = 1024;
		let h = 768;
		let cx = Vec3::new(w as f64 * 0.5135 / h as f64, 0.0, 0.0);
		Self {
			w,
			h,
			samples: if let Some(samp_string) = args().skip(1).next() {
				usize::from_str(&samp_string).unwrap()
			} else {
				64
			},
			aliasing: 2,
			spheres: get_spheres().to_vec(),
			cam,
			cx,
			cy: (cx % cam.direction).norm().mul_f(0.5135),
		}
	}
}


#[inline(always)]
pub fn get_spheres() -> [Sphere; 9] {
	[
		Sphere::new(1e5, Vec3::new(1e5 +  1.0, 40.8, 81.6), Vec3::ZEROES, Vec3::new(0.75, 0.25, 0.25), ReflectionType::Diff),//Left
		Sphere::new(1e5, Vec3::new(-1e5 + 99.0, 40.8, 81.6), Vec3::ZEROES, Vec3::new(0.25, 0.25, 0.75), ReflectionType::Diff),//Rght
		Sphere::new(1e5, Vec3::new(50.0, 40.8, 1e5), Vec3::ZEROES, Vec3::new(0.75, 0.75, 0.75), ReflectionType::Diff),//Back
		Sphere::new(1e5, Vec3::new(50.0, 40.8, -1e5 + 170.0), Vec3::ZEROES, Vec3::ZEROES, ReflectionType::Diff),//Frnt
		Sphere::new(1e5, Vec3::new(50.0, 1e5, 81.6), Vec3::ZEROES, Vec3::new(0.75, 0.75, 0.75), ReflectionType::Diff),//Botm
		Sphere::new(1e5, Vec3::new(50.0, -1e5 + 81.6, 81.6), Vec3::ZEROES, Vec3::new(0.75, 0.75, 0.75), ReflectionType::Diff),//Top
		Sphere::new(16.5, Vec3::new(27.0, 16.5, 47.0), Vec3::ZEROES, Vec3::new(1.0, 1.0, 1.0).mul_f(  0.999), ReflectionType::Spec),//Mirr
		Sphere::new(16.5, Vec3::new(73.0, 16.5, 78.0), Vec3::ZEROES, Vec3::new(1.0, 1.0, 1.0).mul_f(0.999), ReflectionType::Refr),//Glas
		Sphere::new(600.0, Vec3::new(50.0, 681.6 - 0.27, 81.6), Vec3::new(12.0, 12.0, 12.0), Vec3::ZEROES, ReflectionType::Diff), //Lite
	]
}