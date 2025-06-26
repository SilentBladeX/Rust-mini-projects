use std::{io};

fn main() {
    println!("Enter a String");

    let mut str = String::new();

    io::stdin()
        .read_line(&mut str)
        .expect("Failed to read string");

    let str = str.trim().to_string(); 
    
    let large = largest_word(&str);

    println!("The largest word is: {large}");
}

fn largest_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    let mut len = 0;
    let mut start = 0;
    let mut max_len = 0;
    let mut large = None;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            len = i - start;
            if len > max_len {
                max_len = len;
                large = Some(&s[start..i]);
            }
            start = i + 1;
        }
    }

    // Handle the last word
    if start < s.len() {
        len = s.len() - start;
        if len > max_len {
            large = Some(&s[start..]);
        }
    }

    large.unwrap_or("")
}
