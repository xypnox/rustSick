mod binary_search;
mod quicksort;
mod selection_sort;

fn main() {
    let res = binary_search::binary_search(vec![1, 2, 4, 6, 7, 8, 10], 1);

    println!("{:?}", res);

    let array: Vec<i32> = vec![3, 1, 5, 6, 2, 4, 9, 1];

    let array = selection_sort::selection_sort(array);

    println!("{:?}", array);

    let array: Vec<i32> = vec![3, 1, 5, 6, 2, 4, 9, 1];

    let array = quicksort::quicksort(array);

    println!("{:?}", array);
}
