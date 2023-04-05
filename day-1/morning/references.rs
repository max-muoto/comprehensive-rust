fn main() {
    // Like C++ Rust has references.
    let mut x: i32 = 10;
    // In general, it's a better practice to use references as opposed to raw pointers.
    let ref_x: &mut i32 = &mut x;
    // We can dereference this pointer to directly assign its value.
    *ref_x = 20;
    println!("x: {x}");
}

fn dangling_references() {
    let ref_x: &i32;
    {
        // This is forbidden because the lifetime of x only lives for this block, while ref_x has a greater scope.
        let x: i32 = 10;
        ref_x = &x;
    }
    println!("ref_x: {ref_x}");
}