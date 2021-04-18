#[cfg(test)]
mod test_histogram {
    extern crate rand;
    extern crate parallel_cmp;

    use parallel_cmp::{histogram};

    use self::rand::Rng;


    fn gen_histogram_data(len: usize) -> Vec<f64> {
        let mut vec = Vec::with_capacity(len);
        let mut rng = rand::thread_rng();
        for _ in 0..len {
            vec.push(rng.gen_range(0.0..20.0));
        }
        vec
    }

    #[test]
    fn test_histogram() {
        let data = gen_histogram_data(100);

        let histo = histogram::single_threaded::calc_histogram(&data, 10);
        let par_histo = histogram::parallel::calc_histogram(&data, 10, 4);
        assert_eq!(histo, par_histo);
    }
}