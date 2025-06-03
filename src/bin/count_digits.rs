use std::io;

fn main() {
    println!("Enter a number");
    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("failed to read number");

    let num: i32 = num.trim().parse().expect("failed to read number");

    let x=cnt_fun(num);
    println!("Total digits are {} ",x);
}

fn cnt_fun(mut num:i32)->i32{
    let mut count=0;
    while num>0{
        let _digit=num%10;
        count+=1;
        num=num/10;
    }
    count
}
