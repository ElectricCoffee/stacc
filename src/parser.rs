use token::{Stack, Token};
use lexer;
use error::{self, Error};

pub fn parse_nary(stack: &mut Stack, symbol: &str) -> error::Result<()> {
    // get the callback stored in BIFS, if available
    let callback = lexer::BIFS.get(symbol).ok_or(Error::UnknownIdentifier)?;

    let mut args = Vec::new();

    // if the arity is greater than the available data, error
    if stack.len() < callback.arity {
        return Err(Error::ArityMismatch);
    }

    // add the required number of tokens to the args vector
    for _ in 0 .. callback.arity {
        let token = stack.pop_back().unwrap();
        args.push(token);
    };

    // call the inner function
    let fun = callback.func;
    let result = fun(&args)?;

    // if the result isn't a void, add the result to the stack
    if result != Token::Void {
        stack.push_back(result);
    }

    Ok(())
}