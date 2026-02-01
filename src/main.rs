//! A "Hello, world!" program.
//! More comments.

/// The entrypoint of the program.
fn main() {
    print("Hello, world!");
    print("Goodbye, world!");
}

/// A function that prints a message.
fn print(m: &str) {
    println!("{m}");
}
