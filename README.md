# DSA practice in rust

Contains solutions to DSA problems in rust.

## How to run challenges


**Write a solution to  a challenge as a module**

1. Write your solution as a module in `src/challenge_name.rs` inside a `pub fn run()` function.
2. Add the module to `src/main.rs` by adding `mod <challenge_name>;` at the top and `<challenge_name>::run(); in the match expression.
3. Run the code using `cargo run challenge_name`

Our modules need to have a struct that implements the `Solution` trait which have a method name of your choice. The signature must respect the challenge signature. The `run` function of the module will call this method.

**Help**
- To check available challenge: `cargo run list-challenges`
- To make sure you setup the module ok run `cargo build && ./target/debug/dsa_rust help` or simply `cargo run help`
