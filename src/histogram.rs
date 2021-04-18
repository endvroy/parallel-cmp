extern crate rayon;

use rayon::prelude::*;

pub mod single_threaded {
    fn calc_histogram(data: Vec<usize>, n_bins: usize) -> Vec<usize> {
        let mut histogram = vec![0; n_bins];
        for x in data {
            let idx = x % n_bins;
            histogram[idx] += 1;
        }
        histogram
    }
}

pub mod parallel {
    use super::*;

    fn calc_histogram(data: Vec<usize>, n_bins: usize) -> Vec<usize> {
        let mut histogram = vec![0; n_bins];
        // data.par_iter()
        //     .map(|x| x % n_bins)
        //     .for_each(|x| histogram[x] += 1);
        histogram
    }
}