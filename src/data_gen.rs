extern crate rand;

use std::path::Path;


pub mod histogram_data {
    use rand::Rng;
    use super::Path;


    pub fn gen(len: usize) -> Vec<f64> {
        let mut vec = Vec::with_capacity(len);
        let mut rng = rand::thread_rng();
        for _ in 0..len {
            vec.push(rng.gen_range(0.0..20.0));
        }
        vec
    }

    pub fn write(data: Vec<f64>, path: &Path) {
        unimplemented!()
    }

    pub fn read() -> Vec<f64> {
        unimplemented!()
    }
}

pub mod rand_matrix {
    use rand::Rng;
    use super::super::matrix_multi::Matrix;
    use super::Path;

    pub fn gen(n_rows: usize, n_cols: usize) -> Matrix {
        let len = n_rows * n_cols;
        let mut vec = Vec::with_capacity(len);
        let mut rng = rand::thread_rng();
        for _ in 0..len {
            vec.push(rng.gen());
        }
        Matrix {
            n_rows,
            n_cols,
            buf: vec,
        }
    }

    pub fn write(matrix: Matrix, path: &Path) {
        unimplemented!()
    }

    pub fn read() -> Matrix {
        unimplemented!()
    }
}


pub mod quicksort_vec {
    use rand::Rng;
    use super::Path;

    pub fn gen(len: usize, range: usize) -> Vec<usize> {
        let mut rng = rand::thread_rng();
        let mut vec = Vec::with_capacity(len);
        for _i in 0..len {
            vec.push(rng.gen_range(0..range));
        }
        vec
    }

    pub fn write(data: Vec<f64>, path: &Path) {
        unimplemented!()
    }

    pub fn read() -> Vec<f64> {
        unimplemented!()
    }
}

