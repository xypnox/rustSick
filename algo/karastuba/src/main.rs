extern crate num_bigint;
extern crate num_traits;

use num_traits::{Zero, One};
use num_bigint::BigInt;
// This is fricking incomplete. Please forgive me rust!

fn main() {
    println!("Hello, world!");
    println!("1234 * 5678 = {}", multiply(BigInt::from(12_345_678), BigInt::from(98_765_432)));
    // println!("{} {} {}", len(3), len(233), len(122929));
}

/*
func multiply(m, n)
  if len(m) == 1 or len(n) == 1:
      return m * n
  k = 2 * (max(len(m), len(n)) / 2 + 1 )
  a = m / 10 ** k
  b = m % 10 ** k
  c = n / 10 ** k
  d = n % 10 ** k
  ac = multiply(a, c)
  bd = multiply(b, d)
  mid = multiply(a+b, c+d) - ac -bd
  return ac * 10 ** k + mid * 10 ** k/2 + bd
 */

fn len(a: BigInt) -> BigInt {
    if a / 10 == Zero::zero() {
        return One::one();
    }
    len(a / 10) + 1
}

fn max(p: BigInt, q: BigInt) -> BigInt {
    if p > q {
        return p;
    }
    q
}

fn pow(a: BigInt, b: BigInt) -> BigInt{
    let mut c = a;
    while b > BigInt::from(0) {
        c *= a;
        b -= 1;
    }
    a
}

fn multiply(m: BigInt, n: BigInt) -> BigInt {
    if len(m) == One::one() || len(n) == One::one() {
        return m * n;
    }
    let base: BigInt = BigInt::from(10);
    let k: BigInt = max(len(m), len(n)) / 2;
    let a = m / pow(base, k);
    let b = m % pow(base, k);
    let c = n / pow(base, k);
    let d = n % pow(base, k);

    println!("{} {} {} {} {}", a, b, c, d, k);

    let ac = multiply(a, c);
    let bd = multiply(b, d);
    let mid = multiply(a + b, c + d) - ac - bd;

    println!("{} {} {}", ac, bd, mid);

    ac * pow(base, 2 * k) + mid * pow(base, k) + bd
}