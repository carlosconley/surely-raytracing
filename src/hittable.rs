use crate::material::Material;
use crate::object::{ Object, Aabb };
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

	// consider returning &Aabb if we aren't modifying it
	fn bounding_box(&self) -> &Aabb;
}

pub struct HittableList {
	pub objects: Vec<Object>,
	bbox: Aabb,
}

impl HittableList {
	pub fn new() -> HittableList {
		HittableList {
			objects: vec![],
			bbox: Aabb::empty()
		}
	}

	pub fn add(&mut self, object: Object) {
		self.bbox = Aabb::from_boxes(&self.bbox, object.bounding_box());
		self.objects.push(object);
	}
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

	fn bounding_box(&self) -> &Aabb {
		&self.bbox
	}

}

// In the book left and right are shared pointers,
// but slows runtime while saving memory
// I'm going to keep runtime performance at the cost of memory
pub struct BvhNode {
	left: Object,
	right: Object,
	bbox: Aabb
}

impl BvhNode {
	pub fn from_list(list: &HittableList) -> BvhNode {
		BvhNode::new(&list.objects, 0, list.objects.len())
	}

	pub fn new(src_objects: &Vec<Object>, start: usize, end: usize) -> BvhNode {
		todo!()
	}
}

impl Hittable for BvhNode {
	fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
		if !self.bbox.hit(r, ray_t) {
			return None;
		}

		match self.left.hit(r, ray_t) {
			Some(rec) => {
				match self.right.hit(r,
				&Interval { min: ray_t.min, max: rec.t }) {
					None => Some(rec),
					x => x
				}
			},
			None => self.right.hit(r, ray_t)
		}

	}

	fn bounding_box(&self) -> &Aabb {
		&self.bbox
	}
}