extern crate rayon;

use std::time::Instant;

#[allow(dead_code)]
fn time_thread_spawn() {
    let n_loops = 10000;
    let now = Instant::now();
    for _ in 0..n_loops {
        rayon::spawn(|| {});
    }
    println!("{:?}", now.elapsed() / n_loops);
}

fn main() {
    let n_threads = 1000;
    let pool_builder = rayon::ThreadPoolBuilder::new().num_threads(n_threads);
    let now = Instant::now();
    let _result = pool_builder.build();
    println!("{:?}", now.elapsed() / n_threads as u32);
}