use std::io;

fn main() {
    println!("Enter a number:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let number: i32 = input.trim().parse().unwrap();

    if number % 2 == 0 {
        println!("{} is Even", number);
    } else {
        println!("{} is Odd", number);
    }
}
