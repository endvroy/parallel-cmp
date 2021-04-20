extern crate parallel_cmp;

use parallel_cmp::*;
use std::env;
use std::str::FromStr;
use std::path::Path;

/// usage:
/// data_gen quicksort filename n_data range
/// data_gen histogram filename n_data
/// data_gen matrix filename_a filename_b n_rows_a n_cols_a n_cols_b
fn main() {
    let args: Vec<_> = env::args().collect();

    match args[1].as_str() {
        "quicksort" => {
            let fname = &args[2];
            let n_data = usize::from_str(args[3].as_str()).unwrap();
            let path = Path::new(fname);
            let range = usize::from_str(args[4].as_str()).unwrap();
            let data = data_gen::quicksort_vec::gen(n_data, range);
            data_gen::quicksort_vec::write(&data, path);
        }
        "histogram" => {
            let fname = &args[2];
            let n_data = usize::from_str(args[3].as_str()).unwrap();
            let path = Path::new(fname);
            let data = data_gen::histogram_data::gen(n_data);
            data_gen::histogram_data::write(&data, path);
        }
        "matrix" => {
            let fname_a = &args[2];
            let fname_b = &args[3];
            let n_rows_a = usize::from_str(args[4].as_str()).unwrap();
            let n_cols_a = usize::from_str(args[5].as_str()).unwrap();
            let n_cols_b = usize::from_str(args[6].as_str()).unwrap();
            let matrix_a = data_gen::rand_matrix::gen(n_rows_a,n_cols_a);
            let matrix_b = data_gen::rand_matrix::gen(n_cols_a,n_cols_b);
            data_gen::rand_matrix::write(&matrix_a, Path::new(fname_a));
            data_gen::rand_matrix::write(&matrix_b, Path::new(fname_b));
        }
        _ => {
            panic!("unknown dataset argument");
        }
    }
}

