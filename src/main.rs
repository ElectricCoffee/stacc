use std::io;

#[macro_use]
extern crate lazy_static;
extern crate uuid;
#[macro_use]
extern crate clap;

#[cfg(test)] mod test;
mod lexer;
mod parser;
mod token;
mod error;
mod scope;
mod tables;

fn main() -> io::Result<()> {
    let mut scopes = tables::ScopeTable::new();
    let mut main_scope = scope::Scope::new(&mut scopes, None);
    
    let yaml = load_yaml!("cli.yaml");
    let matches = clap::App::from_yaml(yaml).get_matches();

    println!("Hello, world!");

    Ok(())
}
