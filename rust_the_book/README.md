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
notes are in corresponding interactive rust notebook (using the `excvr` kernel)

## 4: ownership
"All programs have to manage the way they use a computer’s memory while running. Some languages have garbage collection that regularly looks for no-longer-used memory as the program runs; in other languages, the programmer must explicitly allocate and free the memory. Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks."

### stack and heap
**the stack**
* LIFO (last in first out)
* data stored on stack must have a known, fixed size such that the compiler can know how to manage memory
* fast

**the heap**
when something is put on the stack the memory allocator finds an empty spot that is big enough, marks it as being in use and returns a `pointer` that refers to that spot in the memory, aka allocation on the heap.
* slower due to the allocation process

### ownership rules
1. `Each value in Rust has an owner`.
2. `There can only be one owner at a time`.
3. `When the owner goes out of scope, the value will be dropped`.