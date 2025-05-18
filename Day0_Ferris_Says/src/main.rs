
// Hauptmethode, notwendig zum Aufruf.
fn main() {
    // Rust nutzt ; als Terminatoren.
    // println! ist ein Makro, eine vordefinierte Funktion.
    println!("Hello, world!");
    println!("Test second line");
    println!("1. Line\n2. Line\n3. Line");
    ferris_says::say(
        // Die zu schreibende Zeichenkette
        "Hallo, Rust",
        // Die Breite der Ausgabe in Spalten
        20,
        // Die Standardausgabe bereitstellen
        std::io::stdout(),
    )
    .expect("ferris says failed");
}
