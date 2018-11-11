#[macro_use]
extern crate lazy_static;
extern crate uuid;

#[cfg(test)] mod test;
mod lexer;
mod parser;
mod token;
mod error;
mod scope;
mod tables;

fn main() {
    println!("Hello, world!");
}
