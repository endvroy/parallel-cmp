extern crate rayon;
extern crate rand;

use self::rand::Rng;

fn three_way_partition<T: PartialOrd>(arr: &mut [T], p_idx: usize) -> (usize, usize) {
    let mut x = 0;
    let mut y = 1;
    let mut z = 1;
    arr.swap(0, p_idx);
    while z < arr.len() {
        if arr[z] < arr[x] {
            arr.swap(x, z);
            arr.swap(y, z);
            x += 1;
            y += 1;
            z += 1;
        } else if arr[z] == arr[x] {
            arr.swap(y, z);
            y += 1;
            z += 1;
        } else {
            z += 1;
        }
    }
    (x, y)
}

pub mod single_threaded {
    use super::Rng;

    pub fn sort<T: PartialOrd>(arr: &mut [T]) {
        let len = arr.len();
        if len <= 1 {
            return;
        }
        let mut rng = rand::thread_rng();
        let p_idx = rng.gen_range(0..len);
        let (x, y) = super::three_way_partition(arr, p_idx);
        sort(&mut arr[0..x]);
        sort(&mut arr[y..len]);
    }
}

pub mod parallel {
    use super::Rng;

    pub fn sort<T: PartialOrd + Send>(arr: &mut [T]) {
        let len = arr.len();
        if len <= 1 {
            return;
        }
        let mut rng = rand::thread_rng();
        let p_idx = rng.gen_range(0..len);
        let (x, y) = super::three_way_partition(arr, p_idx);
        let (arr_rem, right) = arr.split_at_mut(y);
        let (left, _) = arr_rem.split_at_mut(x);
        rayon::join(|| sort(left),
                    || sort(right));
    }
}

