extern crate parallel_cmp;

use parallel_cmp::*;
use std::env;
use std::str::FromStr;
use std::path::Path;

/// usage:
/// quicksort filename serial/parallel n_threads
fn main() {
    let args: Vec<_> = env::args().collect();
    let path = Path::new(args[1].as_str());
    let mut data = data_gen::quicksort_vec::read(path);
    match args[2].as_str() {
        "serial" => {
            quicksort::single_threaded::sort(&mut data);
            println!("{:?}", data);
        }
        "parallel" => {
            let n_threads = usize::from_str(args[3].as_str()).unwrap();
            rayon::ThreadPoolBuilder::new().num_threads(n_threads).build_global().unwrap();
            quicksort::parallel::sort(&mut data);
            println!("{:?}", data);
        }
        _ => {
            panic!("arg must be serial or parallel");
        }
    }
}