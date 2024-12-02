use crate::futils::read_file;

fn parse(input: &str) -> Vec<Vec<i32>> {
    let mut inp: Vec<Vec<i32>> = Vec::new();
    input.lines().for_each(|l| {
        let iter = l.split_whitespace();
        inp.push(iter.map(|s| s.trim().parse::<i32>().unwrap()).collect());
    });

    return inp;
}

enum ArrayState {
    Increasing,
    Decreasing,
    Unknown,
}

fn is_safe(vector: &Vec<i32>) -> bool {
    let mut array_state = ArrayState::Unknown;

    for i in 1..vector.len() {
        let change = vector[i] - vector[i - 1];
        if change.abs() > 3 {
            return false;
        }
        if change == 0 {
            return false;
        }
        match array_state {
            ArrayState::Unknown => {
                if change > 0 {
                    array_state = ArrayState::Increasing;
                } else if change < 0 {
                    array_state = ArrayState::Decreasing;
                }
            }
            ArrayState::Increasing => {
                if change < 0 {
                    return false;
                }
            }
            ArrayState::Decreasing => {
                if change > 0 {
                    return false;
                }
            }
        }
    }

    true
}

fn check_subarrays(vector: &Vec<i32>) -> bool {
    for i in 0..vector.len() {
        let mut sub = vector.clone();
        sub.remove(i);
        if is_safe(&sub) {
            return true;
        }
    }
    false
}

pub fn solution() {
    let contents = read_file("src/y2024/p2.input.txt").expect("Failed to read input file");

    let parsed = parse(&contents);

    let mut count = 0;
    let mut count2 = 0;

    for i in 0..parsed.len() {
        if is_safe(&parsed[i]) {
            count += 1;
            count2 += 1;
        } else if check_subarrays(&parsed[i]) {
            count2 += 1;
        }
        println!(
            "List {:?} Answer {} {}",
            parsed[i],
            is_safe(&parsed[i]),
            check_subarrays(&parsed[i])
        );
    }

    println!("Answer 1: {}", count);
    println!("Answer 2: {}", count2);
}
