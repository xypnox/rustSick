use std::collections::HashMap;

use crate::futils::read_file;

fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut first_list: Vec<i32> = Vec::new();
    let mut second_list: Vec<i32> = Vec::new();
    input.lines().for_each(|l| {
        let (f, l) = l.split_once(" ").unwrap();
        // println!("{i}, {f}, {l}");
        first_list.push(f.trim().parse::<i32>().unwrap());
        second_list.push(l.trim().parse::<i32>().unwrap());
    });
    first_list.sort();
    second_list.sort();

    return (first_list, second_list);
}

pub fn solution() {
    let contents = read_file("src/y2024/p1.input.txt").expect("Failed to read input file");

    let (first_list, second_list) = parse(&contents);

    let mut dists: Vec<i32> = Vec::new();
    for i in 0..first_list.len() {
        dists.push((first_list[i] - second_list[i]).abs());
    }
    let answer = dists.clone().into_iter().reduce(|a, b| a + b).unwrap();

    println!("Answer 1: {}", answer);

    let mut hash_m: HashMap<i32, i32> = HashMap::new();
    for i in 0..second_list.len() {
        let num = second_list[i];
        if !hash_m.contains_key(&num) {
            hash_m.insert(num, 1);
        } else {
            hash_m.insert(num, 1 + hash_m[&num]);
        }
    }
    /* cumulative_similarity_score  */
    let mut css = 0;
    for i in 0..first_list.len() {
        let num = first_list[i];
        let val = hash_m.get(&num).unwrap_or(&0) * num;
        css += val;
    }

    println!("Answer 2: {}", css);
}
