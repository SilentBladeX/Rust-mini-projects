use std::io;

fn main() {
    println!("Enter a number");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failes to read String");

    let input = input.trim();

    let second = second_mostly_used(&input);
    println!("{}", second);
}

fn second_mostly_used(input: &str) -> String {
    let words: Vec<&str> = input.split_whitespace().collect();
    let mut _already_counted = false;
    let mut counts: Vec<(i32, String)> = Vec::new();

    for i in 0..words.len() {
        _already_counted = false;

        for j in 0..i {
            if words[i] == words[j] {
                _already_counted = true;
                break;
            }
        }

        if !_already_counted {
            let mut count = 0;
            for w in words.iter() {
                if *w == words[i] {
                    count += 1;
                }
            }
            counts.push((count, words[i].to_string()));
        }
    }

    counts.sort_by(|a, b| b.0.cmp(&a.0));

    if counts.len() < 2 || counts[0].1 == counts[1].1 {
        "No second most frequent word".to_string()
    } else {
        counts[1].1.clone()
    }
}
