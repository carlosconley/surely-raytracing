use std::f64::consts::PI;
use std::ops;
use std::sync::Arc;

use crate::color::Color;
use crate::constant_medium::ConstantMedium;
use crate::hittable::{BvhNode, HitRecord, Hittable, HittableList};
use crate::interval::{Interval, EMPTY};
use crate::material::Material;
use crate::ray::Ray;
use crate::transform::Transform;
use crate::vec3::{cross, dot, unit_vector, Point3, Vec3};

// Using Arc's for now, but figure out more efficient way to do it later
#[derive(Clone)]
pub enum Object {
    Sphere(Sphere),
    List(Arc<HittableList>),
    Node(Arc<BvhNode>),
    _Plane(Plane),
    Quad(Quad),
    Transform(Transform),
    Volume(ConstantMedium),
}

impl Hittable for Object {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        match self {
            Object::Sphere(o) => o.hit(r, ray_t),
            Object::List(o) => o.hit(r, ray_t),
            Object::Node(o) => o.hit(r, ray_t),
            Object::Quad(o) => o.hit(r, ray_t),
            Object::_Plane(p) => p.hit(r, ray_t),
            Object::Transform(p) => p.hit(r, ray_t),
            Object::Volume(p) => p.hit(r, ray_t),
        }
    }

    fn bounding_box(&self) -> Option<&Aabb> {
        match self {
            Object::Sphere(o) => o.bounding_box(),
            Object::List(o) => o.bounding_box(),
            Object::Node(o) => o.bounding_box(),
            Object::_Plane(o) => o.bounding_box(),
            Object::Quad(o) => o.bounding_box(),
            Object::Transform(o) => o.bounding_box(),
            Object::Volume(o) => o.bounding_box(),
        }
    }
}

#[derive(Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Material,
    center_vec: Option<Vec3>,
    bbox: Aabb,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Material) -> Object {
        let rvec = Vec3::new(radius, radius, radius);
        Object::Sphere(Sphere {
            center,
            radius,
            mat,
            center_vec: None,
            bbox: Aabb::from_points(&(center - rvec), &(center + rvec)),
        })
    }

    pub fn new_moving(center1: Point3, center2: Point3, radius: f64, mat: Material) -> Object {
        let rvec = Vec3::new(radius, radius, radius);
        let box1 = Aabb::from_points(&(center1 - rvec), &(center1 + rvec));
        let box2 = Aabb::from_points(&(center2 - rvec), &(center2 + rvec));
        Object::Sphere(Sphere {
            center: center1,
            radius,
            mat,
            center_vec: Some(center2 - center1),
            bbox: Aabb::from_boxes(&box1, &box2),
        })
    }

    fn center(&self, time: f64) -> Point3 {
        match self.center_vec {
            Some(dir) => self.center + time * dir,
            None => self.center,
        }
    }

    fn get_sphere_uv(p: &Point3) -> (f64, f64) {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;

        let inv_pi = 1.0 / PI;
        (phi * inv_pi * 0.5, theta * inv_pi)
    }

    pub fn _test_uvs() {
        println!("{:?}", Sphere::get_sphere_uv(&Point3::new(1., 0., 0.)));
        println!("{:?}", Sphere::get_sphere_uv(&Point3::new(0., 1., 0.)));
        println!("{:?}", Sphere::get_sphere_uv(&Point3::new(0., 0., 1.)));
        println!("{:?}", Sphere::get_sphere_uv(&Point3::new(-1., 0., 0.)));
        println!("{:?}", Sphere::get_sphere_uv(&Point3::new(0., -1., 0.)));
        println!("{:?}", Sphere::get_sphere_uv(&Point3::new(0., 0., -1.)));
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let center = self.center(r.time());
        let oc = r.origin() - center;
        let a = r.direction().length_squared();
        let half_b = dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (sqrtd - half_b) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let p = r.at(root);
        let outward_normal = (p - center) / self.radius;
        //outward_normal.print(&mut stderr());
        let (u, v) = Sphere::get_sphere_uv(&outward_normal);

        let rec = HitRecord {
            t: root,
            p,
            mat: &self.mat,
            u,
            v,
            normal: Point3::new_zero(),
            front_face: false,
        };

        Some(rec.set_face_normal(r, &outward_normal))
    }

    fn bounding_box(&self) -> Option<&Aabb> {
        Some(&self.bbox)
    }
}

// A disk light source infinitely far away
pub struct Sun {
    pub direction: Vec3,
    albedo: Color,
    limit: f64,
}

impl Sun {
    pub fn new(direction: Vec3, albedo: Color, angular_diameter: f64) -> Sun {
        let limit = 1. - angular_diameter / 180.;

        Sun {
            direction: unit_vector(&direction),
            albedo,
            limit,
        }
    }

    pub fn hit(&self, r: &Ray) -> Color {
        let unit_direction = unit_vector(&r.direction());
        if dot(&unit_direction, &self.direction) > self.limit {
            self.albedo
        } else {
            Color::new_zero()
        }
    }
}

#[derive(Clone)]
pub struct Plane {
    point: Point3,
    normal: Vec3,
    mat: Material,
}
impl Plane {
    pub fn new(point: Point3, normal: Vec3, mat: Material) -> Object {
        Object::_Plane(Plane {
            point,
            normal: unit_vector(&normal),
            mat,
        })
    }
}

impl Hittable for Plane {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let t = dot(&(self.point - r.origin()), &self.normal) / dot(&r.direction(), &self.normal);

        if ray_t.surrounds(t) {
            let rec = HitRecord {
                t,
                p: r.at(t),
                mat: &self.mat,
                u: 0.,
                v: 0.,
                normal: self.normal,
                front_face: false,
            };

            let outward_normal = self.normal;
            Some(rec.set_face_normal(r, &outward_normal))
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Option<&Aabb> {
        None
    }
}

#[derive(Clone, Copy)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    pub fn empty() -> Aabb {
        Aabb {
            x: EMPTY,
            y: EMPTY,
            z: EMPTY,
        }
    }

    pub fn new(x: Interval, y: Interval, z: Interval) -> Aabb {
        Aabb { x, y, z }
    }

    pub fn from_boxes(box0: &Aabb, box1: &Aabb) -> Aabb {
        Aabb {
            x: Interval::from_intervals(&box0.x, &box1.x),
            y: Interval::from_intervals(&box0.y, &box1.y),
            z: Interval::from_intervals(&box0.z, &box1.z),
        }
    }

    pub fn from_points(a: &Point3, b: &Point3) -> Aabb {
        Aabb {
            x: Interval {
                min: a.x().min(b.x()),
                max: a.x().max(b.x()),
            },
            y: Interval {
                min: a.y().min(b.y()),
                max: a.y().max(b.y()),
            },
            z: Interval {
                min: a.z().min(b.z()),
                max: a.z().max(b.z()),
            },
        }
    }

    pub fn axis(&self, n: u8) -> &Interval {
        match n {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Cannot acces the 4th or higher dimension!"),
        }
    }

    pub fn hit(&self, r: &Ray, ray_t: &Interval) -> bool {
        // Override and create copy to get around mutability rules
        let mut ray_t = Interval {
            min: ray_t.min,
            max: ray_t.max,
        };

        for a in 0..3 {
            let inv_d = 1. / r.direction().dim(a);
            let orig = r.origin().dim(a);

            let t0 = (self.axis(a).min - orig) * inv_d;
            let t1 = (self.axis(a).max - orig) * inv_d;

            // swap if less than 0
            let (t0, t1) = if inv_d < 0. { (t1, t0) } else { (t0, t1) };

            if t0 > ray_t.min {
                ray_t.min = t0
            }
            if t1 < ray_t.max {
                ray_t.max = t1
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }

        true
    }

    pub fn pad(&self) -> Aabb {
        let delta = 0.0001;
        Aabb::new(
            if self.x.size() >= delta {
                self.x.clone()
            } else {
                self.x.expand(delta)
            },
            if self.y.size() >= delta {
                self.y.clone()
            } else {
                self.y.expand(delta)
            },
            if self.z.size() >= delta {
                self.z.clone()
            } else {
                self.z.expand(delta)
            },
        )
    }
}

impl ops::Add<Vec3> for Aabb {
    type Output = Aabb;

    fn add(self, offset: Vec3) -> Aabb {
        Aabb::new(
            self.x + offset.x(),
            self.y + offset.y(),
            self.z + offset.z(),
        )
    }
}

impl ops::Add<Aabb> for Vec3 {
    type Output = Aabb;

    fn add(self, bbox: Aabb) -> Aabb {
        bbox + self
    }
}

#[derive(Clone)]
pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    mat: Material,
    bbox: Aabb,
    normal: Vec3,
    d: f64,
    w: Vec3,
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Material) -> Object {
        let bbox = Aabb::from_points(&q, &(q + u + v)).pad();
        let n = cross(&u, &v);
        let normal = unit_vector(&n);
        let w = n / dot(&n, &n);

        Object::Quad(Quad {
            q,
            u,
            v,
            mat,
            bbox,
            normal,
            d: dot(&normal, &q),
            w,
        })
    }
}

impl Hittable for Quad {
    fn bounding_box(&self) -> Option<&Aabb> {
        Some(&self.bbox)
    }

    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let denom = dot(&self.normal, &r.direction());

        // No hit if ray is parallel to plane
        if denom.abs() < 1e-8 {
            return None;
        }

        let t = (self.d - dot(&self.normal, &r.origin())) / denom;
        if !ray_t.contains(t) {
            return None;
        }

        // Determine if hit point is within planar shape using plane coords
        let intersection = r.at(t);
        let planar_hitpt_vector = intersection - self.q;
        let a = dot(&self.w, &cross(&planar_hitpt_vector, &self.v));
        let b = dot(&self.w, &cross(&self.u, &planar_hitpt_vector));

        // If the hit point is in the primitive
        if (a < 0.) || (1. < a) || (b < 0.) || (1. < b) {
            return None;
        }

        // Set rest of hit record
        Some(
            HitRecord {
                t,
                p: intersection,
                mat: &self.mat,
                front_face: true,
                normal: Point3::new_zero(),
                u: a,
                v: b,
            }
            .set_face_normal(r, &self.normal),
        )
    }
}

pub fn make_box(a: &Point3, b: &Point3, mat: &Material) -> Object {
    // Returns the 3D box that contains the two opposite vertices a & b.

    let mut sides = HittableList::new();

    // Construct the two opposite vertices with the minimum and maximum coords.
    let min = Point3::new(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));
    let max = Point3::new(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));

    let dx = Vec3::new(max.x() - min.x(), 0., 0.);
    let dy = Vec3::new(0., max.y() - min.y(), 0.);
    let dz = Vec3::new(0., 0., max.z() - min.z());

    sides.add(Quad::new(
        Point3::new(min.x(), min.y(), max.z()),
        dx,
        dy,
        mat.clone(),
    ));
    sides.add(Quad::new(
        Point3::new(max.x(), min.y(), max.z()),
        -dz,
        dy,
        mat.clone(),
    ));
    sides.add(Quad::new(
        Point3::new(max.x(), min.y(), min.z()),
        -dx,
        dy,
        mat.clone(),
    ));
    sides.add(Quad::new(
        Point3::new(min.x(), min.y(), min.z()),
        dz,
        dy,
        mat.clone(),
    ));
    sides.add(Quad::new(
        Point3::new(min.x(), max.y(), max.z()),
        dx,
        -dz,
        mat.clone(),
    ));
    sides.add(Quad::new(
        Point3::new(min.x(), min.y(), min.z()),
        dx,
        dz,
        mat.clone(),
    ));

    Object::List(Arc::new(sides))
}
