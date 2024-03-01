use crate::vec3::*;

pub struct Ray {
	orig: Point3,
	dir: Vec3,
	tm: f64,
}

impl Ray {
	// Constructors
	pub fn new_timed(origin: Point3, direction: Vec3, time: f64) -> Self {
		Self { orig: origin, dir: direction, tm: time }
	}

	pub fn new(origin: Point3, direction: Vec3) -> Self {
		Self { orig: origin, dir: direction, tm: 0. }
	}

	// Accessors
	pub fn direction(&self) -> Vec3 {
		self.dir
	}

	pub fn origin(&self) -> Point3 {
		self.orig
	}

	pub fn time(&self) -> f64 {
		self.tm
	}

	// Utils
	pub fn at(&self, t: f64) -> Vec3 {
		self.orig + t * self.dir
	}

}