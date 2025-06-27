use std::io;
fn main() {
    println!("Enter a String");
    let mut str = String::new();

    io::stdin().read_line(&mut str).expect("Failed to read msg");

    let str = str.trim();

    let first = first_word(&str);
    println!("First word in String are {}", first);

    let scnd = second_word(&str);
    println!("Second word in String are {}", scnd);

    let third = third_word(&str);
    println!("Third word in String is {}", third);
}

fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => {
            return &s[..i];
        }

        None => {
            return &s[..];
        }
    }
}

fn second_word(s: &str) -> &str {
    let mut iter = s.split_whitespace();
    iter.next();
    iter.next().unwrap_or("")
}

fn third_word(s: &str) -> &str {
    let mut iter = s.split_whitespace();
    iter.next();
    iter.next();
    iter.next().unwrap_or("")
}
