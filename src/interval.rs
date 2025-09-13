//! Interval object

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
/// Linear float interval from min to max
pub struct Interval {
    pub min: f64,
    pub max: f64,
}
impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Interval { min: min, max: max }
    }
    /// Create new interval where min is always < max
    pub fn new_bound_check(a: f64, b: f64) -> Self {
        match a <= b {
            true => Interval::new(a, b),
            false => Interval::new(b, a),
        }
    }
    /// Create new interval from two other intervals with the result tightly
    /// enclosing both
    pub fn new_enclosing(a: &Interval, b: &Interval) -> Self {
        let min = if a.min <= b.min { a.min } else { b.min };
        let max = if a.max >= b.max { a.max } else { b.max };
        Interval::new(min, max)
    }
    pub fn size(&self) -> f64 {
        self.max - self.min
    }
    /// Returns true if x is in interval, bound inclusive
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && self.max >= x
    }
    /// Returns true if x is in interval, bound exclusive
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && self.max > x
    }
    /// Returns the nearest value in the interval to x.
    /// If x is in the interval returns x, else returns either the
    /// maximum or minimum
    pub fn itv_clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        } else if x > self.max {
            return self.max;
        }
        return x;
    }
    /// Adds padding delta/2 to each end of interval and returns new interval
    pub fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.0;
        Interval::new(self.min - padding, self.max + padding)
    }
}

impl std::cmp::Eq for Interval {}
impl std::cmp::Ord for Interval {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.size() < other.size() {
            std::cmp::Ordering::Less
        } else if self.size() > other.size() {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    }
}
#[test]
fn test_ord() {
    let i1 = Interval::new(0.0, 10.0);
    let i2 = Interval::new(-10.0, -5.0);
    let i3 = Interval::new(0.0, 10.0);
    assert!(i1 > i2);
    assert!(i1 == i3);
}
