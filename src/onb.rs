use crate::vec3::{unit_vector, Vec3, cross};

pub struct Onb {
    axis: [Vec3; 3],
}

impl Onb {
    pub fn default() -> Self {
        Onb {
            axis: [Vec3::new_zero(), Vec3::new_zero(), Vec3::new_zero()],
        }
    }

    pub fn u(&self) -> Vec3 {
        self.axis[0]
    }
    pub fn v(&self) -> Vec3 {
        self.axis[1]
    }
    pub fn w(&self) -> Vec3 {
        self.axis[2]
    }

    pub fn local(&self, a: f64, b: f64, c: f64) -> Vec3 {
        a * self.u() + b * self.v() + c * self.w()
    }

    pub fn local_vec(&self, a: &Vec3) -> Vec3 {
        self.local(a.x(), a.y(), a.z())
    }

    pub fn build_from_w(&mut self, w: &Vec3) {
        let unit_w = unit_vector(w);

        let a = if unit_w.x().abs() > 0.9 {
            Vec3::new(0., 1., 0.)
        } else {
            Vec3::new(1., 0., 0.)
        };

        let v = unit_vector(&cross(&unit_w, &a));
        let u = cross(&unit_w, &v);

        self.axis[0] = u;
        self.axis[1] = v;
        self.axis[2] = unit_w;
    }
}
