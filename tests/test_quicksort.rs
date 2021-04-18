#[cfg(test)]
mod test_quicksort {
    extern crate rand;
    extern crate parallel_cmp;

    use parallel_cmp::{quicksort, data_gen};


    #[test]
    fn test_quicksort() {
        let capacity = 256;
        let mut vec = data_gen::quicksort_vec::gen(capacity, capacity / 2);
        let mut ref_vec = vec.clone();
        let mut par_vec = vec.clone();
        ref_vec.sort();
        quicksort::single_threaded::sort(&mut vec);
        quicksort::parallel::sort(&mut par_vec);
        assert_eq!(ref_vec, vec);
        assert_eq!(ref_vec, par_vec);
    }
}