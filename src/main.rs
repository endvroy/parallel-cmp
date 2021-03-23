extern crate rayon;

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

fn quicksort<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    let p_idx = arr.len() / 2;  // todo: use random pivot
    let (x, y) = three_way_partition(arr, p_idx);
    quicksort(&mut arr[0..x]);
    quicksort(&mut arr[y..len]);
}

fn quicksort_par<T: PartialOrd + Send>(arr: &mut [T]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    let p_idx = arr.len() / 2;  // todo: use random pivot
    let (x, y) = three_way_partition(arr, p_idx);
    let (arr_rem, right) = arr.split_at_mut(y);
    let (left, _) = arr_rem.split_at_mut(x);
    rayon::join(|| quicksort_par(left),
                || quicksort_par(right));
}

fn main() {
    let mut vec = vec![3, 4, 4, 5, 2, 2, 5, 7, 9, 5, 6, 1];
    quicksort_par(&mut vec);
    // let (x, y) = three_way_partition(&mut vec, 3);
    dbg!(vec);
    // dbg!((x, y));
}
