# rust the book: study notes and code
https://doc.rust-lang.org/book/

## table of contents

- [rust the book: study notes and code](#rust-the-book:-study-notes-and-code)
  - [1: getting started](#1:-getting-started)
  - [2: guessing game](#2:-guessing-game)
  - [3: programming concepts](#3:-programming-concepts)



## 1: getting started
### 1.2: hello world
* println! is a macro, not a function; macros end with a "!".
* semicolons indicates end of line.

* compiling is done using rustc hello_world.rs, this outputs a binary executable `hello_world`.
* running the binary is done with ./hello_world.
* a binary can be run by anyone as it is compiled (to machine code) ahead of time.


### 1.3: hello cargo
* cargo handles libraries given dependencies (in `Cargo.toml`).
* packages of code are refered to as `crates`.

* `cargo build` compiles, build a binary (debug binary by default) in ./target/debug and creates a `Cargo.lock` which is not intended for manual change, but is rather Rusts way of keeping track of dependencies for the specific binary when it was compiled (Cargo.toml is for the project, Cargo.lock is for the binary). This is for development.
* `cargo build --release` optimises code during compilation and builds a binary in ./target/release. This is for production/the final version of the binary.
* `cargo run` compiles the `main.rs` code and runs the resulting binary, use `./target/release/CARGO_NAME` to run the optimised binary.
* `cargo check` quickly checks if your code compiles but does not result in a binary.

## 2: guessing game
hands on introduction by creating io-game to guess a random number; loop, matching, mutability, borrowing, String, parse, expect (Ok/Err), Ordering, using standard creates.

## 3: programming concepts