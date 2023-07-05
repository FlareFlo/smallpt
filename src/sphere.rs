use crate::vec3::Vec3;

enum ReflectionType {
	/// DIFFuse
	Diff,
	/// SPECular
	Spec,
	/// REFRactive
	Refr,
}

struct Sphere {
	/// radius
	rad: f64,

	/// position
	p: Vec3,

	/// emission
	e: Vec3,

	/// color
	c: Vec3,

	refl: ReflectionType,
}