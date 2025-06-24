# Rust-mini-projects
A collection of beginner-friendly Rust mini projects like temperature converter, Fibonacci series, and more — built for practice and learning.


---

## 🌡️ Temperature Converter in Rust
This is a simple command-line Rust program that allows users to convert temperatures between Celsius and Fahrenheit.

### 🚀 Features
- Convert Celsius to Fahrenheit  
- Convert Fahrenheit to Celsius  
- Input handling and error messages  
- Clean output formatting with two decimal places  

### 🛠️ How to Run
Make sure you have Rust and Cargo installed.

```bash
cargo new Rust-mini-projects
cd Rust-mini-projects
cargo run
```

### 💡 Sample Output
```
Choose an option:
1. Convert from Celsius to Fahrenheit
2. Convert from Fahrenheit to Celsius
> 1
Enter temperature in Celsius:
> 100
Temperature in Fahrenheit: 212.00°F
```

---

## 🚀 Fibonacci Series Generator in Rust
This is a simple Rust program that takes a number from the user and prints the Fibonacci series up to that number of terms.

### 📋 Features
- Takes input from the user at runtime  
- Validates user input  
- Generates Fibonacci series using a `while` loop  
- Clean and readable Rust code

### 💡 Output
```
Enter num to generate fibonacci series
5
0
1
1
2
3
```

---

## 🎄 The Twelve Days of Christmas (Rust)
This Rust program prints the lyrics of the classic Christmas carol **“The Twelve Days of Christmas”**, using a loop and array to elegantly handle repetition and structure.

### ✨ Features
- Uses nested loops to avoid redundant printing  
- Dynamically adds ordinal suffixes like `1st`, `2nd`, `3rd`, `4th`, etc.  
- Uses a fixed array of gifts to generate lyrics efficiently  
- Demonstrates string formatting and loop control in Rust

### 💡 Sample Output
```
On the 1st day of Christmas, my true love sent to me
"A Partridge in a Pear Tree"

On the 2nd day of Christmas, my true love sent to me
"Two Turtle Doves"
AND, "A Partridge in a Pear Tree"
...
```

---

## 🎯 Guessing Game (Rust)
This is a simple command-line Guessing Game built in Rust as a beginner practice project.

### 📋 How It Works
- The computer randomly generates a number between **1 to 5**  
- You are prompted to guess the number  
- After each guess, the program tells you:
  - If your number is too low
  - If your number is too high
  - Or if it's correct  
- The game keeps running until you guess the correct number  
- At the end, it shows:
  - The secret number  
  - The number of attempts you took  

### 🧠 Concepts Used
- `rand::thread_rng().gen_range()` for random number generation  
- `std::io` for taking user input  
- `std::cmp::Ordering` for comparison logic  
- Looping and match statements  
- Basic error handling  

---

## 🧮 Array Value by Index
This is a simple Rust program that asks the user to input an index and returns the value at that index from a predefined array.

### 🔧 How It Works
- The user is prompted to enter an index  
- The input is read as a string and then parsed into an integer  
- The index is checked to ensure it's within bounds (less than array length)  
- If the index is valid, the value at that index is printed  
- If the index is out of bounds or input is invalid, an error message is shown  

### 📌 Notes
- The array used is: `[10, 20, 30, 40, 50]`  
- Only indices `0` to `4` are valid  
- Out-of-bounds access is handled gracefully  


---


## 🔁 Reverse Number (Rust)
This is a simple Rust command-line program that takes a number from the user and prints its reverse. It's a beginner-friendly project to practice input handling, loops, and basic arithmetic.

### 🚀 Features
Takes user input at runtime
Validates input
Uses a custom function to reverse the number
Prints the reversed number

---


## Digit Counter in Rust

### 📝 Description:
This Rust program reads an integer input from the user and counts the total number of digits in that number. For example, if the input is 12345, the output will be 5.

### 🛠️ How It Works:
The program prompts the user to enter a number.
The input is read as a string using read_line()
The string is trimmed and parsed into a 32-bit integer (i32).
The integer is passed to a custom function cnt_fun() that:
Uses a loop to extract and discard each digit using % and /.
Increments a counter on each iteration.
Finally, the program prints the total number of digits.


---

## 📌 Sum of Digits

### 📝 Description:
This Rust program takes an integer input from the user and calculates the sum of its digits. For example, if the user enters 1234, the output will be 10 (since 1 + 2 + 3 + 4 = 10).

### 🛠️ How It Works:
The program prompts the user to enter a number.
It reads the input as a String using read_line() and stores it in a mutable variable.
The string input is trimmed and parsed into a 32-bit integer (i32).
The number is passed to a custom function sum_fun() which:
Extracts each digit using modulus %.
Adds the digit to a running total (sum).
Removes the last digit using integer division /.
The function returns the final sum.
The result is printed to the console.


---

## 📌 Prime Number Checker 

### 📝 Description:
This Rust program reads an integer input from the user and determines whether the number is prime or not prime. It uses an efficient method by checking divisibility only up to the square root of the input number.

### 🛠️ How It Works:
The program prompts the user to enter a number.
It reads the input as a string and converts it into an integer.
The function prime_fun checks whether the number is prime:
If the number is less than 2, it is not prime.
Otherwise, it checks divisibility from 2 up to the square root of the number.
If any divisor is found, the number is declared not prime.
Otherwise, it is declared prime.
The program prints the result to the console.


---

## 📌 Armstrong Number Checker 

### 📝 Description:
This Rust program checks whether a user-input number is an Armstrong number. An Armstrong number is one in which the sum of each digit raised to the power of the total number of digits is equal to the original number.

### 🛠️ How It Works:
Prompts the user to enter a number.
Reads the number from standard input as a string.
Converts the string input to an integer.
Calculates the number of digits.
Separates each digit and computes the power of each digit.
Sums up all powered digits.
Compares the sum with the original number.
Prints whether the number is an Armstrong number or not.


---


## 📌 Palindrome Number Checker 

### 📝 Description:
This Rust program checks whether a given number is a palindrome. A number is considered a palindrome if it reads the same forward and backward (e.g., 121, 1331, 12321).

### 🛠️ How It Works:
Prompts the user to enter a number.
Reads the input from standard input as a String.
Parses the string into an i32 integer.
If input is invalid, it prints an error and exits.
Reverses the number using a loop:
Extracts digits using modulus
Builds reversed number step by step
Compares the reversed number to the original number.
Prints whether the number is a palindrome.

---


## 📌 Factorial Calculator 

### 📝 Description:
This Rust program calculates the factorial of a user-input positive integer.
The factorial of a number n is the product of all positive integers from 1 to n (denoted as n!).

### 🛠️ How It Works:
Prompts the user to enter a number.
Reads input from standard input as a string.
Converts the string input into an i32 integer.
If the input is invalid (non-numeric), it prints an error and exits.
Uses a for loop to compute the factorial.
Prints the factorial result.


---


## GCD and LCM Calculator in Rust

This Rust program reads an vector of integers from the user and calculates:
GCD (Greatest Common Divisor) of all the elements
LCM (Least Common Multiple) of all the elements

### Features:

Accepts dynamic input for vector size and elements
Handles invalid input gracefully
Calculates GCD using Euclidean Algorithm
Calculates LCM using formula:
LCM(a, b) = (a × b) / GCD(a, b)

### Logic Explanation:

GCD is calculated pair by pair using the Euclidean Algorithm:
Repeatedly use:
a % b → swap values → repeat until remainder is 0

LCM is calculated using the formula:
LCM(a, b) = (a * b) / GCD(a, b)
This is also done pair by pair across the array.



---



## 📘  GCD and LCM 

### 📝 Description:
This Rust program allows a user to:
1. Enter the size of an vector.
2. Input integer elements into the vector.
3. Calculate and print the **GCD (Greatest Common Divisor)** of all numbers in the vector.
4. Calculate and print the **LCM (Least Common Multiple)** of all numbers in the vector.



### 📌 Features:
- Input validation (ensures all entries are integers).
- Dynamically stores user input using a `Vec<i32>`.
- Efficiently computes GCD and LCM using custom functions.


### 🧮 Logic:

### ✅ GCD Function (Brute Force Method):
- Takes two numbers `a` and `b`.
- Finds the smaller number using `min = if a < b { a } else { b }`.
- Iterates from 1 to `min`.
- Checks for the greatest number that divides both `a` and `b` without remainder.
- Updates `gcd` whenever a common divisor is found.

### ✅ LCM Function:
- Uses the formula:  
  **LCM(a, b) = (a × b) / GCD(a, b)**
- This method avoids unnecessary loops and reuses the GCD logic.



### 🖥️ How It Works:
1. Program asks for vector size.
2. User enters numbers one by one.
3. GCD is computed across all vector values using a loop and the brute force method.
4. LCM is computed using the multiplication and division formula.
5. Both results are displayed.


### 🛠️ Note:
- GCD is computed using the brute-force method, which is simple and easy to understand for beginners.
- This program handles errors gracefully using `match` for parsing integers.



---


## 1. 🧮 Calculator


### Description:
A basic CLI calculator that takes two numbers from the user and performs addition, subtraction, multiplication, or division based on the selected operator.

### How it works:
- Prompts the user for two numbers.
- Asks which operation to perform (`+`, `-`, `*`, `/`).
- Prints the calculated result.
- Handles divide-by-zero gracefully.



---




## 2. 🔢 Even or Odd Checker

### Description:
A simple program to check whether a number entered by the user is even or odd.

### How it works:
- User enters a whole number.
- Program checks `number % 2`.
- Displays if the number is **Even** or **Odd**.


---



## 3. 📋 Table Printer

### Description:
Prints the multiplication table (paara) of any number entered by the user.

### How it works:
- Takes one number from the user.
- Uses a `for` loop from 1 to 10.
- Prints the multiplication result line-by-line.


---


## 🚗 Distance Converter in Rust

This is a simple Rust CLI program that allows the user to convert distances between:

- Kilometers to Miles
- Miles to Kilometers

### 🧠 How It Works

1. The program asks the user to enter a choice:
   - Enter `1` to convert **Kilometers to Miles**
   - Enter `2` to convert **Miles to Kilometers**

2. Based on the user's choice:
   - If `1`, it prompts for a distance in kilometers and converts it to miles.
   - If `2`, it prompts for a distance in miles and converts it to kilometers.
   - Any other input results in an "Invalid choice" message.

3. The program handles invalid number inputs gracefully using `match`.

### 🧮 Conversion Formulas

- **Kilometers to Miles**: `miles = km × 0.621371`
- **Miles to Kilometers**: `km = miles × 1.60934`


---


## Mini ATM Program in Rust
=========================

### 📌 Description:
---------------
This is a simple command-line ATM simulation program written in Rust. It allows the user to:
1. Check account balance
2. Deposit money
3. Withdraw money
4. Exit the program

The balance is stored and updated using mutable references, demonstrating basic concepts of ownership and borrowing in Rust.

### 🔧 How It Works:
----------------
- The program starts with an initial balance of 0.0.
- A menu is shown to the user with 4 options.
- The user selects an option by entering a number.
- For deposit and withdrawal, the program asks for the amount.
- Balance is updated in memory and shown to the user.

### 📋 Menu Options:
----------------
1 → Check current balance  
2 → Deposit money  
3 → Withdraw money  
4 → Exit the program  

### ✅ Features:
------------
- Input validation for all numeric entries
- Handles incorrect input (e.g., alphabets or symbols)
- Prevents overdrawing (i.e., withdrawing more than the balance)
- Uses `match` statements and references properly


---


## Grade Calculator in Rust

This Rust program calculates the total marks, average marks, and grade for a student based on marks entered for multiple subjects.

### Features

Takes the number of subjects as input.
Takes marks input for each subject individually.
Validates each mark to be between 0 and 100.
Calculates total marks out of the maximum possible marks.
Calculates the average marks.
Assigns a grade based on the average marks using the following scale:

A: 90 - 100
B: 80 - 89.99
C: 70 - 79.99
D: 60 - 69.99
F: below 60

Handles invalid inputs gracefully and prompts user again.

### How to Use
Run the program
Enter the number of subjects (must be a positive integer).
Enter marks for each subject (each between 0 and 100).
After all marks are entered, the program displays:
The marks entered

Total marks and the maximum possible marks
Average marks (rounded to two decimals)
Assigned grade

### Notes

Input validation ensures marks are within the valid range.
The program uses a loop to keep asking for valid marks until they are entered correctly.
The grading scale is customizable within the grade function.


---


## 🔐 Password Validator – Rust Program

This is a simple Rust program that checks if the user's entered password meets common security criteria.

### ✅ Password Criteria

The password must:

Be at least 8 characters long
Contain at least 1 uppercase letter
Contain at least 1 numeric digit
Contain at least 1 special character from `!@#$%^&*()``

### 🧠 How It Works
Prompts the user to enter a password.

Checks:

Password length (pass.len() < 8)
At least one uppercase (ch.is_uppercase())
At least one digit (ch.is_numeric())
At least one special character (using contains() on a string of special characters)
Displays appropriate messages:

If the password is valid → ✅ success message

If invalid → ❌ specific instructions on what’s missing


---


## 🧾 Simple Expense Tracker 

This is a simple command-line based Expense Tracker written in Rust. It allows users to add, view, and calculate total expenses. It stores expenses as a vector of `(f64, String)` tuples, where each entry consists of an amount and a description.



### 📋 Features

- ✅ Add multiple expenses with amount and description
- 📃 View all stored expenses with index
- 💰 Show total amount of all expenses
- ❌ Exit the program


### 🛠️ Code Structure

main() : Main loop for menu and user interaction
add_expense() : Takes user input for multiple expenses
view_expense() : Lists all added expenses
total_expenses() : Calculates and displays the total amount spent


 ### Notes
 
All expense descriptions are stored as String for clarity and display
Amounts are stored as f64 to allow decimal values



---


## 🛒 Grocery List Manager (Rust CLI Project)

A simple command-line Rust application that allows users to manage a grocery list. Users can **add**, **view**, **remove items**, and **calculate the total bill** interactively from the terminal.



### 📦 Features

- ➕ Add multiple grocery items with name, quantity, and price.
- 📋 View all added items in a clean tabular format.
- 💰 Calculate and display the total bill based on quantity × price.
- ❌ Remove items by name (case-insensitive).
- 🚪 Exit the application smoothly.



### Functionality

Option 1: Enter how many items you want to add, and for each, input:

Name (string)
Quantity (integer)
Price per item (floating point)

Option 2: Shows all added grocery items in a formatted table.
Option 3: Calculates and displays the total bill.
Option 4: Removes an item by name. The name check is case-insensitive.
Option 5: Exits the program.



### 🛠️ Code Structure

struct Grocerystore: Holds item details - name, quantity, price.

main: Menu loop handling user choices.
add_items(): Adds items to the list.
view_items(): Displays the current list of products.
total_bill(): Calculates total cost.
remove_item(): Deletes an item by name.


---


## Attendance Logger

This is a simple command-line Attendance Logger program written in Rust.  
It allows you to mark attendance for students, view all marked names with their attendance status, and see the total count of present and absent students.


### Features

- Mark attendance for multiple students by entering their name and attendance status (true for present, false for absent).
- View the list of all students with their attendance status.
- View the total number of present and absent students.
- Exit the program anytime.


### Important Notes

- The program reads input from the user and expects valid input. If invalid input is given (non-integer where integer is expected, or non-boolean where boolean is expected), the program will print "Invalid number" and exit.
- Attendance should be entered as `true` or `false` (without quotes).
- The program flushes the output buffer before reading user input to ensure the menu displays immediately.



### Code Explanation

- Uses a vector `Vec<(String, bool)>` to store student names and their attendance.
- Uses Rust's `std::io` to read user input.
- The `mark_attendace` function reads student data and stores it.
- The `mark_names` function prints all recorded student attendance.
- The `t_p_a` function calculates and displays the total present and absent counts.
- Proper error handling for input parsing is implemented.


---


## 📒 Contact Book - Rust CLI App

This is a simple command-line Contact Book application written in Rust. It allows users to manage their contacts by adding, viewing, updating, deleting, sorting, and searching them based on names or emails.

### 🚀 Features

✅ Add one or more contacts (Name, Phone, Email)
🔁 Update an existing contact's phone and email
👀 View all saved contacts
❌ Delete a contact by name
🔠 Sort contacts by name or email
🔍 Search contacts using name or email
🛑 Exit the program cleanly


### 📚 Concepts Used

struct and Vec<T>
User input handling using io::stdin
Loops (loop, for)
String parsing and trimming
Vector methods: push, remove, sort_by, iter, iter_mut, position
Pattern matching with match
Basic error handling


---



## Word Extractor

### Description:

This is a simple Rust program that extracts the first, second, and third words from a given sentence using string slicing. It helps you understand how to work with &str, string slices, and bytes in Rust.

### How It Works:

The user defines a string like:
let s = String::from("This is Ahad");

The program uses three functions:

first_word(&String) -> &str
→ Returns the first word before the first space.

secong_word(&String) -> &str
→ Returns the second word (between first and second space).

third_word(&String) -> &str
→ Returns the third word (between second and third space).

These functions:

Convert the string to bytes using .as_bytes()
Use .enumerate() to loop through characters
Detect space characters (b' ')
Return appropriate word slices from the original string
If second or third word is not found, the program prints a warning.

### Output

First Word in String is This  
Second Word in String is is  
Third Word in String is Ahad  