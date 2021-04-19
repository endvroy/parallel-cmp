extern crate rand;

use std::path::Path;

pub mod histogram_data {
    use rand::Rng;
    use super::Path;
    use std::fs::File;
    use std::io::{BufReader, BufWriter, Write, BufRead};
    use std::str::FromStr;


    pub fn gen(len: usize) -> Vec<f64> {
        let mut vec = Vec::with_capacity(len);
        let mut rng = rand::thread_rng();
        for _ in 0..len {
            vec.push(rng.gen_range(0.0..20.0));
        }
        vec
    }

    pub fn write(data: &Vec<f64>, path: &Path) {
        let file = File::create(path).unwrap();
        let mut writer = BufWriter::new(file);
        write!(writer, "{} ", data.len()).unwrap();
        for x in data {
            write!(writer, "{} ", x).unwrap();
        }
    }

    pub fn read(path: &Path) -> Vec<f64> {
        let file = File::open(path).unwrap();
        let mut reader = BufReader::new(file);
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        let mut split = line.split(" ");
        let n_data = usize::from_str(split.next().unwrap()).unwrap();
        let mut vec = Vec::with_capacity(n_data);
        for _ in 0..n_data {
            vec.push(f64::from_str(split.next().unwrap()).unwrap());
        }
        vec
    }
}

pub mod rand_matrix {
    use rand::Rng;
    use super::super::matrix_multi::Matrix;
    use super::Path;
    use std::fs::File;
    use std::io::{BufReader, BufWriter, Write, BufRead};
    use std::str::FromStr;

    pub fn gen(n_rows: usize, n_cols: usize) -> Matrix {
        let len = n_rows * n_cols;
        let mut vec = Vec::with_capacity(len);
        let mut rng = rand::thread_rng();
        for _ in 0..len {
            vec.push(rng.gen());
        }
        Matrix {
            n_rows,
            n_cols,
            buf: vec,
        }
    }

    pub fn write(matrix: &Matrix, path: &Path) {
        let file = File::create(path).unwrap();
        let mut writer = BufWriter::new(file);
        write!(writer, "{} {} ", matrix.n_rows, matrix.n_cols).unwrap();
        for x in &matrix.buf {
            write!(writer, "{} ", x).unwrap();
        }
    }

    pub fn read(path: &Path) -> Matrix {
        let file = File::open(path).unwrap();
        let mut reader = BufReader::new(file);
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        let mut split = line.split(" ");
        let n_rows = usize::from_str(split.next().unwrap()).unwrap();
        let n_cols = usize::from_str(split.next().unwrap()).unwrap();
        let n_data = n_rows * n_cols;
        let mut vec = Vec::with_capacity(n_data);
        for _ in 0..n_data {
            vec.push(f64::from_str(split.next().unwrap()).unwrap());
        }
        Matrix {
            n_rows,
            n_cols,
            buf: vec,
        }
    }
}


pub mod quicksort_vec {
    use rand::Rng;
    use super::Path;
    use std::fs::File;
    use std::io::{BufReader, BufWriter, Write, BufRead};
    use std::str::FromStr;

    pub fn gen(len: usize, range: usize) -> Vec<usize> {
        let mut rng = rand::thread_rng();
        let mut vec = Vec::with_capacity(len);
        for _i in 0..len {
            vec.push(rng.gen_range(0..range));
        }
        vec
    }

    pub fn write(data: &Vec<usize>, path: &Path) {
        let file = File::create(path).unwrap();
        let mut writer = BufWriter::new(file);
        write!(writer, "{} ", data.len()).unwrap();
        for x in data {
            write!(writer, "{} ", x).unwrap();
        }
    }

    pub fn read(path: &Path) -> Vec<usize> {
        let file = File::open(path).unwrap();
        let mut reader = BufReader::new(file);
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        let mut split = line.split(" ");
        let n_data = usize::from_str(split.next().unwrap()).unwrap();
        let mut vec = Vec::with_capacity(n_data);
        for _ in 0..n_data {
            vec.push(usize::from_str(split.next().unwrap()).unwrap());
        }
        vec
    }
}

