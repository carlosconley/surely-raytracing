use crate::material::Material;
use crate::ray::Ray;
use crate::interval::Interval;
use crate::vec3::{Point3, Vec3, dot};

use::std::rc::Rc;

pub struct HitRecord {
	pub p: Point3, // intersection point
	pub normal: Vec3, // normal at hit
	pub mat: Rc<dyn Material>,
	pub t: f64, // ray length
	pub front_face: bool,
}

impl HitRecord {
	pub fn new(p: Point3, t:f64, r: &Ray, normal: &Vec3, mat: Rc<dyn Material>) -> Self {
		let front_face = dot(&r.direction(), normal) < 0.;
		HitRecord {
			p,
			t,
			mat,
			front_face,
			normal: if front_face { normal.clone() } else { -normal.clone() }
		}
	}
}

pub trait Hittable {
	fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord>;

	fn shadow_hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
		None
	}
}

pub struct HittableList {
	pub objects: Vec<Rc<dyn Hittable>>

}

impl HittableList {

}

impl Hittable for HittableList {
	fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
		let mut rec = None;
		let mut closest_so_far = ray_t.max;

		for object in self.objects.iter() {
			match object.hit(r, &Interval { min: ray_t.min, max: closest_so_far } ) {
				Some(temp_rec) => {
					closest_so_far = temp_rec.t;
					rec = Some(temp_rec);
				}
				_ => ()
			}
		}

		rec

	}
	fn shadow_hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
		let mut rec = None;

		for object in self.objects.iter() {
			match object.hit(r, ray_t) {
				Some(temp_rec) => {
					return Some(temp_rec);	
				}
				_ => ()
			}
		}	

		rec
	}
}