extern crate parallel_cmp;

use parallel_cmp::*;
use std::env;
use std::str::FromStr;
use std::path::Path;

/// usage:
/// matmul filename_a filename_b serial/parallel n_threads
fn main() {
    let args: Vec<_> = env::args().collect();
    let mat_a = data_gen::rand_matrix::read(Path::new(args[1].as_str()));
    let mat_b = data_gen::rand_matrix::read(Path::new(args[2].as_str()));
    match args[3].as_str() {
        "serial" => {
            matrix_multi::single_threaded::matmul(&mat_a, &mat_b);
        }
        "parallel" => {
            let n_threads = usize::from_str(args[4].as_str()).unwrap();
            rayon::ThreadPoolBuilder::new().num_threads(n_threads).build_global().unwrap();
            matrix_multi::parallel::matmul(&mat_a, &mat_b);
        }
        _ => {
            panic!("arg must be serial or parallel");
        }
    }
}