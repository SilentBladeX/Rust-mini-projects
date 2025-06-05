use std::io;

fn main(){
    

    println!("Enter 1 to convert Kilometers in Miles");
    println!("Enter 2 to convert Miles in Kilometers ");

    
    let mut  num=String::new();

    io::stdin()
    .read_line(&mut num)
    .expect("failed to read number");

    let num:i32=match num.trim().parse(){
        Ok(dis)=>dis,
        Err(_)=>{
            println!("Invalid number");
            return;
        }
    };
    

    if num==1{
    kilo_to_miles();
    }
    else if num==2{
    miles_to_kilo();
    }
    else {
    println!("Invalid choice. Please enter 1 or 2.");
}

}

fn kilo_to_miles(){
    
    println!("enter distance in kilometeres");
    let mut  dis=String::new();

    io::stdin()
    .read_line(&mut dis)
    .expect("failed to read number");

    let dis:f64=match dis.trim().parse(){
        Ok(dis)=>dis,
        Err(_)=>{
            println!("Invalid number");
            return;
        }
    };
    let result=dis*0.621371;

    println!("The distance in Miles are {:.2} ",result);

}


fn  miles_to_kilo(){
   
    println!("enter distance in Miles");
    let mut  dis=String::new();

    io::stdin()
    .read_line(&mut dis)
    .expect("failed to read number");

    let dis:f64=match dis.trim().parse(){
        Ok(dis)=>dis,
        Err(_)=>{
            println!("Invalid number");
            return;
        }
    };
    let result=dis*1.60934;

    println!("The distance in kilometrs are {:.2} ",result);

}