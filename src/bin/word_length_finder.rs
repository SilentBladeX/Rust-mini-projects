use std::io;
fn main(){

    println!("Enter a String");
    let mut s = String::new();

    io::stdin().read_line(&mut s).expect("Failed to read msg");

    let s = s.trim();

    let short=small_word(&s);

    println!("Smallest word in string are: {}",short);

    let large=largest_word(&s);

    println!("Smallest word in string are: {}",large);
}

fn small_word(s :&str)->&str{

    let mut  short="";
    let mut min_len=usize::MAX;

    for word in s.split_whitespace(){
        if word.len() < min_len {
            min_len=word.len();
            short=word;
        }
    }

    short

}

fn largest_word(s: &str)->&str{

    let mut largest="";
    let mut max_num=usize::MIN;

    for word in s.split_whitespace(){
        if word.len()>max_num{
            max_num=word.len();
            largest=word;
        }
    }
largest
}