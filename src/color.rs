pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    pub fn red(self) -> f64 {
        self.r
    }

    pub fn green(self) -> f64 {
        self.g
    }

    pub fn blue(self) -> f64 {
        self.b
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_approx_eq;
    use crate::testlib::approx_equals_fail;

    #[test]
    fn test_color_construction() {
        let c = Color::new(-0.5, 0.4, 1.7);
        assert_approx_eq!(c.r, -0.5);
    }
}
