use crate::{
    utils::{random_int},
    vec3::{dot, random_vec3_range, unit_vector, Point3, Vec3},
};

const POINT_COUNT: usize = 256;
pub struct Perlin {
    ranvec: Vec<Vec3>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Perlin {
    pub fn new() -> Perlin {
        let mut ranvec = vec![Vec3::new_zero(); POINT_COUNT];

        for i in ranvec.iter_mut() {
            *i = unit_vector(&random_vec3_range(-1., 1.));
        }

        Perlin {
            ranvec,
            perm_x: Perlin::generate_perm(),
            perm_y: Perlin::generate_perm(),
            perm_z: Perlin::generate_perm(),
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let mut c = [[[Vec3::new_zero(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di as usize][dj as usize][dk as usize] = self.ranvec[(self.perm_x
                        [((i + di) & 255) as usize]
                        ^ self.perm_y[((j + dj) & 255) as usize]
                        ^ self.perm_z[((k + dk) & 255) as usize])
                        as usize];
                }
            }
        }

        Perlin::trilinear_interp(c, u, w, v)
    }

    pub fn turb_depth(&self, p: &Point3, depth: u32) -> f64 {
        let mut accum = 0.;
        let mut temp_p = *p;
        let mut weight = 1.;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p = temp_p * 2.;
        }

        accum.abs()
    }

    pub fn turb(&self, p: &Point3) -> f64 {
        self.turb_depth(p, 7)
    }

    fn trilinear_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, w: f64, v: f64) -> f64 {
        let uu = u * u * (3. - 2. * u);
        let vv = v * v * (3. - 2. * v);
        let ww = w * w * (3. - 2. * w);
        let mut accum = 0.;

        for _i in 0..2 {
            for _j in 0..2 {
                for _k in 0..2 {
                    let i = _i as f64;
                    let j = _j as f64;
                    let k = _k as f64;
                    let weight_v = Vec3::new(u - i, v - j, w - k);
                    accum += (i * uu + (1. - i) * (1. - uu))
                        * (j * vv + (1. - j) * (1. - vv))
                        * (k * ww + (1. - k) * (1. - ww))
                        * dot(&c[_i][_j][_k], &weight_v);
                }
            }
        }

        accum
    }
    fn generate_perm() -> Vec<i32> {
        let mut p = vec![0; POINT_COUNT];

        for i in 0..POINT_COUNT {
            p[i] = i as i32;
        }

        Perlin::permute(&mut p, POINT_COUNT);

        p
    }

    fn permute(p: &mut Vec<i32>, n: usize) {
        for i in (0..n).rev() {
            let target = random_int(0, i as i64) as usize;
            let tmp = p[i];

            p[i] = p[target];
            p[target] = tmp;
        }
    }
}
