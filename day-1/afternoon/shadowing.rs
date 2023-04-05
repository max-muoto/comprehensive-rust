fn main() {
    let a = 10;
    println!("before: {a}");

    // We can shadow variables from the same/outer scope.
    // A shadowing variable can have a different type.
    // Convenient for holding on to values after .unwrap().
    {
        println!("original scope: {a}");

        let a = "hello";
        println!("inner scope: {a}");

        let a = true;
        println!("shadowed in inner scope: {a}");
    }

    println!("after: {a}");
}