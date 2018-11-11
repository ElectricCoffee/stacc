#[macro_use]
extern crate lazy_static;

#[cfg(test)] mod test;
mod lexer;
mod parser;
mod token;
mod error;

fn main() {
    println!("Hello, world!");
}
