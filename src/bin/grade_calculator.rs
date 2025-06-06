use std::io;

fn main() {
    println!("Enter number of subjects:");

    let mut size = String::new();
    io::stdin().read_line(&mut size).expect("Failed to read msg");

    let size: i32 = match size.trim().parse() {
        Ok(size) if size > 0 => size,
        _ => {
            println!("Invalid size");
            return;
        }
    };

    let mut array: Vec<f64> = vec![0.0; size as usize];

    for i in 0..size {
        loop {
            println!("Enter marks (0-100) for subject {}:", i + 1);
            let mut marks = String::new();
            io::stdin().read_line(&mut marks).expect("Failed to read marks");

            let marks: f64 = match marks.trim().parse() {
                Ok(m) => m,
                Err(_) => {
                    println!("Invalid number. Try again.");
                    continue;
                }
            };

            if marks < 0.0 || marks > 100.0 {
                println!("Marks must be between 0 and 100. Try again.");
            } else {
                array[i as usize] = marks;
                break;
            }
        }
    }

    let out_of = size as f64 * 100.0;
    println!("You entered marks: {:?}", array);

    let total: f64 = array.iter().sum();
    let average = total / size as f64;

    println!("Total Marks: {} out of {}", total, out_of);
    println!("Average Marks: {:.2}", average);

    let grade = grade(average);
    println!("Your Grade is '{}'", grade);
}

fn grade(avg: f64) -> char {
    if avg >= 90.0 && avg <= 100.0 {
        'A'
    } else if avg >= 80.0 {
        'B'
    } else if avg >= 70.0 {
        'C'
    } else if avg >= 60.0 {
        'D'
    } else {
        'F'
    }
}
