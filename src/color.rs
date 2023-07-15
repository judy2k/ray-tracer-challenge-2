use crate::approx_equal;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy)]
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

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        approx_equal(self.r, other.r)
            && approx_equal(self.g, other.g)
            && approx_equal(self.b, other.b)
    }
}

impl Add for Color {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Sub for Color {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl Mul for Color {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
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

    #[test]
    fn test_color_add() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        assert_eq!(c1 + c2, Color::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn test_color_subtract() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        assert_eq!(c1 - c2, Color::new(0.2, 0.5, 0.5));
    }

    #[test]
    fn test_color_multiply_scalar() {
        let c1 = Color::new(0.2, 0.3, 0.4);

        assert_eq!(c1 * 2.0, Color::new(0.4, 0.6, 0.8));
    }

    #[test]
    fn test_color_multiply() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);

        assert_eq!(c1 * c2, Color::new(0.9, 0.2, 0.04));
    }
}
