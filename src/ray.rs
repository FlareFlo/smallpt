use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
	pub origin: Vec3,
	pub direction: Vec3,
}