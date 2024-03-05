
use crate::{color::Color,
	hittable::HitRecord,
	ray::Ray,
	vec3::{random_unit_vector, reflect, unit_vector, refract, dot},
	utils::random_double,
	texture::{ Texture, SolidColor },
};

#[derive(Clone)]
pub enum Material {
	Lambertian(Lambertian),
	Metal(Metal),
	Dielectric(Dielectric),
}

impl MatFn for Material {
	fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
		match self {
			Material::Lambertian(l) => l.scatter(r_in, rec),
			Material::Metal(m) => m.scatter(r_in, rec),
			Material::Dielectric(d) => d.scatter(r_in, rec)
		}
	}
}

pub trait MatFn {
	fn scatter(
		&self,
		r_in: &Ray,
		rec: &HitRecord,
	) -> Option<(Color, Ray)>;
}


#[derive(Clone)]
pub struct Lambertian {
	texture: Texture,
}


impl Lambertian {
	pub fn new(albedo: Color) -> Material {
		Material::Lambertian(Lambertian { texture: SolidColor::new( albedo ) })
	}

	pub fn from_texture(texture: Texture) -> Material {
		Material::Lambertian(
			Lambertian {
				texture
			}
		)
	}
}

impl MatFn for Lambertian {
	fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
		let scatter_direction = rec.normal + random_unit_vector();
		// catch degenderate scatter direction
		let scatter_direction = if scatter_direction.near_zero() { rec.normal } else { scatter_direction };

		Some((
			self.texture.value(rec.u, rec.v, &rec.p),
			Ray::new_timed(rec.p, scatter_direction, r_in.time()),
		))

	}

}

#[derive(Clone)]
pub struct Metal {
	albedo: Color,
	fuzz: f64
}

impl Metal {
	pub fn new(albedo: Color, f: f64) -> Material {
		let fuzz = if f < 1. { f } else { 1. };
		Material::Metal(Metal { albedo, fuzz })
	}
}

impl MatFn for Metal {
	fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
		let reflected = reflect(&unit_vector(&r_in.direction()),&rec.normal);
		let scattered = Ray::new_timed(rec.p, reflected + self.fuzz * random_unit_vector(), r_in.time());
		Some((self.albedo, scattered))
	}
}

#[derive(Clone)]
pub struct Dielectric {
	tint: Color,
	ir: f64 // Index of Refraction
}

impl Dielectric {
	pub fn new(ir: f64, tint: Color) -> Material {
		Material::Dielectric(Dielectric { tint, ir })
	}

	fn reflectance(&self, cosine: f64, ref_idx: f64) -> f64 {
		// Use Schlick's approximation for reflectance

		let r0 = (1. - ref_idx) / (1. + ref_idx);
		let r0 = r0 * r0;

		r0 + (1. - r0) * (1. - cosine).powf(5.)
	}
}

impl MatFn for Dielectric {
	fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
		let refraction_ratio = if rec.front_face {1.0 / self.ir } else { self.ir };

		let unit_direction = unit_vector(&r_in.direction());
		let cos_theta = dot(&-unit_direction, &rec.normal).min(1.);
		let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

		let cannot_refract = refraction_ratio * sin_theta > 1.0;
		let direction = if cannot_refract || self.reflectance(cos_theta, refraction_ratio) > random_double() {
			reflect(&unit_direction, &rec.normal)
		} else {
			refract(&unit_direction, &rec.normal, refraction_ratio)
		};

		Some((self.tint, Ray::new_timed(rec.p, direction, r_in.time())))
	}
}