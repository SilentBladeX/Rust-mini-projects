use std::io;
fn main() {
    println!("Enter a string");

    let mut str = String::new();

    io::stdin().read_line(&mut str).expect("Failed to read msg");

    let str = str.trim();

    let short = small_word(&str);
    println!("Shortest word in String are : {} ", short);

    let long = long_word(&str);
    println!("Longest word in String are : {} ", long);

}

fn small_word(str: &str) -> &str {
    let mut min_len = usize::MAX;
    let mut short = "";

    for word in str.split_whitespace() {
        if word.len() < min_len {
            min_len = word.len();
            short = word;
        }
    }
    short
}

fn long_word(str: &str) -> &str {
    let mut max_len = usize::MIN;
    let mut long = "";

    for word in str.split_whitespace() {
        if word.len() > max_len {
            max_len = word.len();
            long = word;
        }
    }
    long
}
