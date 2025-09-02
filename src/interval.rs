//! Interval object

pub struct Interval {
    pub min: f64,
    pub max: f64
}
impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Interval { min:min, max: max }
    }
    pub fn size(&self) -> f64 {
        self.max - self.min
    }
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && self.max >= x
    }
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && self.max > x
    }
    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        } else if x > self.max {
            return self.max;
        }
        return x;
    }
}