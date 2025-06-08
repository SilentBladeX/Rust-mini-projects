use std::io;

fn main() {
    // Ask the user to enter the size
    println!("Enter size of numbers to take HCF and LCM");
    let mut size = String::new();

    io::stdin()
        .read_line(&mut size)
        .expect("failed to read message");

    // Convert the input from String to i32
    let size: i32 = match size.trim().parse() {
        Ok(size) => size,
        Err(_) => {
            println!("Invalid number");
            return;
        }
    };

    // Create a vector (vec) with given size, initialized with 0s
    let mut vec = Vec::new();

    // Loop to input each element into the array
    for _ in 0..size {
        println!("Enter numbers in vector");
        let mut num = String::new();

        io::stdin()
            .read_line(&mut num)
            .expect("failed to read message");

        // Convert input string to i32
        let num: i32 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number");
                return;
            }
        };
        vec.push(num); // Store number in the array
    }

    // Print the vector
    println!("Your entered number are: {:?}", vec);

    // Calculate GCD of all elements
    let mut result = vec[0];
    for &num in vec.iter().skip(1) {
        result = gcd(result, num); // Reduce GCD pair by pair
    }
    println!("GCD is: {}", result);

    // Calculate LCM of all elements
    let mut result1 = vec[0];
    for &num in vec.iter().skip(1) {
        result1 = lcm(result1, num); // Reduce LCM pair by pair
    }
    println!("LCM is: {}", result1);
}

// Function to calculate GCD using Euclidean algorithm
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b; // Get remainder
        a = temp;
    }
    a
}

// Function to calculate LCM using formula: (a × b) / GCD(a, b)
fn lcm(a: i32, b: i32) -> i32 {
    (a * b) / gcd(a, b)
}
