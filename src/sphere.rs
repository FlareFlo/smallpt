use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ReflectionType {
	/// DIFFuse
	Diff,
	/// SPECular
	Spec,
	/// REFRactive
	Refr,
}

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
	/// radius
	pub rad: f64,

	/// position
	pub p: Vec3,

	/// emission
	pub e: Vec3,

	/// color
	pub c: Vec3,

	pub refl: ReflectionType,
}

impl Sphere {
	pub const fn new(rad: f64, p: Vec3, e: Vec3, c: Vec3, refl: ReflectionType) -> Self {
		Self {
			rad,
			p,
			e,
			c,
			refl,
		}
	}

	/// returns intersection distance, returns 0 when it misses
	#[inline(always)]
	pub fn intersect(&self, r: Ray) -> f64 {
		let op = self.p - r.o; // Solve t^2*d.d + 2*t*(o-p).d + (o-p).(o-p)-R^2 = 0
		let b = op.dot(r.d);
		let mut det = b.powi(2) - op.dot(op) + self.rad.powi(2);

		if det < 0.0 {
			return 0.0
		} else {
			det = det.sqrt();
		}
		let eps = 1e-4;
		if b - det > eps {
			b - det
		} else if b + det > eps {
			b + det
		} else {
			0.0
		}
	}
}