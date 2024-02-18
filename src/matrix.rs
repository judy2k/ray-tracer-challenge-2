use std::fmt::Debug;

pub struct Matrix {
    rows: usize,
    cols: usize,
    values: Vec<f64>,
}

impl Debug for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Matrix<{}, {}>", self.rows, self.cols)?;

        Ok(())
    }
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
    }

    pub fn get(&self, row: usize, col: usize) -> Option<f64> {
        self.values.get(self.index(row, col)).copied()
    }

    fn index(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_color_construction() {
        let m = Matrix::from_values(
            4,
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5,
                16.5,
            ],
        );

        assert_eq!(m.get(0, 0).unwrap(), 1.);
        assert_eq!(m.get(0, 3).unwrap(), 4.);
        assert_eq!(m.get(1, 0).unwrap(), 5.5);
        assert_eq!(m.get(1, 2).unwrap(), 7.5);
        assert_eq!(m.get(2, 2).unwrap(), 11.);
        assert_eq!(m.get(3, 0).unwrap(), 13.5);
        assert_eq!(m.get(3, 2).unwrap(), 15.5);
    }
}
