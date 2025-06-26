use std::io;

fn main(){

    println!("Enter a String");
    let mut str=String::new();

    io::stdin()
    .read_line(&mut str)
    .expect("Failed to read smg");

    let str:String=match str.trim().parse(){
        Ok(str)=>str,
        Err(_)=>{
            println!("Invalid string");
            return;
        }
    };


    let small=short_word(&str);
    println!("The Shortest word in String are : {small}");

}

fn short_word(s: &String)->&str{

    let bytes=s.as_bytes();
    let mut _len=0;
    let mut  min_length=usize::MAX;
    let mut short=None;
    let mut start=0;

    for (i,item) in bytes.iter().enumerate(){

        if *item==b' '{
            _len=i-start;
            if _len<min_length{
                min_length=_len;
                short=Some(&s[start..i]);
            }
            start=i+1;

        }
    }

// Handle last word after final space

    if start<s.len(){
        _len=s.len()-start;
        if _len<min_length{
            short=Some(&s[start..]);
        }

    }

    return short.unwrap_or("");
}