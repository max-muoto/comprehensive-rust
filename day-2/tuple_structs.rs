struct Point(i32, i32);

fn main() {
    // For when field names are unimporatnt, we can use a tuple struct.
    let p = Point(17, 23);
    println!("({}, {})", p.0, p.1);
}
