use std::iter::Peekable;

use crate::{LexError, lex::Token};
pub struct Parser<I>
where
    I:Iterator<Item = Result<Token,LexError>>,
{
    tokens:Peekable<I>,
}
impl<I> Parser<I>
where 
    I:Iterator<Item=Result<Token,LexError>>,
{
    pub fn new(input:I)->Self{
        Self{
            tokens:input.peekable(),
        }
    }
}