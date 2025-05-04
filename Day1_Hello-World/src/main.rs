/**
This is a multiline doc comment in Rust.
This is my very first programme in Rust.

https://doc.rust-lang.org/reference/comments.html
 */

fn main() {
    println!("Hello, world!");

    // funktion aus externen Paket ferris-says
    ferris_says::say(
        // Die schreibende Zeichenkette
        "Hallo Rust",
        // Die Breite der Ausgabe in Spalten
        20,
        // Die Standardausgabe bereitstellen
        std::io::stdout(),
    ).expect("failed to load module");
}




