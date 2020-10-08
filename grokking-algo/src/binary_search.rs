use std::cmp::Ordering;

pub fn binary_search(arr: Vec<i32>, item: i32) -> Option<usize> {
    if arr.is_empty() {
        return None;
    }

    let mut mid;
    let mut low = 0;
    let mut high = arr.len() - 1;

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

#[test]
fn tests() {
    assert_eq!(binary_search(vec![1, 2, 3, 4, 5, 6, 7, 8], 1), Some(0));
    assert_eq!(binary_search(vec![1, 2, 3, 4, 5, 6, 7, 8], 3), Some(2));
    assert_eq!(binary_search(vec![1, 2, 3, 4, 5, 6, 7, 8], 4), Some(3));
    assert_eq!(binary_search(vec![1, 2, 3, 4, 5, 6, 7, 8], 7), Some(6));
    assert_eq!(binary_search(vec![1, 2, 3, 4, 5, 6, 7, 8], 10), None);
}
