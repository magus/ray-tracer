pub struct Interval {
    min: f64,
    max: f64,
}

impl Interval {
    pub const fn new(min: f64, max: f64) -> Interval {
        Interval { min, max }
    }

    pub fn empty() -> Interval {
        Interval {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
        }
    }

    pub fn universe() -> Interval {
        Interval {
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
        }
    }

    pub fn min(&self) -> f64 {
        self.min
    }

    pub fn max(&self) -> f64 {
        self.max
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fields() {
        let i = Interval::new(-4.0, 6.0);
        assert_eq!(i.min(), -4.0);
        assert_eq!(i.max(), 6.0);
    }

    #[test]
    fn test_empty() {
        let i = Interval::empty();
        assert_eq!(i.min(), f64::INFINITY);
        assert_eq!(i.max(), f64::NEG_INFINITY);
    }

    #[test]
    fn test_universe() {
        let i = Interval::universe();
        assert_eq!(i.min(), f64::NEG_INFINITY);
        assert_eq!(i.max(), f64::INFINITY);
    }

    #[test]
    fn test_size() {
        let i = Interval::new(-4.0, 6.0);
        assert_eq!(i.size(), 10.0);
    }

    #[test]
    fn test_contains() {
        let i = Interval::new(-4.0, 6.0);
        assert!(i.contains(-4.0));
        assert!(i.contains(1.5));
        assert!(i.contains(6.0));
    }

    #[test]
    fn test_surrounds() {
        let i = Interval::new(-4.0, 6.0);
        assert!(!i.surrounds(-4.0));
        assert!(i.surrounds(3.4));
        assert!(!i.surrounds(6.0));
    }

    #[test]
    fn test_clamp() {
        let i = Interval::new(-4.0, 6.0);
        assert_eq!(i.clamp(-5.0), -4.0);
        assert_eq!(i.clamp(3.4), 3.4);
        assert_eq!(i.clamp(10.0), 6.0);
    }
}
