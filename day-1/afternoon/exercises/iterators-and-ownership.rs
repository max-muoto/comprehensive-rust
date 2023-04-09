fn main() {
    let v: Vec<String> = vec![String::from("foo"), String::from("bar")];

    // After looping, vector can still be utilized.
    for word in &v {
        println!("word: {word}");
    }

    // Works
    println!("{:#?}", v);

    // We take ownership of each element, stopping us from using the vector anymore.
    for word in v {
        println!("word: {word}");
    }

    // Won't work.
    // println!("{:#?}", v);
}
