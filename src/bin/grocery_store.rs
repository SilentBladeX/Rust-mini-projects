use std::io;

struct Grocerystore {
    name: String,
    quantity: u32,
    price: f64,
}

fn main() {
    let mut vec: Vec<Grocerystore> = Vec::new();

    loop {
        println!("\n================== Grocery List Manager ==================");
        println!("1. Add items");
        println!("2. View all items");
        println!("3. Calculate total bill");
        println!("4. Remove item by name");
        println!("5. Exit");
        println!("==========================================================");

        println!("Enter your choice: ");
        

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        let choice: i32 = match choice.trim().parse() {
            Ok(c) => c,
            Err(_) => {
                println!("Invalid input! Please enter a number from 1 to 5.");
                continue;
            }
        };

        match choice {
            1 => add_items(&mut vec),
            2 => view_items(&vec),
            3 => total_bill(&vec),
            4 => remove_item(&mut vec),
            5 => {
                println!("Exiting the program. Thank you!");
                break;
            }
            _ => {
                println!("Invalid choice! Please try again.");
            }
        }
    }
}

fn add_items(vec: &mut Vec<Grocerystore>) {
    
    println!("Enter size how many products u want to add");

    let mut size = String::new();

    io::stdin()
        .read_line(&mut size)
        .expect("Failed to read num");

    let size: i32 = match size.trim().parse() {
        Ok(size) => size,
        Err(_) => {
            println!("Invalid number");
            return;
        }
    };

    for _ in 0..size {
        println!("Enter products (Name : Quantity : Price");

        let mut name = String::new();

        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read num");

        let name: String = match name.trim().parse() {
            Ok(name) => name,
            Err(_) => {
                println!("Invalid number");
                return;
            }
        };

        let mut quantity = String::new();

        io::stdin()
            .read_line(&mut quantity)
            .expect("Failed to read num");

        let quantity: u32 = match quantity.trim().parse() {
            Ok(quantity) => quantity,
            Err(_) => {
                println!("Invalid number");
                return;
            }
        };

        let mut price = String::new();

        io::stdin()
            .read_line(&mut price)
            .expect("Failed to read num");

        let price: f64 = match price.trim().parse() {
            Ok(price) => price,
            Err(_) => {
                println!("Invalid number");
                return;
            }
        };

        vec.push(Grocerystore {
            name,
            quantity,
            price,
        });
    }
}




fn view_items(vec: &Vec<Grocerystore>) {
    if vec.is_empty() {
        println!("\n No items in the grocery list yet.");
        return;
    }

    println!("\n🛒 Your Grocery List:");
    println!("-----------------------------------------------------------");
    println!("{:<20} {:<10} {:<10}", "Item Name", "Quantity", "Price");
    println!("-----------------------------------------------------------");

    for item in vec {
        println!("{:<20} {:<10} ${:<10.2}", item.name, item.quantity, item.price);
    }
    println!("-----------------------------------------------------------");
}

fn total_bill(vec: &Vec<Grocerystore>) {
    if vec.is_empty() {
        println!("\n🧾 Your grocery list is empty.");
        return;
    }

    let mut total = 0.0;

    for item in vec {
        total += item.price * item.quantity as f64;
    }

    println!("\n Total Bill: ${:.2}", total);
}

fn remove_item(vec: &mut Vec<Grocerystore>) {

    println!("\n Enter the name of the item you want to remove:");
        
        let mut rem_name = String::new();

        io::stdin()
            .read_line(&mut rem_name)
            .expect("Failed to read num");


    let original_len = vec.len();

    let  rem_name=rem_name.trim().to_lowercase();

    vec.retain(|item| item.name.to_lowercase() != rem_name);

    if vec.len() < original_len {

        println!("Item '{}' removed successfully.", rem_name);

    } 
    else
     {
        println!("Item '{}' not found in the list.", rem_name);
    }
}
