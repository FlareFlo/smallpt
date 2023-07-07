use std::f64::consts::PI;

use crate::{intersect, Spheres};
use crate::ray::Ray;
use crate::sphere::ReflectionType;
use crate::vec3::Vec3;


pub fn radiance(spheres: Spheres, mut r: Ray, mut depth: i32, rng: fn() -> f64) -> Vec3 {
	let mut stack: Vec<(Ray, i32, Vec3)> = Vec::new();
	let mut result = Vec3::zero();
	let mut throughput = Vec3::one();

	loop {
		let mut distance_to_intersection = 1e20;
		let mut to_intersect_object_id = 0_usize;

		if intersect(spheres, r, &mut distance_to_intersection, &mut to_intersect_object_id) {
			let obj = spheres[to_intersect_object_id];
			let x = r.origin + r.direction.mul_f(distance_to_intersection);
			let n = (x - obj.position).norm();
			let nl = if n.dot(r.direction) < 0.0 { n } else { n.mul_f(-1.0) };
			let mut f = obj.color;
			let p = f.x.max(f.y).max(f.z);

			depth += 1;
			if depth > 5 {
				if rng() >= p {
					result += throughput.mul_v(obj.emission);
					break;
				}
				f = f.mul_f(1.0 / p);
			}

			if obj.refl == ReflectionType::Diff {
				let r1 = 2.0 * PI * rng();
				let r2 = rng();
				let r2s = r2.sqrt();
				let w = nl;
				let u = if w.x.abs() > 0.1 {
					Vec3::new(0.0, 1.0, 0.0)
				} else {
					Vec3::new(1.0, 0.0, 0.0) % w
				}.norm();
				let v = w % u;
				let d = (u.mul_f(r1.cos() * r2s) + v.mul_f(r1.sin() * r2s) + w.mul_f((1.0 - r2).sqrt())).norm();

				stack.push((Ray { origin: x, direction: d }, depth, throughput.mul_v(obj.emission)));
				r = Ray { origin: x, direction: d };
				throughput = throughput.mul_v(f);
			} else if obj.refl == ReflectionType::Spec {
				r = Ray { origin: x, direction: r.direction - n.mul_f(2.0 * n.dot(r.direction)) };
				throughput = throughput.mul_v(f);
			} else {
				let refl_ray = Ray { origin: x, direction: r.direction - n.mul_f(2.0 * n.dot(r.direction)) };
				let into = n.dot(nl) > 0.0;
				let nc = 1.0;
				let nt = 1.5;
				let nnt = if into { nc / nt } else { nt / nc };
				let ddn = r.direction.dot(nl);
				let cos2t = 1.0 - nnt * nnt * (1.0 - ddn.powi(2));

				if cos2t < 0.0 {
					r = refl_ray;
					throughput = throughput.mul_v(f);
				} else {
					let tdir = (r.direction.mul_f(nnt) - n.mul_f(if into { 1.0 } else { -1.0 } * (ddn * nnt + cos2t.sqrt()))).norm();
					let a = nt - nc;
					let b = nt + nc;
					let R0 = a.powi(2) / b.powi(2);
					let c = 1.0 - if into { -ddn } else { tdir.dot(n) };
					let Re = R0 + (1.0 - R0) * c * c * c * c * c;
					let Tr = 1.0 - Re;
					let P = 0.25 + 0.5 * Re;
					let RP = Re / P;
					let TP = Tr / (1.0 - P);

					if depth > 2 {
						if rng() < P {
							r = refl_ray;
							throughput = throughput.mul_f(RP);
						} else {
							r = Ray { origin: x, direction: tdir };
							throughput = throughput.mul_f(TP);
						}
					} else {
						stack.push((Ray { origin: x, direction: tdir }, depth, throughput.mul_f(Tr)));
						r = refl_ray;
						throughput = throughput.mul_f(Re);
					}
				}
			}
		} else if let Some((prev_ray, prev_depth, prev_throughput)) = stack.pop() {
			r = prev_ray;
			depth = prev_depth;
			throughput = prev_throughput;
		} else {
			break;
		}
	}

	result
}

#[cfg(test)]
mod test {
	#[test]
	fn parity_pre_recursive_refactor() {}
}