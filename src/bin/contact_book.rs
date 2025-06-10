use std::io;

struct ContactBook {
    name: String,
    phone: String,
    email: String,
}

fn main() {
    let mut contact: Vec<ContactBook> = Vec::new();
    loop {
        println!("\n============Contact Book============");
        println!("1. Add Contact");
        println!("2. Update Contact");
        println!("3. View All Contacts");
        println!("4. Delete Contact");
        println!("5. Sort Contacts (Name / Email)");
        println!("6. Search Contact (Name / Email)");
        println!("7. Exit");
        println!("==================================");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read number");

        let choice: i32 = match choice.trim().parse() {
            Ok(choice) => choice,
            Err(_) => {
                println!("Invalid number");
                return;
            }
        };

        match choice {
            1 => add_contact(&mut contact),
            2 => update_contact(&mut contact),
            3 => view_contact(&contact),
            4 => delete_contact(&mut contact),
            5 => sort_contact(&mut contact),
            6 => search_contact(&contact),
            7 => {
                println!("Program may Exit!!!");
                break;
            }
            _ => {
                println!("Invalid number");
                return;
            }
        }
    }
}

fn add_contact(contact: &mut Vec<ContactBook>) {
    println!("\nHow many contacts would you like to add?");
    let mut size = String::new();

    io::stdin()
        .read_line(&mut size)
        .expect("Failed to read number");

    let size: i32 = match size.trim().parse() {
        Ok(size) => size,
        Err(_) => {
            println!("Invalid number");
            return;
        }
    };

    for _ in 0..size {
        println!("Enter contact details: Name, Phone, Email");

        let mut name = String::new();

        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read number");

        let name: String = match name.trim().parse() {
            Ok(name) => name,
            Err(_) => {
                println!("Invalid number");
                return;
            }
        };

        let mut phone = String::new();

        io::stdin()
            .read_line(&mut phone)
            .expect("Failed to read number");

        let phone: String = match phone.trim().parse() {
            Ok(phone) => phone,
            Err(_) => {
                println!("Invalid number");
                return;
            }
        };

        let mut email = String::new();

        io::stdin()
            .read_line(&mut email)
            .expect("Failed to read number");

        let email: String = match email.trim().parse() {
            Ok(email) => email,
            Err(_) => {
                println!("Invalid number");
                return;
            }
        };

        contact.push(ContactBook { name, phone, email });

        println!("✅ Contact added successfully.");
    }
}
fn view_contact(contact: &Vec<ContactBook>) {
    println!("\n📇 Contact List:");

    for i in contact.iter() {
        println!("----------------------------");
        println!("Name  : {}", i.name);
        println!("Phone : {}", i.phone);
        println!("Email : {}", i.email);
    }
    println!("----------------------------");
}
fn update_contact(contact: &mut Vec<ContactBook>) {
    println!("\nEnter the name of the contact to update:");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read number");

    let name: String = match name.trim().parse() {
        Ok(name) => name,
        Err(_) => {
            println!("Invalid number");
            return;
        }
    };

    let mut found = false;

    for i in contact.iter_mut() {
        if name == i.name {
            found = true;

            println!("Enter new phone:");

            let mut phone = String::new();

            io::stdin()
                .read_line(&mut phone)
                .expect("Failed to read number");

            let phone: String = phone.trim().parse().expect("failed to read phone");

            println!("Enter new email:");

            let mut email = String::new();

            io::stdin()
                .read_line(&mut email)
                .expect("Failed to read number");

            let email: String = email.trim().parse().expect("failed to read email");

            i.phone = phone;
            i.email = email;

            println!("✅ Contact updated successfully.");
        }
    }
    if !found {
        println!("❌ Contact not found.");
    }
}

fn delete_contact(contact: &mut Vec<ContactBook>) {
    println!("\nEnter the name of the contact to delete:");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read number");

    let name: String = match name.trim().parse() {
        Ok(name) => name,
        Err(_) => {
            println!("Invalid number");
            return;
        }
    };

    let index = contact.iter().position(|i| i.name == name);

    match index {
        Some(index) => {
            contact.remove(index);
            println!("Contact deleted sucessfully");
        }
        None => {
            println!("Contact Not found");
        }
    }
}

fn sort_contact(contact: &mut Vec<ContactBook>) {
    println!("\nChoose sorting option:");
    println!("1. Sort by Name");
    println!("2. Sort by Email");
    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read number");

    let choice: i32 = match choice.trim().parse() {
        Ok(choice) => choice,
        Err(_) => {
            println!("Invalid number");
            return;
        }
    };

    match choice {
        1 => {
            contact.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
            println!("Contact sorted by name.");
        }

        2 => {
            contact.sort_by(|a, b| a.email.to_lowercase().cmp(&b.email.to_lowercase()));
            println!("Contact sorted by email");
        }
        _ => {
            println!("invalid choice");
            return;
        }
    }
}
fn search_contact(contact: &Vec<ContactBook>) {
    println!("\n🔍 Enter name or email to search:");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read number");

    let choice: String = match choice.trim().to_lowercase().parse() {
        Ok(choice) => choice,
        Err(_) => {
            println!("Invalid number");
            return;
        }
    };

    let mut found = false;
    for i in contact.iter() {
        if i.name.to_lowercase().contains(&choice) || i.email.to_lowercase().contains(&choice) {
            found = true;
            println!("\n✅ Contact found:");
            println!("Name  : {}", i.name);
            println!("Phone : {}", i.phone);
            println!("Email : {}", i.email);
        }
        if !found {
            println!("❌ No contact matched your search.");
        }
    }
}
