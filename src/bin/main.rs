extern crate parallel_cmp;
extern crate rayon;
extern crate rand;

use parallel_cmp::*;
use rand::Rng;

fn gen_histogram_data(len: usize) -> Vec<f64> {
    let mut vec = Vec::with_capacity(len);
    let mut rng = rand::thread_rng();
    for _ in 0..len {
        vec.push(rng.gen_range(0.0..20.0));
    }
    vec
}

fn main() {
    rayon::ThreadPoolBuilder::new().num_threads(4).build_global().unwrap();
    let data = gen_histogram_data(100);

    let histo = histogram::single_threaded::calc_histogram(&data, 10);
    let par_histo = histogram::parallel::calc_histogram(&data, 10, 4);
    dbg!(data);
    dbg!(histo);
    dbg!(par_histo);
    // dbg!((x, y));
}
