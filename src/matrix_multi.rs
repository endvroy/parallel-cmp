extern crate rayon;
extern crate num_traits;

use std::ops::{Index, IndexMut};
use num_traits::{Num, NumAssign};

struct Matrix {
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

    fn matmul(a: &Matrix, b: &Matrix) -> Matrix {
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

    struct MatrixRow<'a> {
        v: &'a [f64],
        size: usize,
    }

    struct MatrixRowIterator<'a> {
        matrix: &'a Matrix,
        row_idx: usize,
    }

    impl<'a> Iterator for MatrixRowIterator<'a> {
        type Item = Matrix;

        fn next(&mut self) -> Option<Self::Item> {
            self.matrix.buf.chunks(self.matrix.n_cols);

            if self.row_idx > self.matrix.n_rows {
                Option::None
            } else {
                let mut row = Matrix::new(1,
                                          self.matrix.n_cols);
                // todo: ref self.row_idx

                self.row_idx += 1;
                Option::Some(row)
            }
        }
    }

    fn matmul<T: NumAssign + Copy>(a: &Matrix, b: &Matrix) -> Matrix {
        unimplemented!()
    }
}
