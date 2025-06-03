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

