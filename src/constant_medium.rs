use std::sync::Arc;

use crate::{
    color::Color,
    hittable::{HitRecord, Hittable},
    interval::{Interval, _UNIVERSE},
    material::{Isotropic, Material},
    object::Object,
    ray::Ray,
    texture::Texture,
    utils::{random_double, INF},
    vec3::Vec3,
};

#[derive(Clone)]
pub struct ConstantMedium {
    boundary: Arc<Object>,
    neg_inv_density: f64,
    phase_function: Material,
}

impl ConstantMedium {
    pub fn new(boundary: Arc<Object>, density: f64, c: Color) -> Object {
        Object::Volume(ConstantMedium {
            boundary,
            neg_inv_density: -1. / density,
            phase_function: Isotropic::new(c),
        })
    }

    pub fn from_texture(boundary: Arc<Object>, density: f64, albedo: Arc<Texture>) -> Object {
        Object::Volume(ConstantMedium {
            boundary,
            neg_inv_density: -1. / density,
            phase_function: Isotropic::from_texture(albedo),
        })
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        // Print occasional samples when debugging. To enable, set enable debug to true.
        let enable_debug = false;
        let debugging = enable_debug && random_double() < 0.00001;

        match self.boundary.hit(r, &_UNIVERSE) {
            None => None,
            Some(mut rec1) => {
                match self.boundary.hit(
                    r,
                    &Interval {
                        min: rec1.t + 0.0001,
                        max: INF,
                    },
                ) {
                    None => None,
                    Some(mut rec2) => {
                        if rec1.t < ray_t.min {
                            rec1.t = ray_t.min
                        };
                        if rec2.t > ray_t.max {
                            rec2.t = ray_t.max
                        };

                        if rec1.t >= rec2.t {
                            return None;
                        };

                        if rec1.t < 0. {
                            rec1.t = 0.
                        };

                        let ray_length = r.direction().length();
                        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
                        let hit_distance = self.neg_inv_density * random_double().ln();

                        if hit_distance > distance_inside_boundary {
                            return None;
                        };

                        let t = rec1.t + hit_distance / ray_length;
                        Some(HitRecord {
                            t,
                            p: r.at(t),
                            normal: Vec3::new(1., 0., 0.), // arbitrary
                            front_face: true,              // arbitrary
                            mat: &self.phase_function,
                            u: 0.,
                            v: 0.,
                        })
                    }
                }
            }
        }
    }

    fn bounding_box(&self) -> Option<&crate::object::Aabb> {
        self.boundary.bounding_box()
    }
}
