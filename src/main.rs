use std::io;
use std::fs;

#[macro_use]
extern crate lazy_static;
extern crate uuid;
#[macro_use]
extern crate clap;
extern crate regex;

#[cfg(test)] mod test;
mod lexer;
mod parser;
mod token;
mod error;
mod scope;
mod tables;
mod callback;

fn read_file(matches: clap::ArgMatches) -> io::Result<String> {
    use io::{self, ErrorKind, Read};
    let filename = matches
        .value_of("INPUT")
        .ok_or(io::Error::new(ErrorKind::NotFound, "Could not find the file"))?;
    let mut file = fs::File::open(filename)?;
    let mut contents = String::new();
    
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() -> io::Result<()> {
    let mut scopes = tables::ScopeTable::new();
    let mut main_scope = scope::Scope::new(&mut scopes, None);
    
    let yaml = load_yaml!("cli.yaml");
    let matches = clap::App::from_yaml(yaml).get_matches();
    let file = read_file(matches)?;

    println!("Contents of the file: {}", file);

    Ok(())
}
