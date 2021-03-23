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

pub fn single_threaded<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    let p_idx = arr.len() / 2;  // todo: use random pivot
    let (x, y) = three_way_partition(arr, p_idx);
    single_threaded(&mut arr[0..x]);
    single_threaded(&mut arr[y..len]);
}

pub fn parallel<T: PartialOrd + Send>(arr: &mut [T]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    let p_idx = arr.len() / 2;  // todo: use random pivot
    let (x, y) = three_way_partition(arr, p_idx);
    let (arr_rem, right) = arr.split_at_mut(y);
    let (left, _) = arr_rem.split_at_mut(x);
    rayon::join(|| parallel(left),
                || parallel(right));
}