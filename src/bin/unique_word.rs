use std::io;
fn main(){

    let mut input=String::new();

    println!("Enter a string");

    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read msg");

    let input=input.trim().to_string();

    let unique=unique_word(input);
    println!("{:?}",unique);

}

fn unique_word(input: String)->Option<String>{
    let words:Vec<&str>=input.split_whitespace().collect();

    for word in words.iter(){

        let mut count=0;

        for other in words.iter(){
            if word==other{
            count=count+1;
            }
        }

        if count==1{
            return Some(word.to_string());
        }
    }
    None

}