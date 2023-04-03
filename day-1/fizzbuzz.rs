fn main() {
    fizzbuzz_to(20);   // Defined below, no forward declaration needed
}

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;  // Corner case, early return
    }
    lhs % rhs == 0     // The last expression in a block is the return value
}

fn fizzbuzz(n: u32) -> () {  // No return value means returning the unit type `()`
    // Case-match based on these two function return values.
    match (is_divisible_by(n, 3), is_divisible_by(n, 5)) {
        (true,  true)  => println!("fizzbuzz"),
        (true,  false) => println!("fizz"),
        (false, true)  => println!("buzz"),
        (false, false) => println!("{n}"),
    }
}

/// FizzBuzz prints for numbers 1 to n.
/// This serves as basically a function docstring.
/// However, contents are treated as markdown.
/// All Rust library crates are automatically documented using this pattern.
fn fizzbuzz_to(n: u32) {  // `-> ()` is normally omitted
    for i in 1..=n {
        fizzbuzz(i);
    }
}