
/**
Maskottchen für Rust
Krabbe Ferris
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
    ).expect("failed to load module for ferris_says");
}
