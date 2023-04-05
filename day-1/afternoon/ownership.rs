fn main() {
    let s1: String = String::from("Hello!");
    // Assignment transfers ownership between variables.
    // Opposite of C++ defaults, which copies unless you use std::move.
    // In Rust, clones are explicit with clone.
    let s2: String = s1;
    println!("s2: {s2}");
    // println!("s1: {s1}");
}