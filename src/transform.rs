use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    object::{Aabb, Object},
    ray::Ray,
    vec3::Vec3,
};

pub enum Transform {
    Translate(Translate),
    //    RotY(RotateY),
    //   RotX(RotateX),
}

pub struct Translate {
    object: Object,
    offset: Vec3,
    bbox: Aabb,
}

impl Translate {
    pub fn new(p: Object, displacement: &Vec3) -> Transform {
        Transform::Translate(Translate {
            object: p,
            offset: *displacement,
            bbox: p.bounding_box() + *displacement,
        })
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &crate::ray::Ray, ray_t: &Interval) -> Option<HitRecord> {
        // Move ray backwards by the offset
        let offset_r = Ray::new_timed(r.origin() - self.offset, r.direction(), r.time());

        // Determine where (if any) an intersection occurs along the offset ray
        match self.object.hit(&offset_r, ray_t) {
            None => None,
            Some(mut rec) => {
                rec.p = rec.p + self.offset;
                Some(rec)
            }
        }
    }

    fn bounding_box(&self) -> Option<&crate::object::Aabb> {
        Some(&self.bbox)
    }
}
