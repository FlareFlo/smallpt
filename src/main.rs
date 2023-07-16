use std::env::args;
use std::str::FromStr;
use std::{env, fs, thread};
use std::fmt::Write as FmtWrite;
use std::io::{Read, Write};
use std::mem::size_of;
use std::net::{Ipv4Addr, TcpListener, TcpStream};
use std::process::exit;
use std::sync::{Arc, Mutex, Once};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::atomic::Ordering::Relaxed;
use std::thread::sleep;
use std::time::{Duration, Instant};
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use crate::radiance::radiance;
use crate::ray::Ray;
use crate::raytracer::erand48;
use crate::scene::{get_spheres, Scene};
use crate::sphere::{ReflectionType, Sphere};
use crate::vec3::Vec3;
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
    let worker_mode = WorkerMode::from_env().unwrap();


}
