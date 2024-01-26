
use crate::interval::Interval;
use crate::color::{Color, write_color};
use crate::ray::Ray;
use crate::vec3::{unit_vector, Point3, Vec3};
use crate::hittable::{Hittable, HittableList};
use crate::utils::{random_double, INF};
use std::thread::{self, available_parallelism};
use std::sync::{Arc, Mutex};

pub struct Camera {
	pub aspect_ratio: f64,
	pub image_width: i32,
	pub samples_per_pixel: i32,
	pub max_depth: i32,
	pub vfov: f64, // Vertical view angle (field of view)
	image_height: i32,
	center: Point3,
	pixel00_loc: Point3,
	pixel_delta_u: Vec3,
	pixel_delta_v: Vec3,
	pixels: Arc<Mutex<Vec<Vec<Color>>>>
}

impl Default for Camera {
	fn default() -> Self {
		Camera::new(
			16. / 9.,
			100,
			10,
			10,
			90.
		)
	}
}

impl Camera {
	pub fn new(aspect_ratio: f64, image_width: i32, samples_per_pixel: i32, max_depth: i32, vfov: f64) -> Self {
		// Calculate the image height, ensure that it's at least 1
		let image_height = (image_width as f64 / aspect_ratio) as i32;
		let image_height = if image_height < 1 { 1 } else {image_height};

		// Camera
		let focal_length = 1.0;
		let theta = vfov.to_radians();
		let h = (theta / 2.).tan();

		// Viewport widths less than one are ok since they are real vallued
		let viewport_height = 2. * h * focal_length;
		let viewport_width = viewport_height * image_width as f64 / image_height as f64;
		let center = Point3::new(0., 0., 0.);

		// Calculate the vectors across the horizontal and down the vertical viewport edges
		let viewport_u = Vec3::new(viewport_width, 0., 0.,);
		let viewport_v = Vec3::new(0., -viewport_height, 0.);

		// Calcualte the horizontal and vertical delta vectors from pixel to pixel
		let pixel_delta_u = viewport_u / image_width as f64;
		let pixel_delta_v = viewport_v / image_height as f64;

		// Calculate the location of the upper left pixel
		let viewport_upper_left = center - Vec3::new(0., 0., focal_length) - viewport_u / 2. - viewport_v / 2.;
		let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

		let pixels = Arc::new(Mutex::new(vec![vec![Color::new_zero(); image_width as usize]; image_height as usize]));
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
			pixels
		};
	}
}

fn _render_thread(cam: Arc<Camera>, samples: i32, world: Arc<HittableList>) {
	let mut thread_pixels = vec![vec![Color::new_zero(); cam.image_width as usize]; cam.image_height as usize];
	for j in 0..cam.image_height  {
        if j % (cam.image_height / 100) == 0 { eprint!("\r Progress: {}% ", j * 100 / cam.image_height); }
        for i in 0..cam.image_width {
			let y = j as usize;
			let x = i as usize;
			for _sample in 0..samples {
				let r = thread_get_ray(cam.clone(), i, j);
				thread_pixels[y][x] = thread_pixels[y][x] + ray_color_threaded(&r,  cam.max_depth, world.clone());
			}
        }
    }

	// lock
	let pixels = &mut cam.pixels.lock().unwrap();
	for j in 0..cam.image_height  {
        for i in 0..cam.image_width {
			let y = j as usize;
			let x = i as usize;
			pixels[y][x] = pixels[y][x] + thread_pixels[y][x];
			// unlock
        }
    }

	// done
}

pub fn render(cam: Arc<Camera>, world: Arc<HittableList>) {
	let threads = 8;
	let samples = (cam.samples_per_pixel as f32 / threads as f32).ceil() as i32;
	let ray_count = threads * samples;
	eprintln!("Rendering with {} thread(s) and {} samples per pixel", threads, samples);
	let mut handles = vec![];

	// spawn threads
	for _ in 0..threads {
		let new_cam = Arc::clone(&cam);
		let new_world = world.clone();
		handles.push(thread::spawn(move || {
			_render_thread(new_cam, samples, new_world);
		}));
	}


	// have each one run the sub render function
	/*for j in 0..cam.image_height {
        eprint!("\r Progress: {:.3}% ", j as f32 / (cam.image_height - 1) as f32 * 100.);
        for i in 0..cam.image_width {
			let mut pixel_color = Vec3::new_zero();
			for _sample in 0..cam.samples_per_pixel {
				let r = get_ray(cam, i, j);
				pixel_color = pixel_color + ray_color(&r,  cam.max_depth, world);
			}
			let y = j as usize;
			let x = i as usize;
			cam.pixels.lock().unwrap()[y][x] = pixel_color;
        }
    }*/

	for handle in handles {
		handle.join().unwrap();
	}

	eprintln!("\rSaving...                           ");
	let pixels = cam.pixels.lock().unwrap();
	println!("P3\n{} {}\n255", cam.image_width, cam.image_height);
	for j in 0..cam.image_height {
        for i in 0..cam.image_width {
            write_color(&mut std::io::stdout(), &pixels[j as usize][i as usize], ray_count as f64);
        }
    }
	eprintln!("\rDone!                          ");
}

fn thread_get_ray(cam: Arc<Camera>, i: i32, j: i32) -> Ray {
	// Geta randomly sampled camera ray for the pixel at location i, j

	let pixel_center = cam.pixel00_loc + (i as f64 * cam.pixel_delta_u) + (j as f64 * cam.pixel_delta_v);
	// you can replace sample_square with sample_disk for circular pixels
	let pixel_sample = pixel_center + pixel_sample_square(&cam);

	let ray_origin = cam.center;
	let ray_direction = pixel_sample - ray_origin;

	Ray::new(ray_origin, ray_direction)

}

fn get_ray(cam: &Camera, i: i32, j: i32) -> Ray {
	// Geta randomly sampled camera ray for the pixel at location i, j

	let pixel_center = cam.pixel00_loc + (i as f64 * cam.pixel_delta_u) + (j as f64 * cam.pixel_delta_v);
	// you can replace sample_square with sample_disk for circular pixels
	let pixel_sample = pixel_center + pixel_sample_square(&cam);

	let ray_origin = cam.center;
	let ray_direction = pixel_sample - ray_origin;

	Ray::new(ray_origin, ray_direction)

}

fn pixel_sample_square(cam: &Camera) -> Vec3 {
	// Returns a random point in the square surrounding a pixel at the origin
	let px = random_double() - 0.5;
	let py = random_double() - 0.5;
	px * cam.pixel_delta_u + py * cam.pixel_delta_v
}


fn ray_color_threaded(r: &Ray, depth: i32, world: Arc<dyn Hittable>) -> Color {
	// check if we hit bounce limit
	if depth <= 0 { return Vec3::new_zero() }

	match world.hit(r, &Interval {min: 0.0001, max:  INF }) {
		Some(rec) => {
			match rec.mat.scatter(r, &rec) {
				Some((attenuation, scattered)) => attenuation * ray_color_threaded(&scattered, depth - 1, world),
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