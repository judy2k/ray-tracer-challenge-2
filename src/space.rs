use crate::{approx_equal, matrix::Matrix};

use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

pub type Point = Tuple;
pub type Vector = Tuple;

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    pub fn point(x: f64, y: f64, z: f64) -> Point {
        Self::new(x, y, z, 1.0)
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Vector {
        Self::new(x, y, z, 0.0)
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn get(&self, index: usize) -> f64 {
        match index {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            3 => self.w,
            4_usize.. => todo!(),
        }
    }

    #[allow(unused)]
    fn w(&self) -> f64 {
        self.w
    }

    pub fn is_point(&self) -> bool {
        approx_equal(self.w, 1.0)
    }

    pub fn is_vector(&self) -> bool {
        approx_equal(self.w, 0.0)
    }

    // vector-only?
    pub fn magnitude(&self) -> f64 {
        (self.x.powf(2.) + self.y.powf(2.) + self.z.powf(2.) + self.w.powf(2.)).sqrt()
    }

    // vector-only?
    pub fn normalize(&self) -> Vector {
        let m = self.magnitude();
        Self::new(self.x / m, self.y / m, self.z / m, 0.0)
    }

    // Note for future refactoring: dot is vector-only.
    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w + other.w
    }

    // Note for future refactoring: cross is vector-only (and returns a vector).
    pub fn cross(&self, other: Self) -> Self {
        Self::vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn translate(&self, x: f64, y: f64, z: f64) -> Tuple {
        Matrix::translation(x, y, z) * self
    }

    pub fn scale(&self, x: f64, y: f64, z: f64) -> Tuple {
        Matrix::scaling(x, y, z) * self
    }

    pub fn shear(&self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Tuple {
        Matrix::shearing(xy, xz, yx, yz, zx, zy) * self
    }

    pub fn rotate_x(&self, radians: f64) -> Tuple {
        Matrix::rotation_x(radians) * self
    }

    pub fn rotate_y(&self, radians: f64) -> Tuple {
        Matrix::rotation_y(radians) * self
    }

    pub fn rotate_z(&self, radians: f64) -> Tuple {
        Matrix::rotation_z(radians) * self
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        approx_equal(self.x, other.x)
            && approx_equal(self.y, other.y)
            && approx_equal(self.z, other.z)
            && approx_equal(self.w, other.w)
    }
}

impl Add for Tuple {
    type Output = Tuple;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl Sub for Tuple {
    type Output = Tuple;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}
impl Neg for Tuple {
    type Output = Tuple;
    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Mul<f64> for Tuple {
    type Output = Tuple;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Div<f64> for Tuple {
    type Output = Tuple;
    fn div(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{assert_approx_eq, testlib::approx_equals_fail, EPSILON};

    #[test]
    fn test_tuple_is_point() {
        let t = Tuple::point(4.3, -4.2, 3.1);
        assert_approx_eq!(t.x(), 4.3);
        assert_approx_eq!(t.y(), -4.2);
        assert_approx_eq!(t.z(), 3.1);
        assert_approx_eq!(t.w(), 1.0);
        assert!(t.is_point());
    }

    #[test]
    fn test_tuple_is_vector() {
        let t = Tuple::vector(4.3, -4.2, 3.1);
        assert_approx_eq!(t.x(), 4.3);
        assert_approx_eq!(t.y(), -4.2);
        assert_approx_eq!(t.z(), 3.1);
        assert_approx_eq!(t.w(), 0.0);
        assert!(t.is_vector())
    }

    #[test]
    fn test_point_from_ints() {
        assert_eq!(Tuple::point(1.0, 2.0, 3.0), Tuple::point(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_tuple_eq() {
        let a = Tuple::vector(4.3, -4.2, 3.1);
        let mut b = Tuple::vector(4.3, -4.2, 3.1);

        assert_eq!(a, b);

        b.x += EPSILON / 2.0;
        assert_eq!(a, b);

        b.x += EPSILON;
        assert_ne!(a, b);

        let b = Tuple::point(4.3, -4.2, 3.1);
        assert_ne!(a, b);
    }

    #[test]
    fn test_tuple_add() {
        let a = Tuple::point(3., -2., 5.);
        let b = Tuple::vector(-2., 3., 1.);
        assert_eq!(
            a + b,
            Tuple {
                x: 1.,
                y: 1.,
                z: 6.,
                w: 1.
            }
        );
    }

    #[test]
    fn test_tuple_subtract_two_points() {
        let a = Tuple::point(3., 2., 1.);
        let b = Tuple::point(5., 6., 7.);

        assert_eq!(
            a - b,
            Tuple {
                x: -2.,
                y: -4.,
                z: -6.,
                w: 0.
            }
        );
    }

    #[test]
    fn test_tuple_subtract_vector_from_point() {
        let a = Tuple::point(3., 2., 1.);
        let b = Tuple::vector(5., 6.0, 7.);
        assert_eq!(
            a - b,
            Tuple {
                x: -2.,
                y: -4.,
                z: -6.,
                w: 1.0,
            }
        );
    }

    #[test]
    fn test_tuple_subtract_two_vectors() {
        let a = Tuple::vector(3., 2., 1.);
        let b = Tuple::vector(5., 6.0, 7.);
        assert_eq!(
            a - b,
            Tuple {
                x: -2.,
                y: -4.,
                z: -6.,
                w: 0.0,
            }
        );
    }

    #[test]
    fn test_tuple_subtract_vector_from_zero_vector() {
        let zero = Tuple::vector(0.0, 0.0, 0.0);
        let v = Tuple::vector(1.0, -2.0, 3.0);

        assert_eq!(zero - v, Tuple::vector(-1.0, 2.0, -3.0))
    }

    #[test]
    fn test_tuple_neg() {
        assert_eq!(
            -Tuple {
                x: 1.0,
                y: -2.0,
                z: 3.0,
                w: -4.0,
            },
            Tuple {
                x: -1.0,
                y: 2.0,
                z: -3.0,
                w: 4.0,
            },
        )
    }

    #[test]
    fn test_tuple_mul() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a * 3.5, Tuple::new(3.5, -7.0, 10.5, -14.0));
    }

    #[test]
    fn test_tuple_div() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a / 2.0, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn test_tuple_magnitude() {
        assert_approx_eq!(Tuple::vector(1.0, 0.0, 0.0).magnitude(), 1.0);
        assert_approx_eq!(Tuple::vector(0.0, 1.0, 0.0).magnitude(), 1.0);
        assert_approx_eq!(Tuple::vector(0.0, 0.0, 1.0).magnitude(), 1.0);
        assert_approx_eq!(Tuple::vector(1.0, 2.0, 3.0).magnitude(), 14_f64.sqrt());
        assert_approx_eq!(Tuple::vector(-1.0, -2.0, -3.0).magnitude(), 14_f64.sqrt());
    }

    #[test]
    fn test_tuple_normalize() {
        assert_eq!(
            Tuple::vector(4.0, 0.0, 0.0).normalize(),
            Tuple::vector(1.0, 0.0, 0.0)
        );

        assert_eq!(
            Tuple::vector(1.0, 2.0, 3.0).normalize(),
            Tuple::vector(0.26726, 0.53452, 0.80178)
        );

        assert_approx_eq!(Tuple::vector(1.0, 2.0, 3.0).normalize().magnitude(), 1.0)
    }

    #[test]
    fn test_tuple_dot() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);
        assert_approx_eq!(a.dot(b), 20.0);
    }

    #[test]
    fn test_tuple_cross() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);

        assert_eq!(a.cross(b), Tuple::vector(-1., 2., -1.));
        assert_eq!(b.cross(a), Tuple::vector(1., -2., 1.));
    }
}
