use std::io;
fn main(){

    println!("Enter a String");

    let mut s=String::new();

    io::stdin()
    .read_line(&mut s)
    .expect("Failed to read string");

    let s=s.trim();

    println!("Enter word index (starting from 0) to extract nth word: ");
    let mut n=String::new();

    io::stdin()
    .read_line(&mut n)
    .expect("Failed to read string");

    let n:i32=match n.trim().parse(){
        Ok(n)=>n,
        Err(_)=>{
            println!("Invalid number");
            return;
        }
    };

let num=nth_word(&s,n);
println!("the word on index {} is {} ",n,num);


}
fn nth_word(s : &str , n:i32)->&str{
    
    let bytes=s.as_bytes();
    let mut in_word=false;
    let mut start=0;
    let mut count=0;
    

    for (i,bytes) in bytes.iter().enumerate(){
        if *bytes != b' ' && !in_word{
            in_word=true;
            start=i;

        }
        else if *bytes ==b' ' && in_word{
            in_word=false;

            if count==n{
                return &s[start..i];
            }
            count+=1;
        }
    }


    //Hanlde last word

    if in_word && count==n{
        return &s[start..];
    }
""
}