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
		let i = (4. * p.x()) as i32 & 255;
		let j = (4. * p.y()) as i32 & 255;
		let k = (4. * p.z()) as i32 & 255;

		self.ranfloat[(self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]) as usize]
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