#[cfg(test)]
mod test_histogram {
    extern crate rand;
    extern crate parallel_cmp;

    use parallel_cmp::*;
    use matrix_multi::Matrix;
    use self::rand::Rng;


    fn gen_rand_matrix(n_rows: usize, n_cols: usize) -> Matrix {
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

    #[test]
    fn test_histogram() {
        let a = gen_rand_matrix(20, 12);
        let b = gen_rand_matrix(12, 8);
        let c = matrix_multi::single_threaded::matmul(&a, &b);
        let d = matrix_multi::parallel::matmul(&a, &b);
        assert_eq!(c.buf, d.buf);
    }
}