enum Result {
    Ok(i32),
    Err(String),
}

struct Foo {
    x: (u32, u32),
    y: u32,
}

fn divide_in_two(n: i32) -> Result {
    if n % 2 == 0 {
        return Result::Ok(n / 2);
    } else {
        return Result::Err(format!("cannot divide {n} into two equal parts"));
    }
}

fn main() {
    let n = 100;
    // We can see the values that we set in the returned enum.
    match divide_in_two(n) {
        Result::Ok(half) => println!("{n} divided in two is {half}"),
        Result::Err(msg) => println!("sorry, an error happened: {msg}"),
    }

    // We can also do the same for structs.
    let foo = Foo { x: (1, 2), y: 3 };
    match foo {
        Foo { x: (1, b), y } => println!("x.0 = 1, b = {b}, y = {y}"),
        Foo { y: 2, x: i } => println!("y = 2, x = {i:?}"),
        Foo { y, .. } => println!("y = {y}, other fields were ignored"),
    }

    // And for arrays.
    let triple = [0, -2, 3];
    println!("Tell me about {triple:?}");
    match triple {
        [0, y, z] => println!("First is 0, y = {y}, and z = {z}"),
        [1, ..] => println!("First is 1 and the rest were ignored"),
        _ => println!("All elements were ignored"),
    }
}
