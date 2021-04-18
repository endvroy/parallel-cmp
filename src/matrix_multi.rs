extern crate rayon;
extern crate num_traits;

use std::ops::{Index, IndexMut};
use num_traits::{Num, NumAssign};

struct Matrix<T: Num> {
    n_rows: usize,
    n_cols: usize,
    buf: Vec<T>,
}

impl<T: Num> Matrix<T> {
    fn new(n_rows: usize, n_cols: usize) -> Self {
        Matrix {
            n_rows,
            n_cols,
            buf: Vec::with_capacity(n_cols * n_rows),
        }
    }
}

impl<T: Num> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x, y) = index;
        &self.buf[x * self.n_cols + y]
    }
}


impl<T: Num> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (x, y) = index;
        &mut self.buf[x * self.n_cols + y]
    }
}

pub mod single_threaded {
    use super::*;

    fn matmul<T: NumAssign + Copy>(a: &Matrix<T>, b: &Matrix<T>) -> Matrix<T> {
        if a.n_cols != b.n_rows {
            panic!("matrix row and columns do not match");
        }
        let mut c = Matrix::new(a.n_rows, b.n_cols);
        for row in 0..a.n_rows {
            for col in 0..b.n_cols {
                let mut sum = T::zero();
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

    struct MatrixRowIterator<'a, T: Num> {
        matrix: &'a Matrix<T>,
        row_idx: usize,
    }

    impl<'a, T: Num> Iterator for MatrixRowIterator<'a, T> {
        type Item = Matrix<T>;

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

    fn matmul<T: NumAssign + Copy>(a: &Matrix<T>, b: &Matrix<T>) -> Matrix<T> {
        unimplemented!()
    }
}
