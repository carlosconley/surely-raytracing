use crate::utils::{random_double, random_range};
use std::io::Write;
use std::ops;

//extern crate nalgebra as na;
//use na::Vector3;

//pub type Vec3 = Vector3<f64>;
pub type Point3 = Vec3;

/*pub fn print<W: Write>(out: &mut W, v: &Vec3) {
    write!(
        out,
        "{} {} {}",
        v.x, v.y, v.z
    ).expect("Couldn't print vector");
}*/

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    // Constructors
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn new_zero() -> Self {
        Vec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }

    /*pub fn clone(&self) -> Self {
        Vec3::new(self.x, self.y, self.z)
    }*/

    // Accesors
    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn dim(&self, n: u8) -> f64 {
        match n {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Cannot access the 4th or higher dimension!"),
        }
    }

    pub fn length_squared(&self) -> f64 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn print<W: Write>(&self, out: &mut W) {
        write!(out, "{} {} {}", self.x, self.y, self.z).expect("Vec3 could not be print");
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }
}

// Operator overloads
impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x - v.x, self.y - v.y, self.z - v.z)
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x * v.x, self.y * v.y, self.z * v.z)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f64) -> Vec3 {
        Vec3::new(self.x * t, self.y * t, self.z * t)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x / v.x, self.y / v.y, self.z / v.z)
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f64) -> Vec3 {
        Vec3::new(self.x / t, self.y / t, self.z / t)
    }
}

impl ops::Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, v: Vec3) -> Vec3 {
        v / self
    }
}

// Utils

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.x * v.x + u.y * v.y + u.z * v.z
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3::new(
        u.y * v.z - u.z * v.y,
        u.z * v.x - u.x * v.z,
        u.x * v.y - u.y * v.x,
    )
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
    v.clone() / v.length()
}

// Random generators
pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(random_range(-1., 1.), random_range(-1., 1.), 0.);
        if p.length_squared() < 1. {
            return p;
        }
    }
}

pub fn random_vec3() -> Vec3 {
    Vec3::new(random_double(), random_double(), random_double())
}

pub fn random_vec3_range(min: f64, max: f64) -> Vec3 {
    Vec3::new(
        random_range(min, max),
        random_range(min, max),
        random_range(min, max),
    )
}

pub fn _random_on_hemisphere(normal: &Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();

    if dot(&on_unit_sphere, normal) > 0. {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

pub fn random_unit_vector() -> Vec3 {
    unit_vector(&random_in_unit_sphere())
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2. * dot(v, n) * *n
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = dot(&-*uv, n).min(1.);
    let r_out_perp = etai_over_etat * (*uv + cos_theta * *n);
    let r_out_parallel = (1.0 - r_out_perp.length_squared()).abs().sqrt() * -1. * *n;

    r_out_perp + r_out_parallel
}

fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = random_vec3_range(-1., 1.);
        if p.length_squared() < 1. {
            return p;
        }
    }
}
