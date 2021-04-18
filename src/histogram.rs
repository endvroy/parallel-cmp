extern crate rayon;


pub mod single_threaded {
    pub fn calc_histogram(data: &[usize], n_bins: usize) -> Vec<usize> {
        let mut histogram = vec![0; n_bins];
        let bin_interval = 20.0 / n_bins as f64;
        for x in data {
            let idx = (*x as f64 / bin_interval) as usize;
            histogram[idx] += 1;
        }
        histogram
    }
}

pub mod parallel {
    use rayon::prelude::*;

    fn combine_histograms(x: Vec<usize>, y: Vec<usize>) -> Vec<usize> {
        let mut result = x;
        for i in 0..y.len() {
            result[i] += y[i];
        }
        result
    }

    fn calc_histogram(data: &[usize], n_bins: usize, n_threads: usize) -> Vec<usize> {
        let chunk_size = data.len() / n_threads;
        let result = data.chunks(chunk_size).par_bridge()
            .map(|x| super::single_threaded::calc_histogram(x, n_bins))
            .reduce_with(|x, y| combine_histograms(x, y))
            .unwrap();
        result
    }
}