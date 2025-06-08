use std::io;

fn main(){
    println!("Enter password");

    let mut pass=String::new();

    io::stdin()
    .read_line(&mut pass)
    .expect("failed to read msg");

    let pass:String=match pass.trim().parse(){
        Ok(pass)=>pass,
        Err(_)=>{
            println!("Invalid input");
            return;

        }
    };

    if pass.len()<8{
        println!("Password must be 8 characters long");
    }

    let mut _has_upper=false;
    let mut _has_digit=false;
    let mut _has_special=false;

    for ch in pass.chars(){
        if ch.is_uppercase(){   
            _has_upper=true;
        }
        if ch.is_numeric(){
            _has_digit=true;
        }
        if "!@#$%^&*()`".contains(ch){
            _has_special=true;
        }
    }

    if _has_digit && _has_special && _has_upper{
        println!("Password is valid!!!");
    }
    else{
        if !_has_digit{
            println!("Password must have 1 numeric value");
        }
        if !_has_special{
            println!("Password must have 1 special character");
        }
        if !_has_upper{
            println!("Password must have 1 Upper character");
        }
    }


}