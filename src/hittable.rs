use crate::material::Material;
use crate::object::Object;
use crate::ray::Ray;
use crate::interval::Interval;
use crate::vec3::{Point3, Vec3, dot};


pub struct HitRecord<'material> {
	pub p: Point3, // intersection point
	pub normal: Vec3, // normal at hit
	pub mat: &'material Material,
	pub t: f64, // ray length
	pub front_face: bool,
}

impl HitRecord<'_> {
	pub fn set_face_normal(&self, r: &Ray, outward_normal: &Vec3 ) -> Self {
		let front_face = dot(&r.direction(), outward_normal) < 0.;
		HitRecord {
			p: self.p,
			mat: self.mat,
			front_face,
			t: self.t,
			normal: if front_face { *outward_normal } else { -*outward_normal }
		}
	}
}

pub trait Hittable {
	fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord>;

}

pub struct HittableList {
	pub objects: Vec<Object>

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

}