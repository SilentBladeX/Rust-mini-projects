use std::io;
fn main(){
    println!("Enter num to generate fibonacci series");
    let mut num=String::new();

    io::stdin().read_line(&mut num).expect("failed to read number");

    let num:i32=match num.trim().parse(){
        Ok(num)=>num,
        Err(_)=>{
            println!("invalid number");
            return;
        }
    };

     fibo_fun(num);

}

fn fibo_fun(num:i32){
    let mut first=0;
    let mut last=1;
    let mut i=0;

    while i<num {
    if i==0{
        println!("{}",first);
    }
    else if i==1{
        println!("{}",last);
    }
    else{
        let temp=first+last;
        first=last;
        last=temp;
        println!("{}",temp);
    }
    i=i+1;
}

}