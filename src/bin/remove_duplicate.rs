use std::io;
fn main() {
    println!("Enter a String");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read msg");

    let input = input.trim().to_string();

    let duplicate = remove_duplicate(input);

    println!("{:?}", duplicate);
}


fn remove_duplicate(s: String) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();

    let words: Vec<&str> = s.split_whitespace().collect();

    for word in words.iter() {
        if !vec.contains(&word.to_string()) {
            vec.push(word.to_string());
        }
    }

    vec
}
