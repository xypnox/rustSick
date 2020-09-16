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

use std::iter::FromIterator;
use std::iter::Iterator;

pub fn run() {
    let mut sols: Vec<u32> = vec![];

    for i in 1..10000 {
        for j in 1..10000 {
            if is_pandigital(i, j, i * j) {
                sols.push(i * j);
            }
        }
    }

    sols.sort();
    sols.dedup();
    println!("{:?}", sols);
    println!("{}", sols.iter().sum::<u32>());
}

fn is_pandigital(n1: u32, n2: u32, n3: u32) -> bool {
    let mut chars: Vec<char> = (&format!("{}{}{}", n1, n2, n3)[..]).chars().collect();
    chars.sort();
    String::from_iter(chars) == "123456789"
}
