use std::io;
fn main() {
    println!("Enter a number");
    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("failed to read number");

    let num: i32 = num.trim().parse().expect("failed to read number");

    arm_fun(num);
}

fn arm_fun(mut num: i32) {
    let p_num = num;
    let len = num.to_string().len() as u32;
    let mut sum = 0;

    while num > 0 {
        let digit = num % 10;
        let dp = digit.pow(len);
        sum = sum + dp;
        num = num / 10;
    }

    if p_num == sum {
        println!("number is armstrong");
    } else {
        println!("number is not armstrong");
    }
}
