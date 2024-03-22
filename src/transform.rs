use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    object::{Aabb, Object},
    ray::Ray,
    vec3::Vec3,
};


#[derive(Clone)]
pub enum Transform {
    Translate(Translate),
    //    RotY(RotateY),
    //   RotX(RotateX),
}

#[derive(Clone)]
pub struct Translate {
    object: Box<Object>,
    offset: Vec3,
    bbox: Aabb,
}

impl Hittable for Transform {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        match self {
            Self::Translate(t) => t.hit(r, ray_t),
        }
    }

    fn bounding_box(&self) -> Option<&Aabb> {
        match self {
            Self::Translate(t) => t.bounding_box(),
        }
    }
}

impl Translate {
    pub fn new(p: Object, displacement: &Vec3) -> Transform {
        let bbox = match p.bounding_box() {
            None => panic!(),
            Some(bbox) => *bbox, 
        };
        Transform::Translate(Translate {
            bbox: bbox + *displacement,
            object: Box::new(p),
            offset: *displacement,
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
