extern crate parallel_cmp;
extern crate rayon;
extern crate rand;

use parallel_cmp::*;
use std::path::Path;

fn main() {
    let histogram_data = data_gen::histogram_data::gen(50);
    data_gen::histogram_data::write(histogram_data, Path::new("./histogram_data.txt"));
}
