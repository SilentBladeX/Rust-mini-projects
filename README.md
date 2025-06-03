# Rust-mini-projects
A collection of beginner-friendly Rust mini projects like temperature converter, Fibonacci series, and more — built for practice and learning.


# 🌡️ Temperature Converter in Rust

This is a simple command-line Rust program that allows users to convert temperatures between **Celsius** and **Fahrenheit**.

---

## 🚀 Features

- Convert **Celsius to Fahrenheit**
- Convert **Fahrenheit to Celsius**
- Input handling and error messages
- Clean output formatting with two decimal places

---

## 🛠️ How to Run

Make sure you have **Rust and Cargo** installed.

1. Clone this repository or copy the `temp_converter.rs` file.
2. Create a new Cargo project (if not already):

```bash
cargo new Rust-mini-projects
cd Rust-mini-projects


cargo run

## Output

Choose an option:
1. Convert from Celsius to Fahrenheit
2. Convert from Fahrenheit to Celsius
> 1
Enter temperature in Celsius:
> 100
Temperature in Fahrenheit: 212.00°F






## Fibonacci Series Generator in Rust

This is a simple Rust program that takes a number from the user and prints the Fibonacci series up to that number of terms.

## 📋 Features

- Takes input from the user at runtime
- Validates user input
- Generates Fibonacci series using a `while` loop
- Clean and readable Rust code

## Output

Enter num to generate fibonacci series
5
0
1
1
2
3




# 🎄 The Twelve Days of Christmas (Rust)

This Rust program prints the lyrics of the classic Christmas carol **“The Twelve Days of Christmas”**, using a loop and array to elegantly handle repetition and structure.

## ✨ Features

- Uses nested loops to avoid redundant printing
- Dynamically adds ordinal suffixes like `1st`, `2nd`, `3rd`, `4th`, etc.
- Uses a fixed array of gifts to generate lyrics efficiently
- Demonstrates string formatting and loop control in Rust


## output

On the 1st day of Christmas, my true love sent to me
"A Partridge in a Pear Tree"

On the 2nd day of Christmas, my true love sent to me
"Two Turtle Doves"
AND, "A Partridge in a Pear Tree"

On the 3rd day of Christmas, my true love sent to me
"Three French Hens"
"Two Turtle Doves"
AND, "A Partridge in a Pear Tree"

... and so on up to 12 days

