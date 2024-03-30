use crate::approx_equal;
use crate::space::{Point, Tuple, Vector};
use once_cell::sync::OnceCell;
use std::{fmt::Debug, ops::Mul};

static IDENTITY_MATRIX: OnceCell<Matrix> = OnceCell::new();

pub fn identity_matrix() -> &'static Matrix {
    IDENTITY_MATRIX.get_or_init(|| {
        Matrix::from_values(
            4,
            4,
            vec![
                1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
            ],
        )
    })
}

#[derive(Clone)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    values: Vec<f64>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            values: vec![0.0; rows * cols],
        }
    }

    pub fn from_values(rows: usize, cols: usize, values: Vec<f64>) -> Self {
        Self { rows, cols, values }
    }

    pub fn set(&mut self, row: usize, col: usize, value: f64) {
        let index = self.index(row, col);
        match self.values.get_mut(index) {
            Some(val) => *val = value,
            None => panic!("{row}, {col} is out of bounds for {self:?}")
        }
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.values.get(self.index(row, col)).copied().unwrap()
    }

    pub fn transpose(&self) -> Matrix {
        let mut result = Matrix::new(self.cols, self.rows);
        for row in 0..self.rows {
            for col in 0..self.cols {
                result.set(col, row, self.get(row, col));
            }
        }
        result
    }

    fn index(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    pub fn determinant(&self) -> f64 {
        if self.rows == 2 && self.cols == 2 {
            self.get(0, 0) * self.get(1, 1) - self.get(0, 1) * self.get(1, 0)
        } else {
            (0..self.cols)
                .map(|col| self.get(0, col) * self.cofactor(0, col))
                .sum()
        }
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix {
        let mut result = Matrix::new(self.rows - 1, self.cols - 1);
        for or in 0..result.rows {
            for oc in 0..result.cols {
                result.set(
                    or,
                    oc,
                    self.get(
                        if or < row { or } else { or + 1 },
                        if oc < col { oc } else { oc + 1 },
                    ),
                );
            }
        }
        result
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let modifier = if (row + col) % 2 == 1 { -1. } else { 1. };
        self.minor(row, col) * modifier
    }

    pub fn invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    pub fn inverse(&self) -> Option<Matrix> {
        if self.invertible() {
            let mut result = Matrix::new(self.cols, self.rows);
            let determinant = self.determinant();
            for row in 0..self.rows {
                for col in 0..self.cols {
                    let c = self.cofactor(row, col);
                    result.set(col, row, c / determinant)
                }
            }
            Some(result)
        } else {
            None
        }
    }

    pub fn translation(x: f64, y: f64, z: f64) -> Self {
        let mut result = identity_matrix().clone();
        result.set(0, 3, x);
        result.set(1, 3, y);
        result.set(2, 3, z);
        result
    }

    pub fn scaling(x: f64, y: f64, z: f64) -> Self {
        let mut result = identity_matrix().clone();
        result.set(0, 0, x);
        result.set(1, 1, y);
        result.set(2, 2, z);
        result
    }

    pub fn rotation_x(radians: f64) -> Self {
        Self::from_values(
            4,
            4,
            vec![
                1.0,
                0.0,
                0.0,
                0.0,
                0.0,
                radians.cos(),
                -radians.sin(),
                0.0,
                0.0,
                radians.sin(),
                radians.cos(),
                0.0,
                0.0,
                0.0,
                0.0,
                1.0,
            ],
        )
    }

    pub fn rotation_y(radians: f64) -> Self {
        Self::from_values(
            4,
            4,
            vec![
                radians.cos(),
                0.0,
                radians.sin(),
                0.0,
                0.0,
                1.0,
                0.0,
                0.0,
                -(radians.sin()),
                0.0,
                radians.cos(),
                0.0,
                0.0,
                0.0,
                0.0,
                1.0,
            ],
        )
    }

    pub fn rotation_z(radians: f64) -> Self {
        Self::from_values(
            4,
            4,
            vec![
                radians.cos(),
                -radians.sin(),
                0.0,
                0.0,
                radians.sin(),
                radians.cos(),
                0.0,
                0.0,
                0.0,
                0.0,
                1.0,
                0.0,
                0.0,
                0.0,
                0.0,
                1.0,
            ],
        )
    }

    pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        Self::from_values(
            4,
            4,
            vec![
                1.0, xy, xz, 0.0, yx, 1.0, yz, 0.0, zx, zy, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        )
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.rows == other.rows && self.cols == other.cols {
            for row in 0..self.rows {
                for col in 0..self.cols {
                    if !approx_equal(self.get(row, col), other.get(row, col)) {
                        return false;
                    }
                }
            }
            return true;
        }
        false
    }
}

impl Debug for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Matrix<{}, {}> [", self.rows, self.cols)?;
        for row in 0..self.rows {
            writeln!(
                f,
                "  {:?}",
                &self.values[self.index(row, 0)..self.index(row, 0) + self.cols]
            )?;
        }
        writeln!(f, "]")?;

        Ok(())
    }
}

impl Mul for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut values = vec![0.0; self.rows * rhs.cols];
        for row in 0..self.rows {
            for col in 0..rhs.cols {
                let mut tally: f64 = 0.0;
                for i in 0..self.rows {
                    tally += self.get(row, i) * rhs.get(i, col);
                }
                *values.get_mut(self.index(row, col)).unwrap() = tally;
            }
        }

        Matrix::from_values(self.rows, rhs.cols, values)
    }
}

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        self.mul(&rhs)
    }
}

impl Mul<&Self> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &Self) -> Self::Output {
        (&self).mul(rhs)
    }
}

impl Mul<&Tuple> for &Matrix {
    type Output = Tuple;
    fn mul(self, rhs: &Tuple) -> Self::Output {
        let result_params: Vec<f64> = (0..4)
            .map(|row| {
                (0..self.cols)
                    .map(|col| self.get(row, col) * rhs.get(col))
                    .sum()
            })
            .collect();

        Tuple::new(
            result_params[0],
            result_params[1],
            result_params[2],
            result_params[3],
        )
    }
}

impl Mul<&Tuple> for Matrix {
    type Output = Tuple;
    fn mul(self, rhs: &Tuple) -> Self::Output {
        (&self).mul(rhs)
    }
}

impl Mul<Tuple> for Matrix {
    type Output = Tuple;
    fn mul(self, rhs: Tuple) -> Self::Output {
        self.mul(&rhs)
    }
}

impl Mul<Tuple> for &Matrix {
    type Output = Tuple;
    fn mul(self, rhs: Tuple) -> Self::Output {
        self.mul(&rhs)
    }
}

impl Mul<Point> for Matrix {
    type Output = Point;
    fn mul(self, rhs: Point) -> Self::Output {
        self.mul(*rhs).into()
    }
}

impl Mul<Point> for &Matrix {
    type Output = Point;
    fn mul(self, rhs: Point) -> Self::Output {
        self.mul(*rhs).into()
    }
}

impl Mul<Vector> for Matrix {
    type Output = Vector;
    fn mul(self, rhs: Vector) -> Self::Output {
        self.mul(*rhs).into()
    }
}

impl Mul<Vector> for &Matrix {
    type Output = Vector;
    fn mul(self, rhs: Vector) -> Self::Output {
        self.mul(*rhs).into()
    }
}

#[cfg(test)]
mod test {
    use std::f64::consts::PI;

    use super::*;
    use crate::space::*;

    #[test]
    fn test_matrix_construction_4x4() {
        let m = Matrix::from_values(
            4,
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5,
                16.5,
            ],
        );

        assert_eq!(m.get(0, 0), 1.);
        assert_eq!(m.get(0, 3), 4.);
        assert_eq!(m.get(1, 0), 5.5);
        assert_eq!(m.get(1, 2), 7.5);
        assert_eq!(m.get(2, 2), 11.);
        assert_eq!(m.get(3, 0), 13.5);
        assert_eq!(m.get(3, 2), 15.5);
    }

    #[test]
    fn test_matrix_construction_3x3() {
        let m = Matrix::from_values(3, 3, vec![-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0]);

        assert_eq!(m.get(0, 0), -3.0);
        assert_eq!(m.get(1, 1), -2.0);
        assert_eq!(m.get(2, 2), 1.0);
    }

    #[test]
    fn test_matrix_construction_2x2() {
        let m = Matrix::from_values(2, 2, vec![-3.0, 5.0, 1.0, -2.0]);
        assert_eq!(m.get(0, 0), -3.0);
        assert_eq!(m.get(0, 1), 5.0);
        assert_eq!(m.get(1, 0), 1.0);
        assert_eq!(m.get(1, 1), -2.0);
    }

    #[test]
    fn test_partial_eq() {
        let m1 = Matrix::from_values(
            4,
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
            ],
        );
        let m2 = Matrix::from_values(
            4,
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
            ],
        );

        assert_eq!(m1, m2);
    }

    #[test]
    fn test_multiplication() {
        let m1 = Matrix::from_values(
            4,
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
            ],
        );
        let m2 = Matrix::from_values(
            4,
            4,
            vec![
                -2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 4.0, 3.0, 6.0, 5.0, 1.0, 2.0, 7.0, 8.0,
            ],
        );

        let result = Matrix::from_values(
            4,
            4,
            vec![
                20.0, 22.0, 50.0, 48.0, 44.0, 54.0, 114.0, 108.0, 40.0, 58.0, 110.0, 102.0, 16.0,
                26.0, 46.0, 42.0,
            ],
        );

        assert_eq!(m1 * m2, result);
    }

    #[test]
    fn test_tuple_multiplication() {
        let m = Matrix::from_values(
            4,
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0,
            ],
        );

        let t = Tuple::point(1.0, 2.0, 3.0);

        let result = Tuple::point(18., 24., 33.);

        assert_eq!(m * t, result);
    }

    #[test]
    fn test_multiply_by_identity() {
        let m = Matrix::from_values(
            4,
            4,
            vec![
                0.0, 1.0, 2.0, 4.0, 1.0, 2.0, 4.0, 8.0, 2.0, 4.0, 8.0, 16.0, 4.0, 8.0, 16.0, 32.0,
            ],
        );

        assert_eq!(m.clone() * identity_matrix(), m);
        assert_eq!(&m * identity_matrix(), m);
    }

    #[test]
    fn test_transpose() {
        let m = Matrix::from_values(
            4,
            4,
            vec![
                0.0, 9.0, 3.0, 0.0, 9.0, 8.0, 0.0, 8.0, 1.0, 8.0, 5.0, 3.0, 0.0, 0.0, 5.0, 8.0,
            ],
        );

        let result = Matrix::from_values(
            4,
            4,
            vec![
                0.0, 9.0, 1.0, 0.0, 9.0, 8.0, 8.0, 0.0, 3.0, 0.0, 5.0, 5.0, 0.0, 8.0, 3.0, 8.0,
            ],
        );

        assert_eq!(m.transpose(), result);

        assert_eq!(&identity_matrix().transpose(), identity_matrix());
    }

    #[test]
    fn test_determinant_2x2() {
        let m = Matrix::from_values(2, 2, vec![1., 5., -3., 2.]);

        assert_eq!(m.determinant(), 17.0);
    }

    #[test]
    fn test_determinant_3x3() {
        let m = Matrix::from_values(3, 3, vec![1.0, 2.0, 6.0, -5.0, 8.0, -4.0, 2.0, 6.0, 4.0]);

        assert_eq!(m.cofactor(0, 0), 56.0);
        assert_eq!(m.cofactor(0, 1), 12.0);
        assert_eq!(m.cofactor(0, 2), -46.0);
        assert_eq!(m.determinant(), -196.0);
    }

    #[test]
    fn test_determinant_4x4() {
        let m = Matrix::from_values(
            4,
            4,
            vec![
                -2.0, -8.0, 3.0, 5.0, -3.0, 1.0, 7.0, 3.0, 1.0, 2.0, -9.0, 6.0, -6.0, 7.0, 7.0,
                -9.0,
            ],
        );

        assert_eq!(m.cofactor(0, 0), 690.0);
        assert_eq!(m.cofactor(0, 1), 447.0);
        assert_eq!(m.cofactor(0, 2), 210.0);
        assert_eq!(m.cofactor(0, 3), 51.0);
        assert_eq!(m.determinant(), -4071.0);
    }

    #[test]
    fn test_submatrix() {
        let m1 = Matrix::from_values(3, 3, vec![1.0, 5.0, 0.0, -3.0, 2.0, 7.0, 0.0, 6.0, -3.0]);
        let r1 = Matrix::from_values(2, 2, vec![-3., 2., 0., 6.]);

        assert_eq!(m1.submatrix(0, 2), r1);

        let m2 = Matrix::from_values(
            4,
            4,
            vec![
                -6.0, 1.0, 1.0, 6.0, -8.0, 5.0, 8.0, 6.0, -1.0, 0.0, 8.0, 2.0, -7.0, 1.0, -1.0, 1.0,
            ],
        );
        let r2 = Matrix::from_values(3, 3, vec![-6.0, 1.0, 6.0, -8.0, 8.0, 6.0, -7.0, -1.0, 1.0]);
        assert_eq!(m2.submatrix(2, 1), r2);
    }

    #[test]
    fn test_minor() {
        let m = Matrix::from_values(3, 3, vec![3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0]);

        assert_eq!(m.minor(1, 0), 25.);
    }

    #[test]
    fn test_cofactor() {
        let m = Matrix::from_values(3, 3, vec![3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0]);

        assert_eq!(m.minor(0, 0), -12.);
        assert_eq!(m.cofactor(0, 0), -12.);
        assert_eq!(m.minor(1, 0), 25.);
        assert_eq!(m.cofactor(1, 0), -25.);
    }

    #[test]
    fn test_invertible() {
        let invertible = Matrix::from_values(
            4,
            4,
            vec![
                6.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 6.0, 4.0, -9.0, 3.0, -7.0, 9.0, 1.0, 7.0, -6.0,
            ],
        );

        let non_invertible = Matrix::from_values(
            4,
            4,
            vec![
                -4.0, 2.0, -2.0, -3.0, 9.0, 6.0, 2.0, 6.0, 0.0, -5.0, 1.0, -5.0, 0.0, 0.0, 0.0, 0.0,
            ],
        );

        assert!(invertible.invertible());
        assert!(!non_invertible.invertible());
    }

    #[test]
    fn test_inversion() {
        let a = Matrix::from_values(
            4,
            4,
            vec![
                -5.0, 2.0, 6.0, -8.0, 1.0, -5.0, 1.0, 8.0, 7.0, 7.0, -6.0, -7.0, 1.0, -3.0, 7.0,
                4.0,
            ],
        );
        let b = a.inverse().unwrap();

        let result = Matrix::from_values(
            4,
            4,
            vec![
                0.21805, 0.45113, 0.2406, -0.04511, -0.80827, -1.45677, -0.44361, 0.52068,
                -0.07895, -0.22368, -0.05263, 0.19737, -0.52256, -0.81391, -0.30075, 0.30639,
            ],
        );

        assert_eq!(a.determinant(), 532.);
        assert_eq!(a.cofactor(2, 3), -160.);

        assert_eq!(b.get(3, 2), -160. / 532.);
        assert_eq!(a.cofactor(3, 2), 105.);
        assert_eq!(b.get(2, 3), 105. / 532.);

        assert_eq!(b, result);
    }

    #[test]
    fn test_inverse_multiplication() {
        let a = Matrix::from_values(
            4,
            4,
            vec![
                3.0, -9.0, 7.0, 3.0, 3.0, -8.0, 2.0, -9.0, -4.0, 4.0, 4.0, 1.0, -6.0, 5.0, -1.0,
                1.0,
            ],
        );
        let b = Matrix::from_values(
            4,
            4,
            vec![
                8.0, 2.0, 2.0, 2.0, 3.0, -1.0, 7.0, 0.0, 7.0, 0.0, 5.0, 4.0, 6.0, -2.0, 0.0, 5.0,
            ],
        );

        let c = &a * &b;
        assert_eq!(c * b.inverse().unwrap(), a);
    }

    #[test]
    fn test_translation() {
        let transform = Matrix::translation(5., -3., 2.);
        let p = Tuple::point(-3., 4., 5.);

        assert_eq!(&transform * p, Tuple::point(2., 1., 7.));

        let inv = transform.inverse().unwrap();
        assert_eq!(inv * p, Tuple::point(-8., 7., 3.));

        let v = Tuple::vector(-3., 4., 5.);
        assert_eq!(transform * v, v);
    }

    #[test]
    fn test_translation_chained() {
        let p = Tuple::point(-3., 4., 5.);
        assert_eq!(p.translate(5., -3., 2.), Tuple::point(2., 1., 7.));

        let v = Tuple::vector(-3., 4., 5.);
        assert_eq!(v.translate(5., -3., 2.), v);
    }

    #[test]
    fn test_scaling() {
        let transform = Matrix::scaling(2., 3., 4.);
        let p = Tuple::point(-4., 6., 8.);
        assert_eq!(&transform * p, Tuple::point(-8., 18., 32.));

        let v = Tuple::vector(-4., 6., 8.);
        assert_eq!(&transform * v, Tuple::vector(-8., 18., 32.));

        let inv = transform.inverse().unwrap();
        assert_eq!(inv * v, Tuple::vector(-2., 2., 2.));
    }

    #[test]
    fn test_scaling_chained() {
        let p = Tuple::point(-4., 6., 8.);
        assert_eq!(p.scale(2., 3., 4.), Tuple::point(-8., 18., 32.));

        let v = Tuple::vector(-4., 6., 8.);
        assert_eq!(v.scale(2., 3., 4.), Tuple::vector(-8., 18., 32.));
    }

    #[test]
    fn test_reflection() {
        let transform = Matrix::scaling(-1., 1., 1.);
        let p = Tuple::point(2., 3., 4.);

        assert_eq!(transform * p, Tuple::point(-2., 3., 4.));
    }

    #[test]
    fn test_rotation_x() {
        let p = Tuple::point(0., 1., 0.);
        let half_quarter = Matrix::rotation_x(PI / 4.);
        let full_quarter = Matrix::rotation_x(PI / 2.);
        assert_eq!(
            half_quarter * p,
            Tuple::point(0.0, (2.0_f64).sqrt() / 2.0, (2.0_f64).sqrt() / 2.0)
        );
        assert_eq!(full_quarter * p, Tuple::point(0.0, 0.0, 1.0));

        assert_eq!(
            p.rotate_x(PI / 4.),
            Tuple::point(0.0, (2.0_f64).sqrt() / 2.0, (2.0_f64).sqrt() / 2.0)
        );
        assert_eq!(p.rotate_x(PI / 2.), Tuple::point(0.0, 0.0, 1.0));

    }

    #[test]
    fn test_rotation_y() {
        let p = Tuple::point(0., 0., 1.);
        let half_quarter = Matrix::rotation_y(PI / 4.);
        let full_quarter = Matrix::rotation_y(PI / 2.);
        assert_eq!(
            half_quarter * p,
            Tuple::point((2.0_f64).sqrt() / 2.0, 0.0, (2.0_f64).sqrt() / 2.0)
        );
        assert_eq!(full_quarter * p, Tuple::point(1.0, 0.0, 0.0));

        assert_eq!(
            p.rotate_y(PI / 4.),
            Tuple::point((2.0_f64).sqrt() / 2.0, 0.0, (2.0_f64).sqrt() / 2.0)
        );
        assert_eq!(p.rotate_y(PI / 2.), Tuple::point(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_rotation_z() {
        let p = Tuple::point(0., 1., 0.);
        let half_quarter = Matrix::rotation_z(PI / 4.);
        let full_quarter = Matrix::rotation_z(PI / 2.);
        assert_eq!(
            half_quarter * p,
            Tuple::point(-(2.0_f64).sqrt() / 2.0, (2.0_f64).sqrt() / 2.0, 0.0)
        );
        assert_eq!(full_quarter * p, Tuple::point(-1.0, 0.0, 0.0));

        assert_eq!(
            p.rotate_z(PI / 4.),
            Tuple::point(-(2.0_f64).sqrt() / 2.0, (2.0_f64).sqrt() / 2.0, 0.0)
        );
        assert_eq!(p.rotate_z(PI / 2.), Tuple::point(-1.0, 0.0, 0.0));
    }

    #[test]
    fn test_shearing_moves_x_to_y() {
        let transform = Matrix::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Tuple::point(5.0, 3.0, 4.0));

        assert_eq!(p.shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0), Tuple::point(5.0, 3.0, 4.0));

    }

    #[test]
    fn test_shearing_moves_x_to_z() {
        let transform = Matrix::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Tuple::point(6.0, 3.0, 4.0));
    }

    #[test]
    fn test_shearing_moves_y_to_x() {
        let transform = Matrix::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Tuple::point(2.0, 5.0, 4.0));

        assert_eq!(p.shear(0.0, 0.0, 1.0, 0.0, 0.0, 0.0), Tuple::point(2.0, 5.0, 4.0));

    }

    #[test]
    fn test_shearing_moves_y_to_z() {
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Tuple::point(2.0, 7.0, 4.0));
        assert_eq!(p.shear(0.0, 0.0, 0.0, 1.0, 0.0, 0.0), Tuple::point(2.0, 7.0, 4.0));
    }

    #[test]
    fn test_shearing_moves_z_to_x() {
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Tuple::point(2.0, 3.0, 6.0));
        assert_eq!(p.shear(0.0, 0.0, 0.0, 0.0, 1.0, 0.0), Tuple::point(2.0, 3.0, 6.0));
    }

    #[test]
    fn test_shearing_moves_z_to_y() {
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Tuple::point(2.0, 3.0, 7.0));
        assert_eq!(p.shear(0.0, 0.0, 0.0, 0.0, 0.0, 1.0), Tuple::point(2.0, 3.0, 7.0));
    }

    #[test]
    fn test_transformations_applied_in_sequence() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let a = Matrix::rotation_x(PI / 2.0);
        let b = Matrix::scaling(5.0, 5.0, 5.0);
        let c = Matrix::translation(10.0, 5.0, 7.0);

        let p2 = &a * p;
        assert_eq!(p2, Tuple::point(1.0, -1.0, 0.0));

        let p3 = &b * p2;
        assert_eq!(p3, Tuple::point(5.0, -5.0, 0.0));

        let p4 = &c * p3;
        assert_eq!(p4, Tuple::point(15.0, 0.0, 7.0));

        let p5 = c * b * a * p;
        assert_eq!(p5, Tuple::point(15.0, 0.0, 7.0));

        assert_eq!(p.rotate_x(PI / 2.0).scale(5.0, 5.0, 5.0).translate(10.0, 5.0, 7.0), Tuple::point(15.0, 0.0, 7.0))
    }
}
