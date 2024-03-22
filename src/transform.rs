use crate::utils::INF;
use std::sync::Arc;

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    object::{Aabb, Object},
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Clone)]
pub enum Transform {
    Translate(Translate),
    RotY(RotateY),
    //   RotX(RotateX),
}

#[derive(Clone)]
pub struct Translate {
    object: Arc<Object>,
    offset: Vec3,
    bbox: Aabb,
}

impl Hittable for Transform {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        match self {
            Self::Translate(t) => t.hit(r, ray_t),
            Self::RotY(t) => t.hit(r, ray_t),
        }
    }

    fn bounding_box(&self) -> Option<&Aabb> {
        match self {
            Self::Translate(t) => t.bounding_box(),
            Self::RotY(t) => t.bounding_box(),
        }
    }
}

impl Translate {
    pub fn new(p: Arc<Object>, displacement: Vec3) -> Object {
        let bbox = match p.bounding_box() {
            None => panic!(),
            Some(bbox) => *bbox + displacement,
        };
        Object::Transform(Transform::Translate(Translate {
            bbox,
            object: p,
            offset: displacement,
        }))
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

#[derive(Clone)]
pub struct RotateY {
    object: Arc<Object>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Aabb,
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut origin = r.origin();
        let mut direction = r.direction();

        origin.set(
            0,
            self.cos_theta * r.origin().x() - self.sin_theta * r.origin().z(),
        );
        origin.set(
            2,
            self.sin_theta * r.origin().x() + self.cos_theta * r.origin().z(),
        );

        direction.set(
            0,
            self.cos_theta * r.direction().x() - self.sin_theta * r.direction().z(),
        );
        direction.set(
            2,
            self.sin_theta * r.direction().x() + self.cos_theta * r.direction().z(),
        );

        let rotated_r = Ray::new_timed(origin, direction, r.time());

        // Determine where (if any) an intersection occurs in object space
        match self.object.hit(&rotated_r, ray_t) {
            None => None,
            Some(mut rec) => {
                // Chane the intersection point from object space to world space
                let mut p = rec.p;
                p.set(0, self.cos_theta * rec.p.x() + self.sin_theta * rec.p.z());
                p.set(2, -self.sin_theta * rec.p.x() + self.cos_theta * rec.p.z());

                // Change the normal from object space to world space
                let mut normal = rec.normal;
                normal.set(
                    0,
                    self.cos_theta * rec.normal.x() + self.sin_theta * rec.normal.z(),
                );
                normal.set(
                    2,
                    -self.sin_theta * rec.normal.x() + self.cos_theta * rec.normal.z(),
                );

                rec.p = p;
                rec.normal = normal;

                Some(rec)
            }
        }
    }

    fn bounding_box(&self) -> Option<&Aabb> {
        Some(&self.bbox)
    }
}

impl RotateY {
    pub fn new(p: Arc<Object>, angle: f64) -> Object {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = match p.bounding_box() {
            None => panic!("No bounding box for rotation"),
            Some(bbox) => {
                let mut min = Point3::new(INF, INF, INF);
                let mut max = Point3::new(-INF, -INF, -INF);

                for i in 0..2 {
                    for j in 0..2 {
                        for k in 0..2 {
                            let i = i as f64;
                            let j = j as f64;
                            let k = k as f64;

                            let x = i * bbox.x.max + (1. - i) * bbox.x.min;
                            let y = j * bbox.y.max + (1. - j) * bbox.y.min;
                            let z = k * bbox.z.max + (1. - k) * bbox.z.min;

                            let newx = cos_theta * x + sin_theta * z;
                            let newz = -sin_theta * x + cos_theta * z;

                            let tester = Vec3::new(newx, y, newz);

                            for c in 0..3 {
                                min.set(c, min.dim(c).min(tester.dim(c)));
                                max.set(c, max.dim(c).max(tester.dim(c)));
                            }
                        }
                    }
                }
                Aabb::from_points(&min, &max)
            }
        };

        Object::Transform(Transform::RotY(RotateY {
            object: p,
            bbox,
            cos_theta,
            sin_theta,
        }))
    }
}
