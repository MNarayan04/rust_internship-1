use std::io;

fn reverse_string(input: &str) -> String {
    input.chars().rev().collect()
}

fn main() {
    // Read the string from the user
    println!("Enter a string:");
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).expect("Failed to read line");

    // Remove trailing newline character
    input_string = input_string.trim().to_string();

    // Reverse the string
    let reversed_string = reverse_string(&input_string);

    // Print the result
    println!("Original string: {}", input_string);
    println!("Reversed string: {}", reversed_string);
}
