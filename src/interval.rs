use crate::utils::INF;

pub const EMPTY: Interval = Interval {
	min: INF,
	max: -INF
};

pub const _UNIVERSE: Interval = Interval {
	min: -INF,
	max: INF
};

#[derive(Clone)]
pub struct Interval {
	pub min: f64,
	pub max: f64,
}

impl Interval {
	pub fn contains(&self, x: f64) -> bool {
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

	pub fn size(&self) -> f64 {
		self.max - self.min
	}

	pub fn expand(&self, delta: f64) -> Interval {
		let padding = delta / 2.;

		Interval { min: self.min - padding, max: self.max + padding }
	}

	pub fn from_intervals(a: &Interval, b: &Interval) -> Interval {
		Interval {
			min: a.min.min(b.min),
			max: a.max.max(b.max)
		}
	}

}

