use crate::vec3::*;

pub struct Ray {
	orig: Point3,
	dir: Vec3,
}

impl Ray {
	// Constructors
	pub fn new(origin: &Point3, direction: &Vec3) -> Self {
		Self { orig: origin.clone(), dir: direction.clone() }
	}

	// Accessors
	pub fn direction(&self) -> Vec3 {
		self.dir
	}

	pub fn origin(&self) -> Point3 {
		self.orig
	}

	// Utils
	pub fn at(&self, t: f64) -> Vec3 {
		self.orig + t * self.dir
	}

}