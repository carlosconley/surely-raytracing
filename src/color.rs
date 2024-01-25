use std::io::Write;
use crate::interval::Interval;
use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color<W: Write>(out: &mut W, pixel_color: &Color, samples_per_pixel: f64) {
	let scale = 1.0 / samples_per_pixel;

	let r = pixel_color.x() * scale;
	let g = pixel_color.y() * scale;
	let b = pixel_color.z() * scale;

	let r = linear_to_gamma_fast(r);
	let g = linear_to_gamma_fast(g);
	let b = linear_to_gamma_fast(b);

	let intensity = Interval {
		min: 0.,
		max: 0.999
	};

	writeln!(
		out,
		"{} {} {}",
		(256. * intensity.clamp(r)) as u8,
		(256. * intensity.clamp(g)) as u8,
		(256. * intensity.clamp(b)) as u8
	)
	.expect("Error writing pixel to buffer");


}

const GAMMA: f64 = 1. / 2.4;

fn gamma_to_linear(gamma: f64) -> f64 {
	if gamma <= 0.04045 {
		gamma / 12.92
	} else {
		(gamma + 0.055 / 1.055).powf(2.4)
	}
}

fn linear_to_gamma_fast(linear: f64) -> f64 {
	linear.sqrt()
}

fn linear_to_gamma(linear: f64) -> f64 {
	if linear <= 0.0031308 {
		12.92 * linear
	} else {
		1.055 * linear.powf(GAMMA) - 0.055
	}

}