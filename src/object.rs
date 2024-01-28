use crate::interval::Interval;
use crate::hittable::{Hittable, HitRecord};
use crate::vec3::{Point3, dot};
use crate::ray::Ray;
use crate::material::Material;


pub enum Object {
	Sphere(Sphere)
}

impl Hittable for Object {
	fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
		match self {
			Object::Sphere(s) => s.hit(r, ray_t)
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
