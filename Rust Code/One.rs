use std::io;

fn is_palindrome(s: &str) -> bool {
    let mut chars = s.chars();
    let mut left = 0;
    let mut right = s.len() - 1;

    while left < right {
        let left_char = match chars.nth(left) {
            Some(c) => c,
            None => break,
        };
        let right_char = match chars.nth_back(0) {
            Some(c) => c,
            None => break,
        }; 
        
        
        if left_char != right_char {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}

fn main() {
    println!("Enter a string:");
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).expect("Failed to read line");

   
   
    input_string = input_string.trim().to_string();

    if is_palindrome(&input_string) {
        println!("'{}' is a palindrome.", input_string);
    } else {
        println!("'{}' is not a palindrome.", input_string);
    }
}
