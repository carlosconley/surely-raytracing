use std::f64::consts::PI;
use std::sync::Arc;

use crate::{
    hittable::Hittable,
    object::Object,
    onb::Onb,
    vec3::{dot, random_cosine_direction, random_unit_vector, unit_vector, Point3, Vec3},
};

pub trait PDF {
    fn value(&self, direction: &Vec3) -> f64;

    fn generate(&self) -> Vec3;
}

pub struct SpherePDF;

impl PDF for SpherePDF {
    fn value(&self, _direction: &Vec3) -> f64 {
        1. / (4. * PI)
    }

    fn generate(&self) -> Vec3 {
        random_unit_vector()
    }
}

pub struct CosinePDF {
    uvw: Onb,
}

impl CosinePDF {
    pub fn new(w: &Vec3) -> CosinePDF {
        let mut uvw = Onb::default();
        uvw.build_from_w(w);
        CosinePDF { uvw }
    }
}

impl PDF for CosinePDF {
    fn value(&self, direction: &Vec3) -> f64 {
        let cosine_theta = dot(&unit_vector(direction), &self.uvw.w());

        0_f64.max(cosine_theta / PI)
    }

    fn generate(&self) -> Vec3 {
        self.uvw.local_vec(&random_cosine_direction())
    }
}

pub struct HittablePDF<'obj> {
    objects: &'obj Object,
    origin: Point3,
}

impl HittablePDF<'_> {
    pub fn new(objects: &Object, origin: Point3) -> HittablePDF {
        HittablePDF { objects, origin }
    }
}

impl PDF for HittablePDF<'_> {
    fn value(&self, direction: &Vec3) -> f64 {
        self.objects.pdf_value(&self.origin, direction)
    }

    fn generate(&self) -> Vec3 {
        self.objects.random(&self.origin)
    }

}
