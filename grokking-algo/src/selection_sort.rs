pub fn selection_sort(arr: &mut Vec<i32>) -> &mut Vec<i32> {
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
