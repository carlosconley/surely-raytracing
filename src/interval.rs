use crate::utils::INF;

pub const _EMPTY: Interval = Interval {
	min: INF,
	max: -INF
};

pub const _UNIVERSE: Interval = Interval {
	min: -INF,
	max: INF
};

pub struct Interval {
	pub min: f64,
	pub max: f64,
}

impl Interval {
	pub fn _contains(&self, x: f64) -> bool {
		self.min <= x && x <= self.max
	}

	pub fn surrounds(&self, x: f64) -> bool {
		self.min < x && x < self.max
	}

	pub fn clamp(&self, x: f64) -> f64 {
		if x < self.min {
			self.min
		} else if x > self.max {
			self.max
		} else {
			x
		}
	}
}

