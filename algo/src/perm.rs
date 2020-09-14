/// Generates permutation of numbers passed.
/// [1, 2, 3] -> [123, 132, 213, 231, 312, 321]
pub fn nums_10(nums: Vec<u32>) -> Vec<u32> {
    let numlen = nums.len();

    if numlen == 1 {
        nums
    } else {
        let mut sols = vec![];

        for (index, number) in nums.iter().enumerate() {
            let mut new_vec = nums.clone();
            new_vec.remove(index);
            for n in nums_10(new_vec).clone().iter() {
                let pow: u32 = (numlen - 1) as u32;
                sols.push(number * 10u32.pow(pow) + n);
            }
        }

        sols
    }
}

/// Generates permutation of numbers passed.
/// [1, 2, 3] -> [123, 132, 213, 231, 312, 321]
pub fn perms_r(nums: Vec<u32>, r: usize) -> Vec<u32> {
    let numlen = nums.len();

    if r > numlen {
        return nums;
    }

    if r == 1 {
        return nums;
    }

    if numlen == 1 {
        nums
    } else {
        let mut sols = vec![];

        for (index, number) in nums.iter().enumerate() {
            let mut new_vec = nums.clone();
            new_vec.remove(index);
            for n in perms_r(new_vec, r - 1).clone().iter() {
                let pow: u32 = (r - 1) as u32;
                sols.push(number * 10u32.pow(pow) + n);
            }
        }

        sols
    }
}

pub fn all_perms(nums: Vec<u32>) -> Vec<u32> {
    let numlen = nums.len();

    if numlen == 1 {
        nums
    } else {
        let mut sols = vec![];

        for (index, number) in nums.iter().enumerate() {
            let mut new_vec = nums.clone();
            new_vec.remove(index);
            for n in all_perms(new_vec).clone().iter() {
                let pow: u32 = (numlen - 1) as u32;
                sols.push(number * 10u32.pow(pow) + n);
                if !sols.contains(n) {
                    sols.push(*n);
                }
            }
            sols.push(*number);
        }

        sols
    }
}

//
pub fn str_perm(chars: Vec<String>) -> Vec<String> {
    let len = chars.len();

    if len == 1 {
        chars
    } else {
        let mut sols = vec![];
        for (index, c) in chars.iter().enumerate() {
            let mut new_vec = chars.clone();
            if c == " " && (len == 10) {
                break;
            }
            new_vec.remove(index);
            for n in str_perm(new_vec).clone().iter() {
                sols.push(format!("{}{}", c, n));
            }
        }

        sols
    }
}
