fn main() {
    let mut a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");

    let s: &mut [i32] = &mut a[2..4];
    println!("s: {s:?}");
    mutable_array()
}

fn mutable_array() {
    let mut a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {:?}", a);

    let s: &mut [i32] = &mut a[2..4];
    println!("s: {:?}", s);

    // Modify the elements within the slice using the mutable reference.
    s[0] = 35;
    s[1] = 45;

    // The array a should now be modified
    println!("Modified a: {:?}", a);
    
   // However, we can't modify indices 2-4 due to Rust ownership constraints. 
}