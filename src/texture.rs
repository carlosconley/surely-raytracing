use crate::{color::Color, vec3::Point3};

#[derive(Clone)]
pub enum Texture {
	Solid(SolidColor),
	Checker(CheckerTexture),
}

impl Texture {
	pub fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
		match self {
			Texture::Solid(t) => t.value(u, v, p),
			Texture::Checker(t) => t.value(u, v, p)
		}

	}
}


#[derive(Clone)]
pub struct SolidColor {
	color_value: Color
}

impl SolidColor {
	pub fn new(color_value: Color) -> Texture {
		Texture::Solid(SolidColor {
			color_value
		})
	}

	pub fn from_rgb(red: f64, green: f64, blue: f64) -> Texture {
		Texture::Solid(SolidColor {
			color_value: Color::new(red, green, blue)
		})
	}

	pub fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
		self.color_value
	}
}

#[derive(Clone)]
pub struct CheckerTexture {
	inv_scale: f64,
	even: Box<Texture>,
	odd: Box<Texture>
}

impl CheckerTexture {
	pub fn new(scale: f64, even: Box<Texture>, odd: Box<Texture>) -> Texture {
		Texture::Checker(
			CheckerTexture { inv_scale: 1.0 / scale, even, odd }
		)
	}

	pub fn from_color(scale: f64, c1: Color, c2: Color) -> Texture {
		Texture::Checker(
			CheckerTexture {
				inv_scale: 1. / scale,
				even: Box::new(SolidColor::new(c1)),
				odd: Box::new(SolidColor::new(c2)),
			}
		)
	}

	pub fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
		let x = (self.inv_scale * p.x()).floor() as i32;
		let y = (self.inv_scale * p.y()).floor() as i32;
		let z = (self.inv_scale * p.z()).floor() as i32;

		if (x + y + z) % 2 == 0 {
			self.even.value(u, v, p)
		} else {
			self.odd.value(u, v, p)
		}
	}
}


