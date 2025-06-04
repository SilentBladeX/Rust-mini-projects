use std::io;

fn main() {
    println!("Enter a number to print its table:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let num: i32 = input.trim().parse().unwrap();

    println!("Table of {}:", num);
    for i in 1..=10 {
        println!("{} x {} = {}", num, i, num * i);
    }
}
