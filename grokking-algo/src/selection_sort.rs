pub fn selection_sort(mut arr: Vec<i32>) -> Vec<i32> {
    for i in (1..(arr.len())).rev() {
        let mut el = i;
        for j in 0..i {
            if arr[j] > arr[el] {
                el = j;
            }
        }
        arr.swap(i, el);
    }
    arr
}

#[test]
fn tests() {
    assert_eq!(
        selection_sort(vec![7, 2, 1, 5, 6, 3, 4, 8]),
        vec![1, 2, 3, 4, 5, 6, 7, 8]
    );
    assert_eq!(selection_sort(vec![]), vec![]);
    assert_eq!(selection_sort(vec![1]), vec![1]);
}
