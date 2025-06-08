use std::io;

fn main() {
    let mut tasks: Vec<String> = Vec::new();
    loop {
        println!("\nEnter 1 to Add task");
        println!("Enter 2 to view task");
        println!("Enter 3 to remove task by index");
        println!("Enter 4 to Exit!!!\n");

        let mut num = String::new();

        io::stdin().read_line(&mut num).expect("Failed to read smg");

        let num: i32 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number");
                return;
            }
        };

        match num {
            1 => add_task(&mut tasks),
            2 => view_task(&tasks),
            3 => remove_task(&mut tasks),
            4 => {
                println!("Program may Exitt");
                break;
            }
            _ => {
                println!("Invalid number");
                return;
            }
        }
    }
}

fn add_task(tasks: &mut Vec<String>) {
    println!("\nEnter how many task u want to add");

    let mut size = String::new();

    io::stdin()
        .read_line(&mut size)
        .expect("Failed to read smg");

    let size: i32 = match size.trim().parse() {
        Ok(size) => size,
        Err(_) => {
            println!("Invalid number");
            return;
        }
    };

    for _i in 0..size {
        println!("\nEnter your task");

        let mut task = String::new();

        io::stdin()
            .read_line(&mut task)
            .expect("Failed to read smg");

        let task: String = match task.trim().parse() {
            Ok(task) => task,
            Err(_) => {
                println!("Invalid number");
                return;
            }
        };
        tasks.push(task);
    }
}

fn view_task(tasks: &Vec<String>) {
    println!("\nYour Entered task are");

    for (i, task) in tasks.iter().enumerate() {
        println!("{}. {} ", i, task);
    }
}

fn remove_task(tasks: &mut Vec<String>) {
    println!("\nEnter index u want to remove task");

    let mut rem = String::new();

    io::stdin().read_line(&mut rem).expect("Failed to read smg");

    let rem: usize = match rem.trim().parse() {
        Ok(rem) => rem,
        Err(_) => {
            println!("Invalid number");
            return;
        }
    };

    if rem >= tasks.len() {
        println!("Invalid index: out of bounds");
        return;
    } else {
        tasks.remove(rem);

        println!("\nYour Entered task are removed {} ", rem);
    }
}
