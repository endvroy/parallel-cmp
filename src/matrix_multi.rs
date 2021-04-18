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
    fn new(height: usize, width: usize) -> Self {
        Matrix {
            n_rows: height,
            n_cols: width,
            buf: Vec::with_capacity(width * height),
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

    impl<T: NumAssign + Copy> IntoParallelRefIterator for Matrix<T> {
        type Iter = ();
        type Item = ();

        fn par_iter(&'data self) -> Self::Iter {
            todo!()
        }
    }

    fn matmul<T: NumAssign + Copy>(a: &Matrix<T>, b: &Matrix<T>) -> Matrix<T> {}
}
