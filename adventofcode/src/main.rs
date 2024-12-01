#![allow(dead_code)]

mod y2024;
mod futils;

fn add(a: i32, b: i32)-> i32 {
    a + b
}

fn main() {
    let b = add(1, 2);
    println!("b = {}", b);
    println!("Hello, world!");

    y2024::p1::solution();

}
