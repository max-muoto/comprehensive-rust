// For a given struct, we create an implementation with methods.

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Don't have ability to modify attributes.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // With mutable self we can modify struct attributes.
    fn inc_width(&mut self, delta: u32) {
        self.width += delta;
    }
}

// Rust has support for Generics.
fn pick_one<T>(a: T, b: T) -> T {
    if std::process::id() % 2 == 0 { a } else { b }
}


fn main() {
    let mut rect = Rectangle { width: 10, height: 5 };
    println!("old area: {}", rect.area());
    rect.inc_width(5);
    println!("new area: {}", rect.area());

    // Generic types can be inferred with specifying them.
    println!("coin toss: {}", pick_one("heads", "tails"));
    println!("cash prize: {}", pick_one(500, 1000));
}

