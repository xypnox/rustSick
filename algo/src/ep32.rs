// Euler Problem 32
//
// We shall say that an n-digit number is pandigital if it makes use of all the
// digits 1 to n exactly once; for example, the 5-digit number, 15234, is 1
// through 5 pandigital.
//
// The product 7254 is unusual, as the identity, 39 × 186 = 7254, containing
// multiplicand, multiplier, and product is 1 through 9 pandigital.
//
// Find the sum of all products whose multiplicand/multiplier/product identity
// can be written as a 1 through 9 pandigital.
// HINT: Some products can be obtained in more than one way so be sure to only
// include it once in your sum.

#[path = "./perm.rs"]
mod perm;

pub fn run() {
    let init_nums = "123456789  ";
    let chars: Vec<String> = init_nums.chars().map(|c| c.to_string()).collect();

    println!("{:?}", chars);
    // for i in 1..10 {
    //     let mut nums = perm::perms_r(init_nums.clone(), i);

    //     // let perms = perm::all_perms(init_nums.clone());

    //     println!("{:?}", nums);
    //     println!("{:?}", nums);
    // }

    let mut sols = vec![];
    for eq in perm::str_perm(chars).iter() {
        let nums: Vec<u32> = eq
            .split_whitespace()
            .map(|x| x.to_string().parse::<u32>().unwrap())
            .collect();

        if nums.len() == 3 {
            if nums[0] * nums[1] == nums[2] {
                sols.push(nums[2]);
            } else if nums[1] * nums[2] == nums[0] {
                sols.push(nums[0]);
            } else if nums[0] * nums[2] == nums[1] {
                sols.push(nums[1]);
            }
        }
    }
    sols.sort();
    sols.dedup();
    println!("{:?}", sols);
}
