fn main() {
    // Variables are immutable by default.
    // i32 is optional due to type inference.
    let x: i32 = 10;
    println!("x: {x}");

    // We are unable to modify x here.

    // x = 20;
    // println!("x: {x}");
}