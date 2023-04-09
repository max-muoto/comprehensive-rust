fn main() {
    let input = 'x';

    // With the match keyword, we can match against 1+ patterns.
    // _ is a any value wildcare.
    match input {
        'q' => println!("Quitting"),
        'a' | 's' | 'w' | 'd' => println!("Moving around"),
        '0'..='9' => println!("Number input"), // Inclusive range.
        _ => println!("Something else"),
    }
}
