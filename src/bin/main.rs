extern crate parallel_cmp;
extern crate rayon;
extern crate rand;

use parallel_cmp::*;
use std::path::Path;
use parallel_cmp::matrix_multi::Matrix;

#[macro_use]
extern crate itertools;

use rayon::prelude::*;

fn main() {
    let data = data_gen::histogram_data::read(Path::new("histogram_data.txt"));
    println!("{}", data.len());
}
