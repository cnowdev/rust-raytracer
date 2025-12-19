use crate::rtweekend::INFINITY;

pub struct Interval {
    pub min: f64,
    pub max: f64
}

impl Interval {
    pub const fn new(min: f64, max: f64) -> Interval {
        Interval {min, max}
    }

    // Static constant: empty interval (contains nothing)
    pub const EMPTY: Interval = Interval { 
        min: INFINITY, 
        max: -INFINITY 
    };

    // Static constant: universe interval (contains everything)
    pub const UNIVERSE: Interval = Interval { 
        min: -INFINITY, 
        max: INFINITY 
    };

    pub fn new_empty() -> Interval {
        Interval {min: -1.0 * INFINITY, max: INFINITY}
    }

    pub fn size(&self) -> f64 {
        return self.max - self.min;
    }

    pub fn contains(&self, x: f64) -> bool {
        return (self.min <= x) && (x <= self.max)
    }

    pub fn surrounds(&self, x: f64) -> bool {
        return (self.min < x) && (x < self.max)
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        } else if x > self.max {
            return self.max;
        } else {
            return x;
        }
    }
}