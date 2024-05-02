use std::{f64::consts::PI, sync::Arc};

use crate::{
    color::Color,
    hittable::HitRecord,
    pdf::{AnyPDF, CosinePDF, SpherePDF},
    ray::Ray,
    texture::{SolidColor, Texture},
    utils::random_double,
    vec3::{
        dot, random_unit_vector, reflect, refract, unit_vector, Point3,
    },
};


pub enum SrecData {
    PdfPtr(Arc<AnyPDF>),
    SkipRay(Ray)
}
pub struct ScatterRecord {
    pub attenuation: Color,
    pub data: SrecData,
}

#[derive(Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
    DiffuseLight(DiffuseLight),
    Isotropic(Isotropic),
}

impl MatFn for Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        match self {
            Material::Lambertian(l) => l.scatter(r_in, rec),
            Material::Metal(m) => m.scatter(r_in, rec),
            Material::Dielectric(d) => d.scatter(r_in, rec),
            Material::DiffuseLight(d) => d.scatter(r_in, rec),
            Material::Isotropic(d) => d.scatter(r_in, rec),
        }
    }

    fn emitted(&self, r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Point3) -> Color {
        match self {
            Material::DiffuseLight(l) => l.emitted(r_in, rec, u, v, p),
            _ => Color::new_zero(),
        }
    }

    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        match self {
            Material::Lambertian(l) => l.scattering_pdf(r_in, rec, scattered),
            Material::Isotropic(l) => l.scattering_pdf(r_in, rec, scattered),
            _ => 0.,
        }
    }
}

pub trait MatFn {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        None
    }

    fn emitted(&self, _r_in: &Ray, _rec: &HitRecord, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::new_zero()
    }

    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        0.
    }
}

#[derive(Clone)]
pub struct Lambertian {
    texture: Arc<Texture>,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Material {
        Material::Lambertian(Lambertian {
            texture: Arc::new(SolidColor::new(albedo)),
        })
    }

    pub fn from_texture(texture: Arc<Texture>) -> Material {
        Material::Lambertian(Lambertian { texture })
    }
}

impl MatFn for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        Some(ScatterRecord {
            attenuation: self.texture.value(rec.u, rec.v, &rec.p),
            data: SrecData::PdfPtr(Arc::new(CosinePDF::new(&rec.normal))),
        })
    }

    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cos_theta = dot(&rec.normal, &unit_vector(&scattered.direction()));

        if cos_theta < 0. {
            0.
        } else {
            cos_theta / PI
        }
    }
}

#[derive(Clone)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, f: f64) -> Material {
        let fuzz = if f < 1. { f } else { 1. };
        Material::Metal(Metal { albedo, fuzz })
    }
}

impl MatFn for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected = reflect(&unit_vector(&r_in.direction()), &rec.normal);
        let reflected = unit_vector(&reflected) + (self.fuzz * random_unit_vector());

        Some(ScatterRecord {
            attenuation: self.albedo,
            data: SrecData::SkipRay(Ray::new_timed(rec.p, reflected, r_in.time())),
        })

    }

    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        1. / (4. * PI)
    }
}

#[derive(Clone)]
pub struct Dielectric {
    tint: Color,
    ir: f64, // Index of Refraction
}

impl Dielectric {
    pub fn new(ir: f64, tint: Color) -> Material {
        Material::Dielectric(Dielectric { tint, ir })
    }

    pub fn new_clear(ir: f64) -> Material {
        Self::new(ir, Color::new(1., 1., 1.))
    }

    fn reflectance(&self, cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance

        let r0 = (1. - ref_idx) / (1. + ref_idx);
        let r0 = r0 * r0;

        r0 + (1. - r0) * (1. - cosine).powf(5.)
    }
}

impl MatFn for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = unit_vector(&r_in.direction());
        let cos_theta = dot(&-unit_direction, &rec.normal).min(1.);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction =
            if cannot_refract || self.reflectance(cos_theta, refraction_ratio) > random_double() {
                reflect(&unit_direction, &rec.normal)
            } else {
                refract(&unit_direction, &rec.normal, refraction_ratio)
            };

        Some(ScatterRecord {
            attenuation: self.tint,
            data: SrecData::SkipRay(Ray::new_timed(rec.p, direction, r_in.time())),
        })

    }
}

#[derive(Clone)]
pub struct DiffuseLight {
    emit: Arc<Texture>,
}

impl DiffuseLight {
    pub fn new(c: Color) -> Material {
        Material::DiffuseLight(DiffuseLight {
            emit: Arc::new(SolidColor::new(c)),
        })
    }

    pub fn _from_texture(emit: Arc<Texture>) -> Material {
        Material::DiffuseLight(DiffuseLight { emit })
    }

    pub fn emitted(&self, r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Point3) -> Color {
        match rec.front_face {
            true => self.emit.value(u, v, p),
            false => Color::new_zero(),
        }
    }
}

impl MatFn for DiffuseLight {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<ScatterRecord> {
        None
    }
}

#[derive(Clone)]
pub struct Isotropic {
    albedo: Arc<Texture>,
}

impl Isotropic {
    pub fn new(c: Color) -> Material {
        Material::Isotropic(Isotropic {
            albedo: Arc::new(SolidColor::new(c)),
        })
    }

    pub fn _from_texture(albedo: Arc<Texture>) -> Material {
        Material::Isotropic(Isotropic { albedo })
    }
}

impl MatFn for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        Some(ScatterRecord {
            attenuation: self.albedo.value(rec.u, rec.v, &rec.p),
            data: SrecData::PdfPtr(Arc::new(AnyPDF::Sphere(SpherePDF))),
        })
    }
}
