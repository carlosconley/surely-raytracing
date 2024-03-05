use std::sync::Arc;

use crate::interval::{Interval, EMPTY};
use crate::hittable::{Hittable, HitRecord, HittableList, BvhNode};
use crate::vec3::{dot, unit_vector, Point3, Vec3};
use crate::color::Color;
use crate::ray::{self, Ray};
use crate::material::Material;


// Using Arc's for now, but figure out more efficient way to do it later
#[derive(Clone)]
pub enum Object {
	Sphere(Sphere),
	List(Arc<HittableList>),
	Node(Arc<BvhNode>),
	Plane(Plane)
}

impl Hittable for Object {
	fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
		match self {
			Object::Sphere(o) => o.hit(r, ray_t),
			Object::List(o) => o.hit(r, ray_t),
			Object::Node(o) => o.hit(r, ray_t),
			Object::Plane(p) => p.hit(r, ray_t)
		}
	}

	fn bounding_box(&self) -> Option<&Aabb> {
		match self {
			//Object::Plane(p) => p.bounding_box(),
			Object::Sphere(o) => o.bounding_box(),
			Object::List(o) => o.bounding_box(),
			Object::Node(o) => o.bounding_box(),
			Object::Plane(o) => o.bounding_box()
		}
	}
}

#[derive(Clone)]
pub struct Sphere {
	center: Point3,
	radius: f64,
	mat: Material,
	center_vec: Option<Vec3>,
	bbox: Aabb,

}

impl Sphere {
	pub fn new(center: Point3, radius: f64, mat: Material) -> Object {
		let rvec = Vec3::new(radius, radius, radius);
		Object::Sphere(Sphere { center, radius, mat, center_vec: None, bbox: Aabb::from_points(&(center - rvec), &(center + rvec)) })
	}

	pub fn new_moving(center1: Point3, center2: Point3, radius: f64, mat: Material,) -> Object {
		let rvec = Vec3::new(radius, radius, radius);
		let box1 = Aabb::from_points(&(center1 - rvec), &(center1 + rvec));
		let box2 = Aabb::from_points(&(center2 - rvec), &(center2 + rvec));
		Object::Sphere(Sphere { center: center1, radius, mat, center_vec: Some(center2 - center1), bbox: Aabb::from_boxes(&box1, &box2)})
	}


	fn center(&self, time: f64) -> Point3 {
		match self.center_vec {
			Some(dir) => self.center + time * dir,
			None => self.center
		}
	}
}

impl Hittable for Sphere {
	fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
		let center = self.center(r.time());
		let oc = r.origin() - center;
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

		let outward_normal = (rec.p - center) / self.radius;


		Some (
			rec.set_face_normal(r, &outward_normal)
		)
	}

	fn bounding_box(&self) -> Option<&Aabb> {
		Some(&self.bbox)
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


#[derive(Clone)]
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

	fn bounding_box(&self) -> Option<&Aabb> {
		None
	}
}

#[derive(Clone)]
pub struct Aabb {
	x: Interval,
	y: Interval,
	z: Interval
}

impl Aabb {
	pub fn empty() -> Aabb {
		Aabb { x: EMPTY, y: EMPTY, z: EMPTY }
	}

	pub fn new(x: Interval, y: Interval, z: Interval) -> Aabb {
		Aabb { x, y, z }
	}

	pub fn from_boxes(box0: &Aabb, box1: &Aabb) -> Aabb {
		Aabb {
			x: Interval::from_intervals(&box0.x, &box1.x),
			y: Interval::from_intervals(&box0.y, &box1.y),
			z: Interval::from_intervals(&box0.z, &box1.z),
		}

	}

	pub fn from_points(a: &Point3, b: &Point3) -> Aabb {
		Aabb {
			x: Interval { min: a.x().min(b.x()), max: a.x().max(b.x()) },
			y: Interval { min: a.y().min(b.y()), max: a.y().max(b.y()) },
			z: Interval { min: a.z().min(b.z()), max: a.z().max(b.z()) },
		}
	}

	pub fn axis(&self, n: u8) -> &Interval {
		match n {
			0 => &self.x,
			1 => &self.y,
			2 => &self.z,
			_ => panic!("Cannot acces the 4th or higher dimension!")
		}

	}

	pub fn hit(&self, r: &Ray, ray_t: &Interval) -> bool {
		// Override and create copy to get around mutability rules
		let mut ray_t = Interval {
			min: ray_t.min,
			max: ray_t.max,
		};

		for a in 0..3 {
			let inv_d = 1. / r.direction().dim(a);
			let orig = r.origin().dim(a);

			let t0 = (self.axis(a).min - orig) * inv_d;
			let t1 = (self.axis(a).max - orig) * inv_d;

			// swap if less than 0
			let (t0, t1) = if inv_d < 0. {
				(t1, t0)
			} else {
				(t0, t1)
			};

			if t0 > ray_t.min { ray_t.min = t0 }
			if t1 < ray_t.max { ray_t.max = t1 }


			if ray_t.max <= ray_t.min {
				return false;
			}
		}

		true
	}

}