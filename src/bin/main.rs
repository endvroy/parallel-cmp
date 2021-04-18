extern crate parallel_cmp;
extern crate rayon;

use parallel_cmp::{quicksort};


fn main() {
    rayon::ThreadPoolBuilder::new().num_threads(4).build_global().unwrap();
    let mut vec = vec![3, 4, 4, 5, 2, 2, 5, 7, 9, 5, 6, 1];
    quicksort::parallel::sort(&mut vec);
    // let (x, y) = three_way_partition(&mut vec, 3);
    dbg!(vec);
    // dbg!((x, y));
}
