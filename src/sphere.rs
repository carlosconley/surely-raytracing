use crate::interval::Interval;
use crate::hittable::{Hittable, HitRecord};
use crate::vec3::{Point3, dot};
use crate::ray::Ray;

pub struct Sphere {
	center: Point3,
	radius: f64,
}

impl Sphere {
	pub fn new(center: Point3, radius: f64) -> Self {
		Sphere { center, radius }
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
			root = (-half_b + sqrtd) / a;
			if !ray_t.surrounds(root) {
				return None;
			}
		}
		let p = r.at(root);
		let outward_normal = (p - self.center) / self.radius;
		
		Some (
			HitRecord::new(r.at(root), root, r, &outward_normal)
		)
	}
}
