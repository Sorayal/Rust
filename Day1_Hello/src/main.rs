

// main function.
/*
Multiline comment:
println! is a macro.
 */
fn main() {
    println!("Hello, world!");
    helper();
}

// helper function
fn helper() -> &'static str {
    return "help!";
}
