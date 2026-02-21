use std::io;

/**
Project for the number guessing game
(WIP)
1. Choosing a number between 1 and 100
2. Pseudo random generator produces a number
3. Check if right or wrong, give a hint
4. Check again 3 tries until the game is over.
 */
fn main() {
    // mutable variable, String without predefined capacity
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!("Hello, world!");
}
