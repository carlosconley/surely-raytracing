use std::io::Write;
use std::ops;
use crate::utils::{random_double, random_range};

//extern crate nalgebra as na;
//use na::Vector3;

//pub type Vec3 = Vector3<f64>;
pub type Point3 = Vec3;

/*pub fn print<W: Write>(out: &mut W, v: &Vec3) {
	write!(
		out,
		"{} {} {}",
		v.x, v.y, v.z
	).expect("Couldn't print vector");
}*/


#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
	x: f64,
	y: f64,
	z: f64,
}

impl Vec3 {
	// Constructors
	pub fn new(x: f64, y: f64, z: f64) -> Self {
		Vec3 { x, y, z }
	}
	
	pub fn new_zero() -> Self {
		Vec3 {x: 0., y: 0., z: 0. }
	}

	/*pub fn clone(&self) -> Self {
		Vec3::new(self.x, self.y, self.z)
	}*/

	// Accesors
	pub fn x(&self) -> f64 { self.x }
	pub fn y(&self) -> f64 { self.y }
	pub fn z(&self) -> f64 { self.z }

	pub fn length_squared(&self) -> f64 {
		return self.x * self.x + self.y * self.y + self.z * self.z;
	}
	
	pub fn length(&self) -> f64 {
		self.length_squared().sqrt()
	}

	pub fn print<W: Write>(&self, out: &mut W) {
		write!(
			out,
			"{} {} {}",
			self.x, self.y, self.z
		).expect("Vec3 could not be print");
	}
}

// Operator overloads
impl ops::Add<Vec3> for Vec3 {
	type Output = Vec3;

	fn add(self, v: Vec3) -> Vec3 {
		Vec3::new(self.x + v.x, self.y + v.y, self.z + v.z)
	}
}

impl ops::Neg for Vec3 {
	type Output = Vec3;

	fn neg(self) -> Vec3 {
		Vec3::new(-self.x, -self.y, -self.z)
	}
}

impl ops::Sub<Vec3> for Vec3 {
	type Output = Vec3;

	fn sub(self, v: Vec3) -> Vec3 {
		Vec3::new(self.x - v.x, self.y - v.y, self.z - v.z)
	}
}

impl ops::Mul<Vec3> for Vec3 {
	type Output = Vec3;

	fn mul(self, v: Vec3) -> Vec3 {
		Vec3::new(self.x * v.x, self.y * v.y, self.z * self.z)
	}
}

impl ops::Mul<f64> for Vec3 {
	type Output = Vec3;

	fn mul(self, t: f64) -> Vec3 {
		Vec3::new(self.x * t, self.y * t, self.z * t)
	}
}

impl ops::Mul<Vec3> for f64 {
	type Output = Vec3;

	fn mul(self, v: Vec3) -> Vec3 {
		v * self
	}
}

impl ops::Div<Vec3> for Vec3 {
	type Output = Vec3;
	
	fn div(self, v: Vec3) -> Vec3 {
		Vec3::new(self.x / v.x, self.y / v.y, self.z / v.z)
	}
}

impl ops::Div<f64> for Vec3 {
	type Output = Vec3;

	fn div(self, t: f64) -> Vec3 {
		Vec3::new(self.x / t, self.y / t, self.z / t)
	}
}

impl ops::Div<Vec3> for f64 {
	type Output = Vec3;

	fn div(self, v: Vec3) -> Vec3 {
		v / self
	}
}
// Utils

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
	u.x * v.x + u.y * v.y + u.z * v.z
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
	Vec3::new(
		u.y * v.z - u.z * v.y,
		u.z * v.x - u.x * u.z,
		u.x * v.y - u.y * v.z
	)
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
	v.clone() / v.length()
}

pub fn random_vec3() -> Vec3 {
	Vec3::new(random_double(), random_double(), random_double())
}

pub fn random_vec3_range(min: f64, max: f64) -> Vec3 {
	Vec3::new(random_range(min, max), random_range(min, max), random_range(min, max))
}

