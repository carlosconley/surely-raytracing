use crate::{
    hittable::{HittableList, Hittable, HitRecord},
    object::Object,
    vec3::Vec3,
    ray::Ray,
    interval::Interval,
};


pub enum Transform {
    Translate(Translate),
//    RotY(RotateY),
//   RotX(RotateX),
}

pub struct Translate {
    object: Object,
    offset: Vec3,
}

impl Translate {
}

impl Hittable for Translate {
    fn hit(&self, r: &crate::ray::Ray, ray_t: &crate::interval::Interval) -> Option<HitRecord> {
        // Move ray backwards by the offset
        let offset_r = Ray::new_timed(r.origin() - self.offset, r.direction(), r.time());

        // Determine where (if any) an intersection occurs along the offset ray
        match self.object.hit(&offset_r, ray_t) {
            None => None,
            Some(mut rec) => {
                rec.p = rec.p + self.offset;
                Some(rec)
            },

        }
    }


    fn bounding_box(&self) -> Option<&crate::object::Aabb> {
        
    }

}




