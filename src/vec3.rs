use std::ops::{Mul, Rem};

#[derive(Debug, Copy, Clone)]
/// Stores positions, but may double as colors too
pub struct Vec3 {
	x: f64,
	y: f64,
	z: f64,
}


/// mult function equivalent
impl Mul for Vec3 {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		Self {
			x: rhs.x * self.x,
			y: rhs.y * self.y,
			z: rhs.z * self.z,
		}
	}
}

/// operator% equivalent
impl Rem for Vec3 {
	type Output = Self;

	fn rem(self, b: Self) -> Self::Output {
		Self {
			x: self.y * b.z - self.z * b.y,
			y: self.z * b.x - self.x * b.z,
			z: self.x * b.y - self.y * b.x,
		}
	}
}

impl Vec3 {
	/// operator* equivalent
	pub fn mul_f(&self, by: f64) -> Self {
		Self {
			x: self.x * by,
			y: self.y * by,
			z: self.z * by,
		}
	}

	/// operator+ equivalent
	pub fn add_f(&self, with: f64) -> Self {
		Self {
			x: self.x + with,
			y: self.y + with,
			z: self.z + with,
		}
	}

	/// operator- equivalent
	pub fn sub_f(&self, with: f64) -> Self {
		Self {
			x: self.x - with,
			y: self.y - with,
			z: self.z - with,
		}
	}

	pub fn norm(&mut self) {
		let len = self.len();
		*self = self.mul_f(1 / len);
	}

	pub fn dot(&self, other: &Self) -> f64 {
		self.x * other.x + self.y * other.y + self.z * other.z
	}

	pub fn len(&self) -> f64 {
		(self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
	}
}