use std::io;
fn main() {
    println!("Enter a number");

    let mut num = String::new();

    io::stdin().read_line(&mut num).expect("failed to read msg");

    let num: i32 = match num.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("invalid number");
            return;
        }
    };

    prime_fun(num);
}

fn prime_fun(num: i32) {
    if num < 2 {
        println!("Number is not prime");
        return;
    }

    let sqrt_num = (num as f64).sqrt() as i32;
    let mut is_prime = true;

    for i in 2..=sqrt_num {
        if num % i == 0 {
            is_prime = false;
            break;
        }
    }

    if is_prime {
        println!("Number is prime");
    } else {
        println!("numbe is not prime");
    }
}
