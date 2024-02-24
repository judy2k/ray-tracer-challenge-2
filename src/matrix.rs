use crate::space::Tuple;
use std::{fmt::Debug, ops::Mul};
use once_cell::sync::OnceCell;

static IDENTITY_MATRIX: OnceCell<Matrix> = OnceCell::new();

pub fn identity_matrix() -> &'static Matrix {
    IDENTITY_MATRIX.get_or_init(||  Matrix::from_values(4,4, vec![
        1.,0.,0.,0.,
        0.,1.,0.,0.,
        0.,0.,1.,0.,
        0.,0.,0.,1.,
    ]))
}


#[derive(PartialEq, Clone)]
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
        if let Some(val) = self.values.get_mut(index) {
            *val = value;
        }
        // TODO: Raise an error if out of bounds.
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.values.get(self.index(row, col)).copied().unwrap()
    }

    fn index(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }
}

impl Debug for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Matrix<{}, {}>", self.rows, self.cols)?;

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


impl Mul<Tuple> for &Matrix {
    type Output = Tuple;
    fn mul(self, rhs: Tuple) -> Self::Output {
        let mut result_params = [0.0_f64; 4];

        for row in 0..self.rows {
            let mut tally: f64 = 0.0;
            for col in 0..self.cols {
                tally += self.get(row, col) * rhs.get(col)
            }
            result_params[row] = tally;
        }

        Tuple::new(
            result_params[0],
            result_params[1],
            result_params[2],
            result_params[3],
        )
    }
}

impl Mul<Tuple> for Matrix {
    type Output = Tuple;
    fn mul(self, rhs: Tuple) -> Self::Output {
        (&self).mul(rhs)
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::space::*;

    #[test]
    fn test_color_construction_4x4() {
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
    fn test_color_construction_3x3() {
        let m = Matrix::from_values(3, 3, vec![-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0]);

        assert_eq!(m.get(0, 0), -3.0);
        assert_eq!(m.get(1, 1), -2.0);
        assert_eq!(m.get(2, 2), 1.0);
    }

    #[test]
    fn test_color_construction_2x2() {
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

        let t = Tuple::new(1.0, 2.0, 3.0, 1.0);

        let result = Tuple::new(18., 24., 33., 1.);

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
}
