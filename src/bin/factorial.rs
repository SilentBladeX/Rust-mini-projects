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

    fact_fun(num);

}
fn fact_fun(num:i32){

    let mut fact=1;
    for i in 1..=num{
         fact=fact*i;
    }
    println!("factorial of num {} is {} ",num,fact);
}