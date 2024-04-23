fn longest_common_prefix(strings: &[&str]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    let mut prefix = String::new();
    let first_str = strings[0];

    'outer: for (i, c) in first_str.char_indices() {
        for s in &strings[1..] {
            if let Some(sc) = s.chars().nth(i) {
                if sc != c {
                    break 'outer;
                }
            } else {
                break 'outer;
            }
        }
        prefix.push(c);
    }

    prefix
}

fn main() {
    let strings1 = ["flower", "flow", "flight"];
    let strings2 = ["dog", "racecar", "car"];

    println!("Longest common prefix of {:?}: {}", strings1, longest_common_prefix(&strings1));
    println!("Longest common prefix of {:?}: {}", strings2, longest_common_prefix(&strings2));
}
