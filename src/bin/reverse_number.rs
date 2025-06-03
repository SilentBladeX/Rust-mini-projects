use std::io;
fn main(){
    println!("enter a number to reverse");
    let mut num=String::new();

    io::stdin()
    .read_line(&mut num)
    .expect("failed to read number");

    let num:i32=match num.trim().parse(){
        Ok(num)=>num,
        Err(_)=>{
            println!("Invalid number");
            return;
        }

    };

    let x=rev_fun(num);

    println!("reverse of number is {} ",x)
}

fn rev_fun(mut num:i32)->i32{
    let mut reverse=0;
    while num!=0{
    let digit=num%10;
    reverse=reverse*10+digit;
    num=num/10;

}
    reverse

}