#[cfg(test)]
mod test_quicksort {
    extern crate rand;
    extern crate parallel_cmp;

    use parallel_cmp::{quicksort};

    use self::rand::Rng;

    #[test]
    fn test_quicksort() {
        let capacity = 256;
        let mut vec = Vec::with_capacity(capacity);
        let mut rng = rand::thread_rng();
        for _i in 0..capacity {
            vec.push(rng.gen_range(0..capacity / 2));
        }
        let mut ref_vec = vec.clone();
        let mut par_vec = vec.clone();
        ref_vec.sort();
        quicksort::single_threaded(&mut vec);
        quicksort::parallel(&mut par_vec);
        assert_eq!(ref_vec, vec);
        assert_eq!(ref_vec, par_vec);
    }
}