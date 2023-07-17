use std::net::TcpListener;

use get_port::Ops;
use mdns_sd::ServiceInfo;

use crate::sphere::Sphere;
use crate::worker_mode::{run_worker_mode, WorkerMode};

mod vec3;
mod sphere;
mod ray;
mod radiance;
mod parallel_compute;
mod raytracer;
mod scene;
mod worker_mode;

pub type Spheres<'a> = &'a [Sphere];


fn main() {
	run_worker_mode();
}
