fn main() {
    let mut a: [i8; 10] = [42; 10];
    // In order to print an array, you have to provide a string literal to format with.

    // :? does a debug print of the array.
    // :#? does a pretty print of the array. 
    println!("a: {:?}", a);
    
    let mut z: i32 = 5;

    // We can combine this with standard named formatting.
    println!("a: {:#?}, {z}", a);
    
    // Or positional formatting for standard variables.
    println!("a: {:?}, {}", a, z);
    
    // Tuples work similarily to a language such as Python and can be indexed.
    let t: (i8, bool) = (7, true);
    println!("1st index: {}", t.0);
    println!("2nd index: {}", t.1);
    
    // Rust also has the concept of "struct tuples".
    struct NamedT(i8, bool);
    let t_named = NamedT(7, true);
}