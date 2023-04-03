fn multiply(x: i16, y: i16) -> i16 {
    x * y
}

fn main() {
    // Smaller bit ints to larger conversion.
    let x: i8 = 15;
    let y: i16 = 1000;

    println!("{x} * {y} = {}", multiply(x.into(), y));
    
    
    // There only exist From<T> implementations for smaller int-sizes to larger int-sizes (e.g. i8 -> i16 or i8 -> i32)
    // As opposed to larger int-sizes to smaller ones. For example, there is no From<i32> implementation for i16.
    
    // Bools can implicity convert into the various int types however.
    let l: bool = true;
    println!("{l} * {y} = {}", multiply(l.into(), y));
}