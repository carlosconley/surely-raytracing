
use crate::interval::Interval;
use crate::color::{Color, write_color};
use crate::ray::Ray;
use crate::vec3::{unit_vector, Point3, Vec3, cross, random_in_unit_disk};
use crate::hittable::{Hittable, HittableList};
use crate::utils::{random_double, INF};
use crate::material::MatFn;
use rayon::prelude::*;

pub struct Camera {
	pub aspect_ratio: f64,
	pub image_width: i32,
	pub samples_per_pixel: i32,
	pub max_depth: i32,
	pub vfov: f64, // Vertical view angle (field of view)
	pub lookat: Point3,
	pub vup: Vec3,
	pub defocus_angle: f64,
	pub focus_dist: f64,
	image_height: i32,
	center: Point3,
	pixel00_loc: Point3,
	pixel_delta_u: Vec3,
	pixel_delta_v: Vec3,
	defocus_disk_u: Vec3,
	defocus_disk_v: Vec3

}

impl Default for Camera {
	fn default() -> Self {
		Camera::new(
			16. / 9.,
			100,
			10,
			10,
			90.,
			Point3::new(0., 0., -1.),
			Point3::new_zero(),
			Vec3::new(0., 1., 0.),
			0., 10.
		)
	}
}

impl Camera {
	pub fn new(aspect_ratio: f64, image_width: i32, samples_per_pixel: i32, max_depth: i32, vfov: f64, lookfrom: Point3, lookat: Point3, vup: Vec3, defocus_angle: f64, focus_dist: f64) -> Self {
		// Calculate the image height, ensure that it's at least 1
		let image_height = (image_width as f64 / aspect_ratio) as i32;
		let image_height = if image_height < 1 { 1 } else {image_height};

		let center = lookfrom;
		// Camera
		let theta = vfov.to_radians();
		let h = (theta / 2.).tan();

		// Viewport widths less than one are ok since they are real vallued
		let focus_dist = if focus_dist <= 0. { 1. } else { focus_dist };
		let viewport_height = 2. * h * focus_dist;
		let viewport_width = viewport_height * image_width as f64 / image_height as f64;

		// Calculate u, v, w basis vectors for camera
		let w = unit_vector(&(lookfrom - lookat));
		let u = unit_vector(&cross(&vup, &w));
		let v = cross(&w, &u);

		// Calculate the vectors across the horizontal and down the vertical viewport edges
		let viewport_u = viewport_width * u;
		let viewport_v = viewport_height * -v;

		// Calcualte the horizontal and vertical delta vectors from pixel to pixel
		let pixel_delta_u = viewport_u / image_width as f64;
		let pixel_delta_v = viewport_v / image_height as f64;

		// Calculate the location of the upper left pixel
		let viewport_upper_left = center - (focus_dist * w) - viewport_u / 2. - viewport_v / 2.;
		let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

		let defocus_radius = focus_dist * (defocus_angle / 2.).to_radians().tan();

		return Camera {
			aspect_ratio,
			image_width,
			image_height,
			center,
			vfov,
			pixel00_loc,
			pixel_delta_u,
			pixel_delta_v,
			samples_per_pixel,
			max_depth,
			lookat, vup,
			defocus_angle,
			focus_dist,
			defocus_disk_u: u * defocus_radius,
			defocus_disk_v: v * defocus_radius
		};
	}
}

pub fn init_pixels(cam: &Camera) -> Vec<Color> {
	vec![Color::new_zero(); (cam.image_height * cam.image_width) as usize]
}

pub fn render(cam: &Camera, world: &HittableList, pixels: &mut Vec<Color>) {
	println!("P3\n{} {}\n255", cam.image_width, cam.image_height);

	for j in 0..cam.image_height {
        eprint!("\r Progress: {:.1}% ", j as f32 / (cam.image_height - 1) as f32 * 100.);

        for i in 0..cam.image_width {
			//let mut pixel_color = Vec3::new_zero();
			let index = (j * cam.image_width + i) as usize;
			for _sample in 0..cam.samples_per_pixel {
				let r = get_ray(cam, i, j);
				pixels[index] =
				pixels[index] + ray_color(&r,  cam.max_depth, world);
			}
            write_color(&mut std::io::stdout(), &pixels[index], cam.samples_per_pixel as f64);
        }
    }

	eprintln!("\rDone!                           ");
}

pub fn render_par(cam: &Camera, world: &HittableList, pixels: &mut Vec<Color>) {
	println!("P3\n{} {}\n255", cam.image_width, cam.image_height);


	let threads: usize = match std::thread::available_parallelism() {
		Ok(ok) => ok.into(),
		Err(_) => {
			let threads = rayon::current_num_threads();
			eprintln!("Could not count cores, defaulting to {} threads", threads);
			threads
		}
	};


	match rayon::ThreadPoolBuilder::new().num_threads(threads.into()).build_global() {
		Ok(_ok) => eprintln!("Rendering on {} threads", threads),
		Err(_) => eprintln!("Could not set threads, rayon will use 4 threads.")
	};


	// let chunk_size = ((cam.image_height * cam.image_width) as f64 / (threads * 12) as f64) as usize;
	let chunk_size = (cam.image_width * 3) as usize; // ratio taken from https://github.com/dps/rust-raytracer

	let rows: Vec<(usize, &mut [Color])> = pixels.chunks_mut(chunk_size).enumerate().collect();
	let len = rows.len();

	let progress_chunk = 100. / len as f64;

	let counter = std::sync::Mutex::new(0);
	rows.into_par_iter().for_each(|(j, row)| {
        for i in 0..row.len() {
			let idx = (j * chunk_size + i) as i32;
			let x = idx as i32 % cam.image_width;
			let y = idx / cam.image_width;

			for _sample in 0..cam.samples_per_pixel {
				let r = get_ray(cam, x, y);

				row[i] = row[i] + ray_color(&r,  cam.max_depth, world);
			}
        }

		let mut counter = counter.lock().expect("should work");
		*counter += 1;
		eprint!("\rProgress {:.1}%", *counter as f64 * progress_chunk);
    });

	eprintln!("\rWriting...            ");

	for j in 0..cam.image_height {
        for i in 0..cam.image_width {
			let index = (j * cam.image_width + i) as usize;

            write_color(&mut std::io::stdout(), &pixels[index], cam.samples_per_pixel as f64);
        }
    }

	eprintln!("\rDone!                           ");
}

fn get_ray(cam: &Camera, i: i32, j: i32) -> Ray {
	// Get a randomly sampled camera ray for the pixel at location i, j, originating from camera defocus disk

	let pixel_center = cam.pixel00_loc + (i as f64 * cam.pixel_delta_u) + (j as f64 * cam.pixel_delta_v);
	// you can replace sample_square with sample_disk for circular pixels
	let pixel_sample = pixel_center + pixel_sample_square(&cam);

	let ray_origin = if cam.defocus_angle <= 0. {
		cam.center
	} else {
		defocus_disk_sample(cam)
	};

	let ray_direction = pixel_sample - ray_origin;

	Ray::new(ray_origin, ray_direction)

}

fn defocus_disk_sample(cam: &Camera) -> Point3 {
	let p = random_in_unit_disk();
	cam.center + (p.x() * cam.defocus_disk_u) + (p.y() * cam.defocus_disk_v)
}

fn pixel_sample_square(cam: &Camera) -> Vec3 {
	// Returns a random point in the square surrounding a pixel at the origin
	let px = random_double() - 0.5;
	let py = random_double() - 0.5;
	px * cam.pixel_delta_u + py * cam.pixel_delta_v
}

fn ray_color(r: &Ray, depth: i32, world: &dyn Hittable) -> Color {
	// check if we hit bounce limit
	if depth <= 0 { return Vec3::new_zero() }

	match world.hit(r, &Interval {min: 0.0001, max:  INF }) {
		Some(rec) => {
			match rec.mat.scatter(r, &rec) {
				Some((attenuation, scattered)) => attenuation * ray_color(&scattered, depth - 1, world),
				None => Color::new_zero()
			}
		}
		None => {
			// This sets the skybox + ambient light
			let unit_direction = unit_vector(&r.direction());
			let a = 0.5 * (unit_direction.y() + 1.0);

			(1.0 - a) * Color::new(1.0, 1.0, 1.) + a * Color::new(0.5, 0.7, 1.0)
		}
	}

}

/*fn do_sun(r: &Ray, world: &dyn Hittable) -> Color {
	let sun = unit_vector(&Vec3::new(1., 1., 1.));
	let sun_color = Color::new(1., 1., 1.);

	compute_sun(&sun, &sun_color, r, world)
}

fn compute_sun(sun: &Vec3, sun_color: &Color, r: &Ray, world: &dyn Hittable) -> Color {
	let unit_direction = unit_vector(&r.direction());
	return dot(&unit_direction, &sun).max(0.) * *sun_color;
	match world.hit(&Ray::new(r.origin(), &sun), &Interval {min: 0.001, max: f64::INFINITY}) {
		None => {

			let sun_intensity = dot(&unit_direction, &sun);

			if sun_intensity > 0. {
				sun_intensity * sun_color.clone()
			} else {
				Color::new_zero()
			}
		}
		_ => {
			//eprintln!("woah this shouldn't happen");
			Color::new_zero()
		}
	}
}
*/
/*fn ray_color_sun(r: &Ray, depth: i32, world: &dyn Hittable, normal: Option<&Vec3>) -> Color {
	// check if we hit bounce limit
	if depth <= 0 { return Vec3::new_zero() }

	match world.hit(r, &Interval {min: 0.0001, max:  INF }) {
		Some(rec) => {
			match rec.mat.scatter(r, &rec) {
				Some((attenuation, scattered)) => attenuation * ray_color_sun(&scattered, depth - 1, world, Some(&rec.normal)),
				None => Color::new_zero()
			}
		}
		None => {
			let unit_direction = unit_vector(&r.direction());
			match normal {
				None => {
					let a = 0.5 * (unit_direction.y() + 1.0);

					Color::new_zero()
				//(1.0 - a) * Color::new(1.0, 1.0, 1.) + a * Color::new(0.5, 0.7, 1.0)
				}
				Some (normal) => {
					let sun = unit_vector(&Vec3::new(1.0, 1., 0.));
					let sun_color = Color::new(1., 1., 1.);
					//match world.shadow_hit(&Ray::new(&r.origin(), &sun), &Interval {min: 0.0001, max:  INF}) {
						//None => dot(&sun, &normal).max(0.) * sun_color,
						//Some(_) => Color::new_zero()
					//}
					dot(&sun, &unit_direction) * sun_color
				}
			}
		}
	}
	// This sets the skybox + ambient light

}
*/