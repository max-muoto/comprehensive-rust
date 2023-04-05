### Ownership

- When you pass a value to a function, this transfers ownership.

### Copying/Cloning

- Certain Rust types will copy by default.
- For example `i32`.
- You can opt your own structs to use copy semantics by defaut.

```rust
#[derive(Copy, Clone, Debug)]
struct Point(i32, i32);

fn main() {
    let p1 = Point(3, 4);
    let p2 = p1;
    println!("p1: {p1:?}");
    println!("p2: {p2:?}");
}
```

- Use references to avoid ownership being transfered.
