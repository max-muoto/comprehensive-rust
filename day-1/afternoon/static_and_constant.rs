// We can declare compile-time constants as such.
// These get in-lined into the program upon compilation.
// See: https://rust-lang.github.io/rfcs/0246-const-vs-static.html

const DIGEST_SIZE: usize = 3;
const ZERO: Option<u8> = Some(42);

// We also have static variables which are not inlined upon compilation.
static BANNER: &str = "Welcome to RustOS 3.14";


fn compute_digest(text: &str) -> [u8; DIGEST_SIZE] {
    let mut digest = [ZERO.unwrap_or(0); DIGEST_SIZE];
    for (idx, &b) in text.as_bytes().iter().enumerate() {
        digest[idx % DIGEST_SIZE] = digest[idx % DIGEST_SIZE].wrapping_add(b);
    }
    digest
}

fn main() {
    let digest = compute_digest("Hello");
    println!("Digest: {digest:?}");
}