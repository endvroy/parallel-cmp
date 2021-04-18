#[cfg(test)]
mod test_histogram {
    extern crate rand;
    extern crate parallel_cmp;

    use parallel_cmp::{histogram, data_gen};

    use self::rand::Rng;


    #[test]
    fn test_histogram() {
        let data = data_gen::histogram_data::gen(100);

        let histo = histogram::single_threaded::calc_histogram(&data, 10);
        let par_histo = histogram::parallel::calc_histogram(&data, 10, 4);
        assert_eq!(histo, par_histo);
    }
}