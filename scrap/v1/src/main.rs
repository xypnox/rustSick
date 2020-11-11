fn main() {
    println!("Hello, world!");
    let x = 3;
    let y = 4;

    println!("x = {}, y = {}", x, y);

    let (x, y) = (y, x);

    println!("x = {}, y = {}", x, y);

    let tup = (0, 3.5, 7);

    println!("tup = ({}, {}, {}), {:?}", tup.0, tup.1, tup.2, tup);
}
