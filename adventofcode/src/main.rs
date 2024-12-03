#![allow(dead_code)]

mod futils;
mod y2024;

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let b = add(1, 2);
    println!("b = {}", b);
    println!("Hello, world!");

    y2024::p3::solution();
}
