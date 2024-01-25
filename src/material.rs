
use crate::{color::Color,
	hittable::HitRecord,
	ray::Ray,
	vec3::{random_unit_vector, reflect, unit_vector}
};

pub trait Material {
	fn scatter(
		&self,
		r_in: &Ray,
		rec: &HitRecord,
	) -> Option<(Color, Ray)>;
}




pub struct Lambertian {
	albedo: Color,
}


impl Lambertian {
	pub fn new(albedo: Color) -> Self {
		Lambertian { albedo }
	}
}

impl Material for Lambertian {
	fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
		let scatter_direction = rec.normal + random_unit_vector();
		let scatter_direction = if scatter_direction.near_zero() { rec.normal } else { scatter_direction };

		Some((
			self.albedo,
			Ray::new(&rec.p, &scatter_direction),
		))

	}

}

pub struct PerfectLambertian {
	albedo: Color
}

impl PerfectLambertian {
	pub fn new(albedo: Color) -> Self {
		PerfectLambertian { albedo }
	}
}

impl Material for PerfectLambertian {
	fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
		Some((
			self.albedo,
			Ray::new(&rec.p, &rec.normal)
		))
	}
}

pub struct Metal {
	albedo: Color
}

impl Metal {
	pub fn new(albedo: Color) -> Self {
		Metal { albedo }
	}
}

impl Material for Metal {
	fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
		let reflected = reflect(&unit_vector(&r_in.direction()),&rec.normal);
		Some((self.albedo, Ray::new(&rec.p, &reflected)))
	}
}