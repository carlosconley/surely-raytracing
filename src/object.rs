use crate::interval::Interval;
use crate::hittable::{Hittable, HitRecord};
use crate::vec3::{self, dot, unit_vector, Point3, Vec3};
use crate::color::Color;
use crate::ray::Ray;
use crate::material::Material;


pub enum Object {
	Sphere(Sphere),
	Plane(Plane)
}

impl Hittable for Object {
	fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
		match self {
			Object::Sphere(s) => s.hit(r, ray_t),
			Object::Plane(p) => p.hit(r, ray_t)
		}
	}
}
pub struct Sphere {
	center: Point3,
	radius: f64,
	mat: Material
}

impl Sphere {
	pub fn new(center: Point3, radius: f64, mat: Material) -> Object {
		Object::Sphere(Sphere { center, radius, mat })
	}
}

impl Hittable for Sphere {
	fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
		let oc = r.origin() - self.center;
		let a = r.direction().length_squared();
		let half_b = dot(&oc, &r.direction());
		let c = oc.length_squared() - self.radius*self.radius;

		let discriminant = half_b * half_b - a * c;
		if discriminant < 0. { return None; }

		let sqrtd = discriminant.sqrt();

		// Find the nearest root that lies in the acceptable range
		let mut root = (-half_b - sqrtd) / a;
		if !ray_t.surrounds(root) {
			root = (sqrtd - half_b) / a;
			if !ray_t.surrounds(root) {
				return None;
			}
		}

		let rec = HitRecord {
			t: root,
			p: r.at(root),
			mat: &self.mat,
			normal: Point3::new_zero(),
			front_face: false
		};

		let outward_normal = (rec.p - self.center) / self.radius;


		Some (
			rec.set_face_normal(r, &outward_normal)
		)
	}
}

// A disk light source infinitely far away
pub struct Sun {
	pub direction: Vec3,
	albedo: Color,
	limit: f64
}

impl Sun {
	pub fn new(direction: Vec3, albedo: Color, angular_diameter: f64) -> Sun {
		let limit = 1. - angular_diameter / 180.;

		Sun { direction: unit_vector(&direction), albedo, limit }
	}

	pub fn hit(&self, r: &Ray) -> Color {
		let unit_direction = unit_vector(&r.direction());
		if dot(&unit_direction, &self.direction) > self.limit {
			self.albedo
		} else {
			Color::new_zero()
		}
	}

}

pub struct Plane {
	point: Point3,
	normal: Vec3,
	mat: Material,
}
impl Plane {
	pub fn new(point: Point3, normal: Vec3, mat: Material) -> Object {
		Object::Plane(Plane {
			point, normal: unit_vector(&normal), mat
		})
	}

}

impl Hittable for Plane {
	fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
		let t = dot(&(self.point - r.origin()), &self.normal) / dot(&r.direction(), &self.normal);

		if ray_t.surrounds(t) {
			let rec = HitRecord {
			t,
			p: r.at(t),
			mat: &self.mat,
			normal: self.normal,
			front_face: false
			};

			let outward_normal = self.normal;
			Some (
				rec.set_face_normal(r, &outward_normal)
			)
		} else {
			None
		}
	}
}