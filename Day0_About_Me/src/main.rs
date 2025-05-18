/*
Command to create a new project
rust new <filename>

Command to compile files in the source folder:
cargo rustc <filename>
cargo rustc

Command to build the whole project:
cargo build
cargo build --release

Command to run in the source folder:
cargo run

Command to check the build
cargo check

Command to format the files
rustfmt <filename>
cargo fmt <filename>

Rust has snake_case_convention for functions.
 */

fn main() {
    println!("Hello, world!");
    println!("I am Maru.");
    println!("Nice to meet you.");

    print_lines();
}

// prints everything in one line. So there is no linebreak.
fn print_lines() {
    print!("Hello, world!");
    print!("I am Maru.");
    print!("Nice to meet you.");
}

