use crate::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Interval {
    pub min: float,
    pub max: float,
}

impl Interval {
    pub const EMPTY: Interval = Interval::new(float::INFINITY, float::NEG_INFINITY);
    pub const UNIVERSE: Interval = Interval::new(float::NEG_INFINITY, float::INFINITY);
    pub const NOT_NEGATIVE: Interval = Interval::new(0.0, float::INFINITY);

    #[inline]
    pub const fn new(min: float, max: float) -> Self {
        Self { min, max }
    }

    #[inline]
    pub fn contains(&self, value: float) -> bool {
        value >= self.min && value <= self.max
    }

    #[inline]
    pub fn contains_some(&self, value: float) -> Option<float> {
        self.contains(value).then_some(value)
    }

    #[inline]
    pub fn surrounds(&self, value: float) -> bool {
        value > self.min && value < self.max
    }

    #[inline]
    pub fn surrounds_some(&self, value: float) -> Option<float> {
        self.surrounds(value).then_some(value)
    }

    #[inline]
    pub fn clamp(&self, value: float) -> float {
        value.clamp(self.min, self.max)
    }

    #[inline]
    pub fn with_max(self, new_max: float) -> Self {
        Self::new(self.min, new_max)
    }
}
