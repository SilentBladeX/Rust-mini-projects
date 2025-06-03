use std::io;

fn main() {
    println!("Choose an option:");
    println!("1. Convert from Celsius to Fahrenheit");
    println!("2. Convert from Fahrenheit to Celsius");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");

    let choice: i32 = choice.trim().parse().expect("Please enter a valid number");

    if choice == 1 {
        println!("Enter temperature in Celsius:");

        let mut celsius = String::new();
        io::stdin()
            .read_line(&mut celsius)
            .expect("Failed to read temperature");

        let celsius: f64 = celsius.trim().parse().expect("Please enter a valid number");
        let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;

        println!("Temperature in Fahrenheit: {:.2}°F", fahrenheit);
    } else if choice == 2 {
        println!("Enter temperature in Fahrenheit:");

        let mut fahrenheit = String::new();
        io::stdin()
            .read_line(&mut fahrenheit)
            .expect("Failed to read temperature");

        let fahrenheit: f64 = fahrenheit
            .trim()
            .parse()
            .expect("Please enter a valid number");
        let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;

        println!("Temperature in Celsius: {:.2}°C", celsius);
    } else {
        println!("Invalid choice. Please enter 1 or 2.");
    }
}
