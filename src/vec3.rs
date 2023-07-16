use std::ops::{Add, AddAssign, Mul, Rem, Sub};

#[derive(Debug, Copy, Clone)]
/// Stores positions, but may double as colors too
pub struct Vec3 {
	pub x: f64,
	pub y: f64,
	pub z: f64,
}


/// mult function equivalent
impl Mul for Vec3 {
	type Output = Self;

	#[inline(always)]
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

	#[inline(always)]
	fn rem(self, b: Self) -> Self::Output {
		Self {
			x: self.y * b.z - self.z * b.y,
			y: self.z * b.x - self.x * b.z,
			z: self.x * b.y - self.y * b.x,
		}
	}
}

impl Sub for Vec3 {
	type Output = Self;

	#[inline(always)]
	fn sub(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
			z: self.z - rhs.z,
		}
	}
}

impl Add for Vec3 {
	type Output = Self;

	#[inline(always)]
	fn add(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
			z: self.z + rhs.z,
		}
	}
}

impl AddAssign for Vec3 {
	fn add_assign(&mut self, rhs: Self) {
		*self = *self + rhs;
	}
}

impl Vec3 {
	pub const ZEROES: Self = Self {
		x: 0.0,
		y: 0.0,
		z: 0.0,
	};

	pub const fn one() -> Self {
		Self {
			x: 1.0,
			y: 1.0,
			z: 1.0,
		}
	}
	pub const fn zero() -> Self {
		Self {
			x: 0.0,
			y: 0.0,
			z: 0.0,
		}
	}

	pub const fn new(x: f64, y: f64, z: f64) -> Self { Self { x, y, z } }

	/// operator* equivalent
	#[inline(always)]
	pub fn mul_f(self, by: f64) -> Self {
		Self {
			x: self.x * by,
			y: self.y * by,
			z: self.z * by,
		}
	}

	/// operator* equivalent
	#[inline(always)]
	pub fn mul_v(self, rhs: Self) -> Self {
		self * rhs
	}

	/// operator+ equivalent
	#[inline(always)]
	pub fn add_f(self, with: f64) -> Self {
		Self {
			x: self.x + with,
			y: self.y + with,
			z: self.z + with,
		}
	}

	#[inline(always)]
	pub fn norm(self) -> Self {
		let len = self.len();
		self.mul_f(1.0 / len)
	}


	#[inline(always)]
	pub fn dot(self, other: Self) -> f64 {
		self.x * other.x + self.y * other.y + self.z * other.z
	}

	#[inline(always)]
	pub fn len(self) -> f64 {
		(self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
	}
}