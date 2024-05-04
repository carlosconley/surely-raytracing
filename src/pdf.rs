use std::f64::consts::PI;
use std::sync::Arc;

use crate::{
    hittable::Hittable, object::Object, onb::Onb, utils::random_double, vec3::{dot, random_cosine_direction, random_unit_vector, unit_vector, Point3, Vec3}
};

pub enum AnyPDF {
    Sphere(SpherePDF),
    Cosine(CosinePDF),
    _Mixed(MixturePDF),
    Hittable(HittablePDF),
}

impl PDF for AnyPDF {

    fn value(&self, direction: &Vec3) -> f64 {
        match self {
            AnyPDF::Sphere(p) => p.value(direction),
            AnyPDF::Cosine(p) => p.value(direction),
            AnyPDF::_Mixed(p) => p.value(direction),
            AnyPDF::Hittable(p) => p.value(direction),

        }
    }

    fn generate(&self) -> Vec3 {
        match self {
            AnyPDF::Sphere(p) => p.generate(),
            AnyPDF::Cosine(p) => p.generate(),
            AnyPDF::_Mixed(p) => p.generate(),
            AnyPDF::Hittable(p) => p.generate(),
        }

    }
}

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
    pub fn new(w: &Vec3) -> AnyPDF {
        let mut uvw = Onb::default();
        uvw.build_from_w(w);
        AnyPDF::Cosine(CosinePDF { uvw })
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

pub struct HittablePDF {
    objects: Arc<Object>,
    origin: Point3,
}

impl HittablePDF {
    pub fn new(objects: Arc<Object>, origin: Point3) -> AnyPDF {
        AnyPDF::Hittable(HittablePDF { objects, origin })
    }
}

impl PDF for HittablePDF {
    fn value(&self, direction: &Vec3) -> f64 {
        self.objects.pdf_value(&self.origin, direction)
    }

    fn generate(&self) -> Vec3 {
        self.objects.random(&self.origin)
    }

}

pub struct MixturePDF {
    p0: Box<AnyPDF>,
    p1: Box<AnyPDF>
}

impl MixturePDF {
    pub fn new(p0: Box<AnyPDF>, p1: Box<AnyPDF>) -> Self {
        MixturePDF {
            p0, p1
        }
    }
}

impl PDF for MixturePDF {
    fn value(&self, direction: &Vec3) -> f64 {
        0.5 * self.p0.value(direction) + 0.5 * self.p1.value(direction)
    }

    fn generate(&self) -> Vec3 {
        if random_double() < 0.5 {
            self.p0.generate()
        } else {
            self.p1.generate()
        }
    }
}
