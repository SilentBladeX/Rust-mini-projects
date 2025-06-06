use std::io;


fn main(){
    let mut balance=0.0;
    loop{
    println!("\nEnter 1 for check balance");
    println!("Enter 2 for deposit money");
    println!("Enter 3 for withdraw money");
    println!("Enter 4 for Exit\n");

    let mut num=String::new();

    io::stdin()
    .read_line(&mut num)
    .expect("failed to read number");

    let num:i32=match num.trim().parse(){
        Ok(num)=>num,
        Err(_)=>{
            println!("Invaalid number");
            return;
        }
    };

    match num{
        1 =>  println!("Your current balance is {}" ,balance),
        2 => deposit_money(&mut balance),
        3 => withdraw_money(&mut balance),
        4 => {
            println!("Program may exit");
            break;
        }
        _ => {
            println!("Invalid number");
            return;
        }
    }
}
}



fn deposit_money(balance: &mut f64){
    println!("Enter amount  to deposit");

    let mut amount=String::new();

    io::stdin()
    .read_line(&mut amount)
    .expect("failed to read number");

    let amount:f64=match amount.trim().parse(){
        Ok(bal)=>bal,
        Err(_)=>{
            println!("Invalid number");
            return;
        }
    };

    *balance+=amount;

    println!("Balance {} deposited in your account",amount);
    

}
fn withdraw_money(balance: &mut f64 ){

    println!("Enter withdraw amount");

    let mut w_amount=String::new();

    io::stdin()
    .read_line(&mut w_amount)
    .expect("failed to read msg");


    let w_amount:f64=match w_amount.trim().parse(){
        Ok(w_amount)=>w_amount,
        Err(_)=>{
            println!("Invalid number");
            return;
        }
    };

    if *balance > w_amount{
        
        *balance-=w_amount;
      println!("Your remaining balance is {:.2}", balance);

    }

    else{
        println!("Your withdraw amount is greater than your account balance");
    }

    

}



