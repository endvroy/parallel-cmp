extern crate rayon;
extern crate num_traits;

use std::ops::{Index, IndexMut};
// use num_traits::{Num, NumAssign};

pub struct Matrix {
    n_rows: usize,
    n_cols: usize,
    buf: Vec<f64>,
}

impl Matrix {
    fn new(n_rows: usize, n_cols: usize) -> Self {
        Matrix {
            n_rows,
            n_cols,
            buf: Vec::with_capacity(n_cols * n_rows),
        }
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x, y) = index;
        &self.buf[x * self.n_cols + y]
    }
}


impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (x, y) = index;
        &mut self.buf[x * self.n_cols + y]
    }
}

pub mod single_threaded {
    use super::*;

    pub fn matmul(a: &Matrix, b: &Matrix) -> Matrix {
        if a.n_cols != b.n_rows {
            panic!("matrix row and columns do not match");
        }
        let mut c = Matrix::new(a.n_rows, b.n_cols);
        for row in 0..a.n_rows {
            for col in 0..b.n_cols {
                let mut sum = 0.0;
                for i in 0..a.n_cols {
                    sum += a[(row, i)] * b[(i, col)];
                }
                c[(row, col)] = sum;
            }
        }
        c
    }
}

pub mod parallel {
    use super::*;
    use rayon::prelude::*;

    struct RowIterator<'a> {
        matrix: &'a Matrix,
        row: usize,
        i: usize,
    }

    impl<'a> RowIterator<'a> {
        fn new(matrix: &'a Matrix, row: usize) -> Self {
            RowIterator {
                matrix,
                row,
                i: 0,
            }
        }
    }

    impl Iterator for RowIterator<'_> {
        type Item = f64;

        fn next(&mut self) -> Option<Self::Item> {
            if self.i < self.matrix.n_cols {
                let result = Option::Some(self.matrix[(self.row, self.i)]);
                self.i += 1;
                result
            } else {
                Option::None
            }
        }
    }

    struct ColIterator<'a> {
        matrix: &'a Matrix,
        col: usize,
        i: usize,
    }

    impl<'a> ColIterator<'a> {
        fn new(matrix: &'a Matrix, col: usize) -> Self {
            ColIterator {
                matrix,
                col,
                i: 0,
            }
        }
    }

    impl Iterator for ColIterator<'_> {
        type Item = f64;

        fn next(&mut self) -> Option<Self::Item> {
            if self.i < self.matrix.n_rows {
                let result = Option::Some(self.matrix[(self.i, self.col)]);
                self.i += 1;
                result
            } else {
                Option::None
            }
        }
    }

    impl Matrix {
        fn iter_row(&self, row: usize) -> RowIterator {
            if row >= self.n_rows {
                panic!("row index out of range");
            } else {
                RowIterator::new(self, row)
            }
        }
        fn iter_col(&self, col: usize) -> ColIterator {
            if col >= self.n_cols {
                panic!("column index out of range");
            } else {
                ColIterator::new(self, col)
            }
        }
    }

    fn matmul_one(a: &Matrix, b: &Matrix, row: usize, col: usize) -> f64 {
        a.iter_row(row).zip(b.iter_col(col))
            .par_bridge()
            .map(|(x, y)| x * y)
            .sum()
    }

    pub fn matmul(a: &Matrix, b: &Matrix) -> Matrix {
        let buf = (0..a.n_rows)
            .zip(0..b.n_cols)
            .par_bridge()
            .map(|(x, y)| matmul_one(a, b, x, y))
            .collect();
        Matrix {
            n_rows: a.n_rows,
            n_cols: b.n_cols,
            buf,
        }
    }
}
