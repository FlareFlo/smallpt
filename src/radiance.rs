use std::f64::consts::PI;

use crate::{erand48, intersect, Spheres};
use crate::ray::Ray;
use crate::sphere::{ReflectionType, Sphere};
use crate::vec3::Vec3;

pub fn radiance(spheres: Spheres, r: Ray, mut depth: i32) -> Vec3 {
	let mut distance_to_intersection = 1e20;
	let mut to_intersect_object_id = 0_usize;
	if !intersect(spheres, r, &mut distance_to_intersection, &mut to_intersect_object_id) {
		return Vec3::ZEROES;
	}

	// object currently being measured
	let obj = spheres[to_intersect_object_id];
	let x = r.o + r.d.mul_f(distance_to_intersection);
	let n = (x - obj.p).norm();
	let nl = if n.dot(r.d) < 0.0 { n } else { n.mul_f(-1.0) };
	let mut f = obj.c;
	let p = if f.x > f.y && f.x > f.z {
		f.x
	} else if f.y > f.z {
		f.y
	} else {
		f.z
	};

	depth += 1;
	if (depth) > 5 {
		if erand48() < p {
			f = f.mul_f(1.0 / p);
		} else {
			return obj.e;
		}
	}

	if obj.refl == ReflectionType::Diff {
		let r1 = 2.0 * PI * erand48();
		let r2 = erand48();
		let r2s = r2.sqrt();
		let w = nl;
		let u = if w.x.abs() > 0.1 {
			Vec3::new(0.0, 1.0, 0.0)
		} else {
			Vec3::new(1.0, 0.0, 0.0) % w
		}.norm();
		let v = w % u;
		let d = (u.mul_f(r1.cos() * r2s) + v.mul_f(r1.sin() * r2s) + w.mul_f((1.0 - r2).sqrt())).norm();
		// Ideal DIFFUSE reflection
		return obj.e + f * radiance(spheres, Ray { o: x, d }, depth);
	} else if obj.refl == ReflectionType::Spec {
		// Ideal SPECULAR reflection
		return obj.e + f * radiance(spheres, Ray { o: x, d: r.d - n.mul_f(2.0 * n.dot(r.d)) }, depth);
	}
	// Ideal dielectric REFRACTION
	let refl_ray = Ray {
		o: x,
		d: r.d - n.mul_f(2.0 * n.dot(r.d)),
	};

	// Ray from outside going in?
	let into = n.dot(nl) > 0.0;
	let nc = 1.0;
	let nt = 1.5;
	let nnt = if into { nc / nt } else { nt / nc };
	let ddn = r.d.dot(nl);
	let cos2t = 1.0 - nnt * nnt * (1.0 - ddn.powi(2));
	if cos2t < 0.0 {
		return obj.e + f * radiance(spheres, refl_ray, depth);
	}
	let tdir = (r.d.mul_f(nnt) - n.mul_f(if into { 1.0 } else { -1.0 } * (ddn * nnt + cos2t.sqrt()))).norm();
	let a = nt - nc;
	let b = nt + nc;
	let R0 = a.powi(2) / b.powi(2);
	let c = 1.0 - if into { -ddn } else { tdir.dot(n) };
	let Re = R0 + (1.0 - R0) * c * c * c * c * c;
	let Tr = 1.0 - Re;
	let P = 0.25 + 0.5 * Re;
	let RP = Re / P;
	let TP = Tr / (1.0 - P);
	return obj.e + f * {
		if depth > 2 {
			if erand48() < P {
				radiance(spheres,refl_ray, depth).mul_f(RP)
			} else {
				radiance(spheres, Ray { o: x, d: tdir }, depth)
			}
		} else {
			radiance(spheres, refl_ray, depth).mul_f(Re) + radiance(spheres, Ray { o: x, d: tdir}, depth).mul_f(Tr)
		}
	}
}