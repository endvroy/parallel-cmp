extern crate parallel_cmp;

use parallel_cmp::*;
use std::env;
use std::str::FromStr;
use std::path::Path;

/// usage:
/// histogram filename serial/parallel n_threads
fn main() {
    let args: Vec<_> = env::args().collect();
    let path = Path::new(args[1].as_str());
    let data = data_gen::histogram_data::read(path);
    match args[2].as_str() {
        "serial" => {
            let result = histogram::single_threaded::calc_histogram(&data, 20);
            println!("{:?}", result);
        }
        "parallel" => {
            let n_threads = usize::from_str(args[3].as_str()).unwrap();
            rayon::ThreadPoolBuilder::new().num_threads(n_threads).build_global().unwrap();
            let result = histogram::parallel::calc_histogram(&data, 20, n_threads);
            println!("{:?}", result);
        }
        _ => {
            panic!("arg must be serial or parallel");
        }
    }
}