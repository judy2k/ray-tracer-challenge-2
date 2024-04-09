use crate::{approx_equal, matrix::Matrix};

use std::ops::{Add, Deref, DerefMut, Div, Mul, Neg, Sub};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point(Tuple);

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(Tuple::new(x, y, z, 1.0))
    }

    pub fn subtract_origin(&self) -> Vector {
        let mut t = **self;
        t.w = 0.0;
        Vector(t)
    }

    pub fn translate(&self, x: f64, y: f64, z: f64) -> Point {
        Point((**self).translate(x, y, z))
    }

    pub fn scale(&self, x: f64, y: f64, z: f64) -> Point {
        Point((**self).scale(x, y, z))
    }

    pub fn shear(&self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Point {
        Point((**self).shear(xy, xz, yx, yz, zx, zy))
    }

    pub fn rotate_x(&self, x: f64) -> Point {
        Point((**self).rotate_x(x))
    }

    pub fn rotate_y(&self, y: f64) -> Point {
        Point((**self).rotate_y(y))
    }

    pub fn rotate_z(&self, z: f64) -> Point {
        Point((**self).rotate_z(z))
    }
}

impl From<Tuple> for Point {
    fn from(value: Tuple) -> Self {
        Self(value)
    }
}

impl Add<Vector> for Point {
    type Output = Point;
    fn add(self, rhs: Vector) -> Self::Output {
        Point(*self + *rhs)
    }
}

impl Add<&Vector> for &Point {
    type Output = Point;
    fn add(self, rhs: &Vector) -> Self::Output {
        Point(**self + **rhs)
    }
}

impl Sub<Point> for Point {
    type Output = Vector;
    fn sub(self, rhs: Point) -> Self::Output {
        Vector(*self - *rhs)
    }
}

impl Sub<&Point> for &Point {
    type Output = Vector;
    fn sub(self, rhs: &Point) -> Self::Output {
        Vector(**self - **rhs)
    }
}

impl Sub<Vector> for Point {
    type Output = Point;
    fn sub(self, rhs: Vector) -> Self::Output {
        Point(*self - *rhs)
    }
}

impl Sub<&Vector> for &Point {
    type Output = Point;
    fn sub(self, rhs: &Vector) -> Self::Output {
        Point(**self - **rhs)
    }
}

impl Deref for Point {
    type Target = Tuple;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Point {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector(Tuple);

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(Tuple::new(x, y, z, 0.0))
    }

    pub fn translate(&self, x: f64, y: f64, z: f64) -> Vector {
        Vector((**self).translate(x, y, z))
    }

    pub fn scale(&self, x: f64, y: f64, z: f64) -> Vector {
        Vector((**self).scale(x, y, z))
    }

    pub fn rotate_x(&self, x: f64) -> Vector {
        Vector((**self).rotate_x(x))
    }

    pub fn rotate_y(&self, y: f64) -> Vector {
        Vector((**self).rotate_y(y))
    }

    pub fn rotate_z(&self, z: f64) -> Vector {
        Vector((**self).rotate_z(z))
    }

    pub fn shear(&self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Vector {
        Vector((**self).shear(xy, xz, yx, yz, zx, zy))
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powf(2.) + self.y.powf(2.) + self.z.powf(2.) + self.w.powf(2.)).sqrt()
    }

    pub fn normalize(&self) -> Vector {
        let m = self.magnitude();
        Vector::new(self.x / m, self.y / m, self.z / m)
    }

    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w + other.w
    }

    pub fn cross(&self, other: Self) -> Vector {
        Vector::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn reflect(&self, normal: &Vector) -> Vector {
        *self - normal * 2.0 * self.dot(*normal)
    }
}

impl From<Tuple> for Vector {
    fn from(value: Tuple) -> Self {
        let mut value = value;
        value.w = 0.0;
        Self(value)
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;
    fn add(self, rhs: Vector) -> Self::Output {
        Vector(*self + *rhs)
    }
}

impl Add<&Vector> for &Vector {
    type Output = Vector;
    fn add(self, rhs: &Vector) -> Self::Output {
        Vector(**self + **rhs)
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;
    fn sub(self, rhs: Vector) -> Self::Output {
        Vector(*self - *rhs)
    }
}

impl Sub<&Vector> for &Vector {
    type Output = Vector;
    fn sub(self, rhs: &Vector) -> Self::Output {
        Vector(**self - **rhs)
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;
    fn mul(self, rhs: f64) -> Self::Output {
        Vector(*self * rhs)
    }
}

impl Mul<f64> for &Vector {
    type Output = Vector;
    fn mul(self, rhs: f64) -> Self::Output {
        Vector(**self * rhs)
    }
}

impl Deref for Vector {
    type Target = Tuple;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Vector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    pub fn point(x: f64, y: f64, z: f64) -> Point {
        Point::new(x, y, z)
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Vector {
        Vector::new(x, y, z)
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
        let t = Point::new(4.3, -4.2, 3.1);
        assert_approx_eq!(t.x(), 4.3);
        assert_approx_eq!(t.y(), -4.2);
        assert_approx_eq!(t.z(), 3.1);
        assert_approx_eq!(t.w(), 1.0);
        assert!(t.is_point());
    }

    #[test]
    fn test_tuple_is_vector() {
        let t = Vector::new(4.3, -4.2, 3.1);
        assert_approx_eq!(t.x(), 4.3);
        assert_approx_eq!(t.y(), -4.2);
        assert_approx_eq!(t.z(), 3.1);
        assert_approx_eq!(t.w(), 0.0);
        assert!(t.is_vector())
    }

    #[test]
    fn test_point_from_ints() {
        assert_eq!(Point::new(1.0, 2.0, 3.0), Point::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_tuple_eq() {
        let a = Vector::new(4.3, -4.2, 3.1);
        let mut b = Vector::new(4.3, -4.2, 3.1);

        assert_eq!(a, b);

        b.x += EPSILON / 2.0;
        assert_eq!(a, b);

        b.x += EPSILON;
        assert_ne!(a, b);

        // Type system prevents comparing point with a vector.
        // let b = Point::new(4.3, -4.2, 3.1);
        // assert_ne!(a, b);
    }

    #[test]
    fn test_tuple_add() {
        let a = Point::new(3., -2., 5.);
        let b = Vector::new(-2., 3., 1.);
        assert_eq!(a + b, Point::new(1., 1., 6.));
    }

    #[test]
    fn test_tuple_subtract_two_points() {
        let a = Point::new(3., 2., 1.);
        let b = Point::new(5., 6., 7.);

        assert_eq!(
            *(a - b),
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
        let a = Point::new(3., 2., 1.);
        let b = Vector::new(5., 6.0, 7.);
        assert_eq!(
            *(a - b),
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
        let a = Vector::new(3., 2., 1.);
        let b = Vector::new(5., 6.0, 7.);
        assert_eq!(
            *(a - b),
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
        let zero = Vector::new(0.0, 0.0, 0.0);
        let v = Vector::new(1.0, -2.0, 3.0);

        assert_eq!(zero - v, Vector::new(-1.0, 2.0, -3.0))
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
        assert_approx_eq!(Vector::new(1.0, 0.0, 0.0).magnitude(), 1.0);
        assert_approx_eq!(Vector::new(0.0, 1.0, 0.0).magnitude(), 1.0);
        assert_approx_eq!(Vector::new(0.0, 0.0, 1.0).magnitude(), 1.0);
        assert_approx_eq!(Vector::new(1.0, 2.0, 3.0).magnitude(), 14_f64.sqrt());
        assert_approx_eq!(Vector::new(-1.0, -2.0, -3.0).magnitude(), 14_f64.sqrt());
    }

    #[test]
    fn test_tuple_normalize() {
        assert_eq!(
            Vector::new(4.0, 0.0, 0.0).normalize(),
            Vector::new(1.0, 0.0, 0.0)
        );

        assert_eq!(
            Vector::new(1.0, 2.0, 3.0).normalize(),
            Vector::new(0.26726, 0.53452, 0.80178)
        );

        assert_approx_eq!(Vector::new(1.0, 2.0, 3.0).normalize().magnitude(), 1.0)
    }

    #[test]
    fn test_tuple_dot() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(2.0, 3.0, 4.0);
        assert_approx_eq!(a.dot(b), 20.0);
    }

    #[test]
    fn test_tuple_cross() {
        let a = Vector::new(1.0, 2.0, 3.0);
        let b = Vector::new(2.0, 3.0, 4.0);

        assert_eq!(a.cross(b), Vector::new(-1., 2., -1.));
        assert_eq!(b.cross(a), Vector::new(1., -2., 1.));
    }

    #[test]
    fn test_reflect_45() {
        let v = Vector::new(1.0, -1.0, 0.0);
        let n = Vector::new(0.0, 1.0, 0.0);
        let r = v.reflect(&n);
        assert_eq!(r, Vector::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn test_reflect_slanted() {
        let hsq = (2.0_f64).sqrt() / 2.0;
        let v = Vector::new(0.0, -1.0, 0.0);
        let n = Vector::new(hsq, hsq, 0.0);

        assert_eq!(v.reflect(&n), Vector::new(1.0, 0.0, 0.0));
    }
}
