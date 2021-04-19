#[cfg(test)]
mod test_matrix_multi {
    extern crate rand;
    extern crate parallel_cmp;

    use parallel_cmp::*;
    use matrix_multi::Matrix;

    #[test]
    fn test_iter() {
        let mut m = Matrix::new(2, 3);
        for i in 0..6 {
            m.buf[i] = i as f64;
        }
        let row_1: Vec<_> = m.iter_row(1).collect();
        for i in 0..m.n_cols {
            assert_eq!(row_1[i], m[(1, i)]);
        }
        let col_1: Vec<_> = m.iter_col(1).collect();
        for i in 0..m.n_rows {
            assert_eq!(col_1[i], m[(i, 1)]);
        }
    }

    #[test]
    fn test_matrix_multi() {
        let a = data_gen::rand_matrix::gen(20, 12);
        let b = data_gen::rand_matrix::gen(12, 8);
        let c = matrix_multi::single_threaded::matmul(&a, &b);
        let d = matrix_multi::parallel::matmul(&a, &b);
        assert_eq!(c.n_rows, d.n_rows);
        assert_eq!(c.n_cols, d.n_cols);
        for (x, y) in c.buf.iter().zip(d.buf.iter()) {
            assert!((x - y).abs() < 1e-5);
        }
    }
}