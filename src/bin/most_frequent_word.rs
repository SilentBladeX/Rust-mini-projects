use std::io;
fn main(){

    println!("Enter a string");
    let mut input=String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("failed to read msg");

    let input=input.trim().to_string();

    let most=mostly_use_word(input);
    println!("Most frequently word is {:?}",most);


}

fn mostly_use_word(m: String)->String{

    let words:Vec<&str>=m.split_whitespace().collect();

    let mut  mfw=String::new();

    let mut max_count=0;

    for i in 0..words.len(){
        let mut count=0;
        for j in 0..words.len(){
            if words[i]==words[j]{
                count=count+1;
            }
        }
        if count>max_count{
            max_count=count;
            mfw=words[i].to_string();
            
            
        }
    }

    mfw



}