use rand::{seq::SliceRandom, thread_rng};

fn bogosort<T: Ord>(mut v: Vec<T>) -> Vec<T> {
    while !is_sorted(&mut v) {
        v.shuffle(&mut thread_rng())
    }
    v
}

fn is_sorted<T: Ord>(v: &mut Vec<T>) -> bool {
    v.windows(2).all(|i| i[0] <= i[1])
}

fn main() {
    let n = vec![5, 6, 4, 2, 8, 1, 3, 10, 9];
    assert!(bogosort(n)[0] == 1);
}
