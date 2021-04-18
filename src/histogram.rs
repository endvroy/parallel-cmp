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


    fn calc_histogram(data: &[usize], n_bins: usize, n_threads: usize) -> Vec<usize> {
        let mut histogram = vec![0; n_bins];
        let chunk_size = data.len() / n_threads;
        let local_histograms:Vec<_> = data.chunks(chunk_size).par_bridge()
            .map(|x| super::single_threaded::calc_histogram(x, n_bins))
            .collect();
        for local_histogram in local_histograms {
            for i in 0..histogram.len() {
                histogram[i] += local_histogram[i];
            }
        }
        histogram
    }
}