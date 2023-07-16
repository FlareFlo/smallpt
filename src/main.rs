use crate::sphere::Sphere;
use crate::worker_mode::WorkerMode;

mod vec3;
mod sphere;
mod ray;
mod radiance;
mod parallel_compute;
mod raytracer;
mod scene;
mod worker_mode;

pub const PORT: u16 = 12346; // "smallpt" to decimal % 2^16 = 28788


pub type Spheres<'a> = &'a [Sphere];


fn main() {
	let _worker_mode = WorkerMode::from_env().unwrap();
}
