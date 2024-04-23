use std::io;

fn is_palindrome(input: &str) -> bool {
    let input = input.to_lowercase(); // Convert to lowercase for case-insensitive comparison
    let reversed = input.chars().rev().collect::<String>(); // Reverse the string

    input == reversed // Check if the original and reversed strings are equal
}

fn main() {
    println!("Enter a string:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let input = input.trim(); // Remove trailing newline character

    println!("Is '{}' a palindrome? {}", input, is_palindrome(input));
}
