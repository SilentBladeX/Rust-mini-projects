use std::io;
fn main(){
    
    println!("Enter an index");

    let mut index=String::new();

    io::stdin()
    .read_line(&mut index)
    .expect("failed to read number");

    let index:i32=match index.trim().parse(){
        Ok(index)=>index,
        Err(_)=>{
            println!("Invalid number");
            return;
        }
    };

     get_value(index);
}

fn get_value(index:i32){
    let array=[10,20,30,40,50];

    if index<5{
       
        println!("The value at index {} is {} ",index,(array[index as usize]));
    }
    else{
        println!("index out of bound");
    }

}