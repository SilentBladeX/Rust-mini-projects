use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let mut counter=1;
    let secret_number: i32 = rand::thread_rng().gen_range(1..6);

    loop {
        println!("\nguess a num between 1 to 6 ");
        let mut num = String::new();

        io::stdin()
            .read_line(&mut num)
            .expect("failed to read number");

        let u_num: i32 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("invalid number");
                return;
            }
        };

        println!("You Entered number is {} ", u_num);
        
        match u_num.cmp(&secret_number) {
            Ordering::Less => {
                println!("Number is less than guessing number");
            }
            Ordering::Greater => {
                println!("Number is Greater than guessing number")
            }
            Ordering::Equal => {
                println!("Number is equal");
                println!("Congratulation you won the game!!!");

                break;
            }
        }
        counter+=1;
    }
    println!("\nSecret number is {} ", secret_number);
    println!("\nYou take {} attempts to won the game ",counter);
}
