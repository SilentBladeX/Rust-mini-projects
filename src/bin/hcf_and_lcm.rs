use std::io;

fn main() {
    // Ask the user to enter the size of the array
    println!("Enter size of array");
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

    // Create a vector (array) with given size, initialized with 0s
    let mut array = vec![0; size as usize];

    // Loop to input each element into the array
    for i in 0..size {
        println!("Enter numbers in array");
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
        array[i as usize] = num; // Store number in the array
    }

    // Print the array
    println!("Your array is: {:?}", array);

    // Calculate GCD of all elements
    let mut result = array[0];
    for &num in array.iter().skip(1) {
        result = gcd(result, num); // Reduce GCD pair by pair
    }
    println!("GCD is: {}", result);

    // Calculate LCM of all elements
    let mut result1 = array[0];
    for &num in array.iter().skip(1) {
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
