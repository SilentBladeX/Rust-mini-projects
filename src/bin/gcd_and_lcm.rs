use std::io;

fn main() {
    // Ask the user to enter the size of the array
    println!("Enter size of array");
    let mut size = String::new();

    // Read the input size as a string
    io::stdin()
        .read_line(&mut size)
        .expect("failed to read number");

    // Convert the string to an integer (i32)
    let size: i32 = match size.trim().parse() {
        Ok(size) => size,
        Err(_) => {
            println!("Invalid number");
            return;
        }
    };

    // Create a vector of size 'size' with default value 0
    let mut array = vec![0; size as usize];

    // Take 'size' number of inputs from the user to fill the array
    for i in 0..size {
        println!("Enter numbers in array");
        let mut num = String::new();

        io::stdin()
            .read_line(&mut num)
            .expect("failed to read number");

        // Convert input string to i32 number
        let num: i32 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number");
                return;
            }
        };
        // Assign number to the correct index in the array
        array[i as usize] = num;
    }

    // Print the full array entered by the user
    println!("Your entered array are {:?}", array);

    // Find GCD of all numbers in the array
    let mut result = array[0];
    for &num in array.iter().skip(1) {
        result = gcd(result, num); // call gcd function for each pair
    }
    println!("GCD is {} ", result);

    // Find LCM of all numbers in the array
    let mut result1 = array[0];
    for &num in array.iter().skip(1) {
        result1 = lcm(result1, num); // call lcm function for each pair
    }
    println!("LCM is {} ", result1);
}

// Function to calculate GCD (Greatest Common Divisor)
fn gcd(a: i32, b: i32) -> i32 {
    let min = if a < b { a } else { b };
    let mut gcd = 0;

    // Loop to find the greatest number that divides both a and b
    for i in 1..=min {
        if a % i == 0 && b % i == 0 {
            gcd = i;
        }
    }
    gcd
}

// Function to calculate LCM (Least Common Multiple)
fn lcm(a: i32, b: i32) -> i32 {
    // LCM formula: (a × b) / GCD(a, b)
    (a * b) / gcd(a, b)
}
