use std::io;
fn main(){
    println!("enter a number");
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

    palindrome_fun(num);

    
}

fn palindrome_fun(mut num:i32){
    let p_num=num;
    let mut reverse=0;
    while num!=0{
    let digit=num%10;
    reverse=reverse*10+digit;
    num=num/10;

}
    if p_num==reverse{
        println!("Number is palindrome");
    }
    else{
        println!("Number is not palindrome");
    }

}