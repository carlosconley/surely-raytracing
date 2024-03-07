use crate::{
	utils::{random_double, random_int},
	vec3::Point3
};


const POINT_COUNT: usize = 256;
pub struct Perlin {
	ranfloat: Vec<f64>,
	perm_x: Vec<i32>,
	perm_y: Vec<i32>,
	perm_z: Vec<i32>,
}

impl Perlin {
	pub fn new() -> Perlin {
		let mut ranfloat = vec![0.; POINT_COUNT];

		for i in ranfloat.iter_mut() {
			*i = random_double();
		}

		Perlin {
			ranfloat,
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

		let mut c = [[[0.;2] ; 2] ; 2];

		for di in 0..2 {
			for dj in 0..2 {
				for dk in 0..2 {
					c[di as usize][dj as usize][dk as usize] = self.ranfloat[
						(self.perm_x[((i + di) & 255) as usize] ^
						self.perm_y[((j + dj) & 255) as usize] ^
						self.perm_z[((k + dk) & 255) as usize]) as usize
					];
				}
			}
		}

		Perlin::trilinear_interp(c, u, w, v)
	}

	fn trilinear_interp(c: [[[f64; 2]; 2]; 2], u: f64, w: f64, v: f64) -> f64 {
		let mut accum = 0.;

		for _i in 0..2 {
			for _j in 0..2 {
				for _k in 0..2 {
					let i = _i as f64;
					let j = _j as f64;
					let k = _k as f64;
					accum += (i * u + (1. - i) * (1. -u)) *
						(j*v + (1. - j) * ( 1. - v))*
						(k * w + (1. - k) * (1. - w)) * c[_i][_j][_k]
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
