use std::cmp::Ordering;

pub fn binary_search(arr: Vec<i32>, item: i32) -> Option<usize> {
    if arr.is_empty() {
        return None;
    }

    let mut mid;
    let mut low = 0;
    let mut high = arr.len();

    while low <= high {
        mid = (low + high) / 2;
        println!("low = {}, mid = {}, high = {}", low, mid, high);
        match arr[mid].cmp(&item) {
            Ordering::Less => low = mid + 1,
            Ordering::Greater => high = mid - 1,
            Ordering::Equal => return Some(mid),
        }
    }

    None
}
