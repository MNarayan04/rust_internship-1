use std::io;

fn find_shortest_word(sentence: &str) -> Option<&str> {
    let mut shortest_word: Option<&str> = None;
    let mut shortest_length = usize::max_value();

    for word in sentence.split_whitespace() {
        let word_length = word.len();
        if word_length < shortest_length {
            shortest_length = word_length;
            shortest_word = Some(word);
        }
    }

    shortest_word
}

fn main() {
    // Read the string from the user
    println!("Enter a string of words:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Find the shortest word in the string
    if let Some(shortest_word) = find_shortest_word(&input) {
        println!("The shortest word is '{}'", shortest_word);
    } else {
        println!("No words found in the string.");
    }
}
