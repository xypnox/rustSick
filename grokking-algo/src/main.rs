mod binary_search;

fn main() {
    let res = binary_search::binary_search(vec![1, 2, 4, 6, 7, 8, 10], 1);

    println!("{:?}", res);
}
