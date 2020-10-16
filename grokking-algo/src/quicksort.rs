pub fn quicksort(mut arr: Vec<i32>) -> Vec<i32> {
    if arr.len() < 2 {
        return arr;
    }
    let mut a = vec![];
    let mut b = vec![];
    let pivot = arr.remove(arr.len() / 2);
    for i in arr {
        if i <= pivot {
            a.push(i);
        } else {
            b.push(i);
        }
    }

    let mut a = quicksort(a);
    a.push(pivot);
    a.append(&mut quicksort(b));
    a
}

#[test]
fn tests() {
    assert_eq!(
        quicksort(vec![7, 2, 1, 5, 6, 3, 4, 8]),
        vec![1, 2, 3, 4, 5, 6, 7, 8]
    );
    assert_eq!(quicksort(vec![]), vec![]);
    assert_eq!(quicksort(vec![1]), vec![1]);
}
