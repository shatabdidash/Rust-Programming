fn reverse_string(input: &str) -> String {
    let mut chars: Vec<char> = input.chars().collect(); // Convert the string to a vector of characters
    chars.reverse(); // Reverse the vector
    chars.into_iter().collect() // Convert the vector back to a string
}

fn main() {
    let input = "Hello, world!";
    let reversed = reverse_string(input);
    println!("Original: {}", input);
    println!("Reversed: {}", reversed);
}
