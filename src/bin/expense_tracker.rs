use std::io;

fn main() {
    let mut tup: Vec<(f64, String)> = Vec::new();

    loop {
        println!("\nEnter 1 for add expense (Amount and Description");
        println!("Enter 2 for view all expenses");
        println!("Enter 3 for total expense");
        println!("Enter 4 for Exit\n");
        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read msg");

        let choice: i32 = match choice.trim().parse() {
            Ok(choice) => choice,
            Err(_) => {
                println!("Invalid number");
                return;
            }
        };

        match choice {
            1 => add_expense(&mut tup),
            2 => view_expense(&tup),
            3 => total_expenses(&tup),
            4 => {
                println!("Program may exit");
                break;
            }
            _ => {
                println!("Invalid number");
                return;
            }
        }
    }
}

fn add_expense(tup: &mut Vec<(f64, String)>) {
    println!("\nEnter number of expenses u want to add");

    let mut size = String::new();

    io::stdin()
        .read_line(&mut size)
        .expect("Failed to read msg");

    let size: i32 = match size.trim().parse() {
        Ok(size) => size,
        Err(_) => {
            println!("Invalid number");
            return;
        }
    };

    for _ in 0..size {
        println!("Enter your expenses (amount and description) ");

        let mut amount = String::new();
        io::stdin()
            .read_line(&mut amount)
            .expect("Failed to read msg");

        let amount: f64 = match amount.trim().parse() {
            Ok(amount) => amount,
            Err(_) => {
                println!("Invalid number");
                return;
            }
        };

        let mut description = String::new();
        io::stdin()
            .read_line(&mut description)
            .expect("Failed to read msg");

        let description: String = match description.trim().parse() {
            Ok(description) => description,
            Err(_) => {
                println!("Invalid number");
                return;
            }
        };
        tup.push((amount, description));
    }
}

fn view_expense(tup: &Vec<(f64, String)>) {
    println!("\nYour entered expenses are");

    for (i, (amount, description)) in tup.iter().enumerate() {
        println!("{}. {}  {} ", i, amount, description);
    }
}

fn total_expenses(tup: &Vec<(f64, String)>) {
    let mut sum = 0.0;

    for i in tup.iter() {
        sum = sum + i.0;
    }
    println!("\nTotl expense are {}", sum);
}
