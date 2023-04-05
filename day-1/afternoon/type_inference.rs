fn takes_u32(x: u32) {
    println!("u32: {x}");
}

fn takes_i8(y: i8) {
    println!("i8: {y}");
}

fn main() {
    let x = 10;
    let y = 20;

    // The compiler is able to infer the types for these two vars
    // based on the arg types of the functions they're used in.
    takes_u32(x);
    takes_i8(y);

    // However this won't work since there are two possible types it could take on.
    // Even if we use into() it won't work since we can't convert i8 into u32.
    // takes_u32(y);
}