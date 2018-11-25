/// Result wrapper around an `Error`
pub type Result<T> = std::result::Result<T, Error>;

/// A collection of all the possible errors that can occur during the parsing and evaluation of the language.
#[derive(PartialEq, Debug)]
pub enum Error {
    /// Issued when the user inputs an symbol not understood by the parser
    InvalidToken,
    /// Issued when supplying an argument of the wrong type, like a string where a boolean was expected.
    ArgumentMismatch, 
    /// Issued when an identifier can't be found in the symbol table.
    UnknownIdentifier,
    /// Issued when too few or too many arguments are supplied to a function.
    /// Too many would only occur by mistake within the parser.
    ArityMismatch,
    /// Issued when the user attempts to perform operations on an empty stack.
    EmptyStack,
}