use std::f64::consts::E;
use std::io::Write;
use crate::interval::Interval;
use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color<W: Write>(out: &mut W, pixel_color: &Color, samples_per_pixel: f64, exposure_val: Option<f64>) {
	let scale = 1.0 / samples_per_pixel;

	let intensity = Interval {
		min: 0.,
		max: 0.999
	};

	let rgb = [pixel_color.x(), pixel_color.y(), pixel_color.z()];

	let rgb = rgb.map(|x| x * scale).map(|linear| {
		match exposure_val {
			Some(val) => exposure(linear, val),
			None => linear
		}
	}).map(linear_to_gamma).map(|rgb| (256. * intensity.clamp(rgb)) as u8);

	writeln!(
		out,
		"{} {} {}", rgb[0], rgb[1], rgb[2]
	)
	.expect("Error writing pixel to buffer");


}

//const _GAMMA: f64 = 1. / 2.4;

fn exposure(linear:f64, v: f64) -> f64 {
	1. - E.powf(-v * linear)
}

fn _gamma_to_linear(gamma: f64) -> f64 {
	if gamma <= 0.04045 {
		gamma / 12.92
	} else {
		(gamma + 0.055 / 1.055).powf(2.4)
	}
}

fn _linear_to_gamma_fast(linear: f64) -> f64 {
	linear.sqrt()
}

fn linear_to_gamma(linear: f64) -> f64 {
	if linear <= 0.0031308 {
		12.92 * linear
	} else {
		1.055 * linear.powf(1. / 2.4) - 0.055
	}

}