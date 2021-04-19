#[cfg(test)]
mod test_data_gen {
    extern crate parallel_cmp;

    use parallel_cmp::data_gen;
    use std::path::Path;

    #[test]
    fn test_data_gen_histo() {
        let data = data_gen::histogram_data::gen(50);
        let path = Path::new("test_histogram_data.txt");
        data_gen::histogram_data::write(&data, path);
        let readback = data_gen::histogram_data::read(path);
        assert_eq!(data, readback);
    }

    #[test]
    fn test_data_gen_matrix() {
        let matrix = data_gen::rand_matrix::gen(20, 30);
        let path = Path::new("test_matrix_data.txt");
        data_gen::rand_matrix::write(&matrix, path);
        let readback = data_gen::rand_matrix::read(path);
        assert_eq!(matrix.n_rows, readback.n_rows);
        assert_eq!(matrix.n_cols, readback.n_cols);
        assert_eq!(matrix.buf, readback.buf);
    }

    #[test]
    fn test_data_gen_quicksort() {
        let data = data_gen::quicksort_vec::gen(50, 100);
        let path = Path::new("test_quicksort_data.txt");
        data_gen::quicksort_vec::write(&data, path);
        let readback = data_gen::quicksort_vec::read(path);
        assert_eq!(data, readback);
    }
}