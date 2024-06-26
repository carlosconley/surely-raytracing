use std::cmp::Ordering;
use std::sync::Arc;

use crate::interval::Interval;
use crate::material::Material;
use crate::object::{Aabb, Object};
use crate::ray::Ray;
use crate::utils::random_int;
use crate::vec3::{dot, Point3, Vec3};

pub struct HitRecord<'material> {
    pub p: Point3,    // intersection point
    pub normal: Vec3, // normal at hit
    pub mat: &'material Material,
    pub t: f64, // ray length
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord<'_> {
    pub fn set_face_normal(&self, r: &Ray, outward_normal: &Vec3) -> Self {
        let front_face = dot(&r.direction(), outward_normal) < 0.;
        HitRecord {
            p: self.p,
            mat: self.mat,
            front_face,
            t: self.t,
            u: self.u,
            v: self.v,
            normal: if front_face {
                *outward_normal
            } else {
                -*outward_normal
            },
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord>;

    // consider returning &Aabb if we aren't modifying it
    fn bounding_box(&self) -> Option<&Aabb>;

    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64 {
        0.
    }

    fn random(&self, origin: &Point3) -> Vec3 {
        Vec3::new(1., 0., 0.)
    }
}

pub struct HittableList {
    pub objects: Vec<Object>,
    bbox: Aabb,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: vec![],
            bbox: Aabb::empty(),
        }
    }

    pub fn from_object(obj: Object) -> HittableList {
        let mut list = Self::new();
        list.add(obj);
        list
    }

    pub fn add(&mut self, object: Object) {
        match object.bounding_box() {
            Some(bbox) => self.bbox = Aabb::from_boxes(&self.bbox, bbox),
            None => (),
        };
        self.objects.push(object);
    }

    pub fn create_bvh(&mut self) -> HittableList {
        HittableList::from_object(Object::Node(Arc::new(BvhNode::from_list(self))))
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut rec = None;
        let mut closest_so_far = ray_t.max;

        for object in self.objects.iter() {
            match object.hit(
                r,
                &Interval {
                    min: ray_t.min,
                    max: closest_so_far,
                },
            ) {
                Some(temp_rec) => {
                    closest_so_far = temp_rec.t;
                    rec = Some(temp_rec);
                }
                _ => (),
            }
        }

        rec
    }

    fn bounding_box(&self) -> Option<&Aabb> {
        Some(&self.bbox)
    }

    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64 {
        let weight = 1. / self.objects.len() as f64;

        self.objects
            .iter()
            .map(|obj| obj.pdf_value(origin, direction) )
            .reduce(|x, y| x + y).expect("Could not sum pdf values of list")
            * weight

    }

    fn random(&self, origin: &Point3) -> Vec3 {
        let int_size = self.objects.len() as i64;
        self.objects[random_int(0, int_size - 1) as usize].random(origin)
    }
}

// In the book left and right are shared pointers,
// but slows runtime while saving memory
// I'm going to keep runtime performance at the cost of memory
pub struct BvhNode {
    left: Object,
    right: Object,
    bbox: Aabb,
}

impl BvhNode {
    pub fn from_list(list: &mut HittableList) -> BvhNode {
        let len = list.objects.len();
        BvhNode::new(&mut list.objects, 0, len)
    }

    pub fn new(src_objects: &mut Vec<Object>, start: usize, end: usize) -> BvhNode {
        let objects = src_objects;

        let axis = random_int(0, 2);
        let comparator = if axis == 0 {
            Self::box_x_compare
        } else if axis == 1 {
            Self::box_y_compare
        } else {
            Self::box_z_compare
        };

        let object_span = end - start;

        let (left, right) = if object_span == 1 {
            (objects[start].clone(), objects[start].clone())
        } else if object_span == 2 {
            match comparator(&objects[start], &objects[start + 1]) {
                Ordering::Less => (objects[start].clone(), objects[start + 1].clone()),
                _ => (objects[start + 1].clone(), objects[start].clone()),
            }
        } else {
            objects[start..end].sort_by(comparator);

            let mid = start + object_span / 2;

            (
                Object::Node(Arc::new(BvhNode::new(objects, start, mid))),
                Object::Node(Arc::new(BvhNode::new(objects, mid, end))),
            )
        };

        BvhNode {
            bbox: Aabb::from_boxes(
                left.bounding_box().expect("No left bounding box"),
                right.bounding_box().expect("No right bounding box"),
            ),
            left,
            right,
        }
    }

    fn box_compare(a: &Object, b: &Object, axis: u8) -> Ordering {
        a.bounding_box()
            .expect("No bounding box for object a")
            .axis(axis)
            .min
            .total_cmp(
                &b.bounding_box()
                    .expect("No bounding box for object b")
                    .axis(axis)
                    .min,
            )
    }

    fn box_x_compare(a: &Object, b: &Object) -> Ordering {
        Self::box_compare(a, b, 0)
    }

    fn box_y_compare(a: &Object, b: &Object) -> Ordering {
        Self::box_compare(a, b, 1)
    }

    fn box_z_compare(a: &Object, b: &Object) -> Ordering {
        Self::box_compare(a, b, 2)
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        if !self.bbox.hit(r, ray_t) {
            return None;
        }

        match self.left.hit(r, ray_t) {
            Some(rec) => {
                match self.right.hit(
                    r,
                    &Interval {
                        min: ray_t.min,
                        max: rec.t,
                    },
                ) {
                    None => Some(rec),
                    x => x,
                }
            }
            None => self.right.hit(r, ray_t),
        }
    }

    fn bounding_box(&self) -> Option<&Aabb> {
        Some(&self.bbox)
    }
}

