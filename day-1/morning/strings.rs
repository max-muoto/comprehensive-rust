fn main() {
    // Immutable string slice.
    // Rough C++ equivalent: const char*
    // Can only contain UTF-8 encoded bytes.
    let s1: &str = "World";
    println!("s1: {s1}");

    // Rust's string type is simply a wrapper a vector of bytes Vec<T>.
    // Rough C++ equivalent: std::string
    // String::from() creates a String from a string literal.
    let mut s2: String = String::from("Hello ");
    println!("s2: {s2}");
    s2.push_str(s1);
    println!("s2: {s2}");
    
    // We can borrow string slices from String.
    let s3: &str = &s2[6..];
    println!("s3: {s3}");
}