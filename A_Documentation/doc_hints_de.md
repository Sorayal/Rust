# Hinweise für Rust

## Rust Rover
### Anlegen von Projekten
Im Verzeichnis Rechtsklick > New > Cargo Crate

### Terminal Kommandos

__Kommando, um neues Projekt zu erstellen:__

```Bash
rust new <filename>
```

__Kommando zum Kompilieren von Dateien im Source Ordner:__

```Bash
cargo rustc <filename>
cargo rustc
```

__Kommando zum Bauen des gesamten Projekts:__

```Bash
cargo build
cargo build --release
```

__Kommando zum Laufen des Projekts im Source Ordner:__

```Bash
cargo run
```

__Kommando zum statischen Prüfen des Codes ohne Kompilierung:__

```Bash
cargo check
```

__Kommandos zum Formatieren der Dateien:__

```Bash
rustfmt <filename>
cargo fmt <filename>
```

### Code Conventions

Rust hat eine Snake Case Konvention für Funktionen.

### Links
[Hello Example](https://doc.rust-lang.org/rust-by-example/hello.html)

[Rustlings](https://github.com/rust-lang/rustlings/)

[Rust Learn](https://www.rust-lang.org/learn)

[Rust_Learn_Paskhaver](https://github.com/paskhaver/learn-to-code-with-rust)
