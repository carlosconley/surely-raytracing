use std::io::Write;
use crate::interval::Interval;
use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color<W: Write>(out: &mut W, pixel_color: &Color, samples_per_pixel: f64) {
	let scale = 1.0 / samples_per_pixel;

	let r = pixel_color.x() * scale;
	let g = pixel_color.y() * scale;
	let b = pixel_color.z() * scale;

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