use std::io;

fn main() {
    let mut tup: Vec<(String, bool)> = Vec::new();

    loop {
        println!("=========== Attendace Logger =============");
        println!("Enter 1 for Mark attendace");
        println!("Enter 2 for view all marked names");
        println!("Enter 3 for view total present and absent");
        println!("Enter 4 for Exit program");
        println!("==================================");

        println!("Enter your choice");

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
            1 => mark_attendace(&mut tup),
            2 => mark_names(&tup),
            3 => t_p_a(&tup),
            4 => {
                println!("Program may Exit!!!");
                return;
            }
            _ => {
                println!("Invalid number");
                return;
            }
        }
    }
}

fn mark_attendace(tup: &mut Vec<(String, bool)>) {
    println!("Enter number of Students");

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
        println!("Enter student (Name And Attendace)");
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
        let mut attendace = String::new();

        io::stdin()
            .read_line(&mut attendace)
            .expect("Failed to read number");

        let attendace: bool = match attendace.trim().parse() {
            Ok(attendace) => attendace,
            Err(_) => {
                println!("Invalid number");
                return;
            }
        };

        tup.push((name, attendace));
    }
}

fn mark_names(tup: &Vec<(String,bool)>) {

    println!("======================");
    println!("Marks Entered students are");
    for i in tup{
        println!("Name:  {}   Status: {} ",i.0,i.1);
    }
}

fn t_p_a(tup: &Vec<(String,bool)>) {

    let mut t_present=0;
    let mut t_absent=0;


    for (_,present) in tup{
        if *present {
           t_present=t_present+1 ; 
        } 
        else{
            t_absent+=1;
        }
        
    }

    println!("Total presente are {}", t_present);
    println!("Total absent are {}", t_absent);
}
