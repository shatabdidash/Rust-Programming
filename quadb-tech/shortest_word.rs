fn shortest_word(s: &str) -> Option<&str> {
    s.split_whitespace().min_by_key(|word| word.len())
}

fn main() {
    let sentence = "This is test string with some short words.";
    match shortest_word(sentence) {
        Some(shortest) => println!("The shortest word is: {}", shortest),
        None => println!("The string is empty."),
    }
}
