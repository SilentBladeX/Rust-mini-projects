

fn main() {
    let s = String::from("This is Ahad");

    let first = first_word(&s);
    let second = secong_word(&s);
    let third = third_word(&s);

    println!("First Word in String is {first}");
    println!("Second Word in String is {second}");
    println!("Third Word in String is {third}");
}

fn first_word(s: &String) ->&str{

    let bytes = s.as_bytes();

    for (i,item) in bytes.iter().enumerate(){

        if *item==b' '{
            return &s[..i];
    }
    }
    &s[..]
}
fn secong_word(s: &String) ->&str{

    let bytes=s.as_bytes();

    let mut start=None;

    let mut found=false;
    for (i,item) in bytes.iter().enumerate(){
        if *item == b' '{
            found=true;
            start=Some(i+1);
            break;
        } 
    }

    if !found{
        println!("There's no 2nd word in String");
    }

     if let Some(start_index)=start{
        for (i,item) in bytes[start_index..].iter().enumerate()  {
            if *item==b' '{
                return &s[start_index..start_index+i];
            }
            
        }
        return &s[start_index..];
     }
    

    ""



}
fn third_word(s: &String) ->&str{

    let bytes=s.as_bytes();
    let mut counter=0;
    let mut start=None;
    let mut found=false;

    for (i,item) in bytes.iter().enumerate() {
        if *item==b' '{
            counter+=1;
            if counter==2{
                found=true;
                start=Some(i+1);
                break;
            }

        }
        
    }
    if !found{
        println!("There's no third word in string");
    }

    match start{
        Some(start_index)=>{
            for (i,item) in bytes[start_index..].iter().enumerate() {
                if *item==b' '{
                    return &s[start_index..start_index+i];
                }
                
            }
            return &s[start_index..];
        }

        None=>{
            return "";

        }
    }

}
