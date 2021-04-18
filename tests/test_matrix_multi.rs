#[cfg(test)]
mod test_histogram {
    extern crate rand;
    extern crate parallel_cmp;

    use parallel_cmp::*;
    use matrix_multi::Matrix;
    use self::rand::Rng;


    #[test]
    fn test_histogram() {
        let a = data_gen::rand_matrix::gen(20, 12);
        let b = data_gen::rand_matrix::gen(12, 8);
        let c = matrix_multi::single_threaded::matmul(&a, &b);
        let d = matrix_multi::parallel::matmul(&a, &b);
        assert_eq!(c.buf, d.buf);
    }
}