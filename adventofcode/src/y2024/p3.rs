use crate::futils::read_file;

// 0123456789
// mul(XXX,XXX)
// need to scan at most 12 chars after mul

fn get_valid_nums(input: &str) -> Option<i32> {
    if input.chars().nth(3) != Some('(') {
        return None;
    }
    let sl_start = input.find('(')? + 1;
    let sl_end = input.find(')')?;
    let sliceds = &input[sl_start..sl_end];
    let (num1, num2) = sliceds.split_once(',')?;
    let pn1 = match num1.parse::<i32>() {
        Ok(n) => n,
        Err(_e) => 0,
    };
    let pn2 = match num2.parse::<i32>() {
        Ok(n) => n,
        Err(_e) => 0,
    };
    return Some(pn1 * pn2);
}

fn valid_mul(input: &str, with_instructions: bool) -> i32 {
    let mut val = 0;
    let v: Vec<_> = input.match_indices("mul").collect();
    let dos: Vec<_> = input.match_indices("do()").collect();
    let donts: Vec<_> = input.match_indices("don't()").collect();

    let ignore_ranges: Vec<[usize; 2]> = if with_instructions {
        donts
            .clone()
            .into_iter()
            .map(|(i, _)| {
                let nextdo = match dos.iter().position(|&(r, _)| r > i) {
                    Some(i) => dos[i].0,
                    None => input.len(),
                };
                [i, nextdo]
            })
            .collect()
    } else {
        // for speed dont generate when no with_instructions
        vec![]
    };

    for m in v.into_iter() {
        let (i, _) = m;
        if with_instructions {
            let is_disabled_range: bool = {
                let inside_r = ignore_ranges.iter().find(|r| r[0] < i && r[1] > i);
                let r = match inside_r {
                    Some(_k) => true,
                    None => false,
                };
                r
            };
            if is_disabled_range {
                continue;
            }
        }
        // remember max 12 (could be 11 XD)
        let slice_end = std::cmp::min(input.len(), i + 12);
        let slic = &input[i..slice_end];
        let r = get_valid_nums(slic);
        match r {
            Some(res) => val += res,
            None => continue,
        }
    }

    val
}

pub fn solution() {
    let contents = read_file("src/y2024/p3.input.txt").expect("Failed to read input file");
    let a1 = valid_mul(&contents, false);
    let a2 = valid_mul(&contents, true);
    println!("Answer 1: {}", a1);
    println!("Answer 2: {}", a2);
}
