// use miette::{Error, WrapErr};

// pub enum Token {
//     LeftParen,
//     RightParen,
//     LeftBrace,
//     RightBrace,
//     Comma,
//     Dot,
//     Minus,
//     Plus,
//     Semicolon,
//     Star,
// }
// impl std::fmt::Display for Token{
//   fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{
//     write!(
//       f,
//       "{}",
//       match self{
//         Token::LeftParen=>"LEFT_PAREN",
//         Token::RightParen=>"RIGHT_PAREN",
//         Token::LeftBrace=>"LEFT_BRACE",
//         Token::RightBrace=>"RIGHT_BRACE",
//         Token::Comma=>"COMMA",
//         Token::Dot=>"DOT",
//         Token::Minus=>"MINUS",
//         Token::Plus=>"PLUS",
//         Token::Semicolon=>"SEMICOLON",
//         Token::Star=>"STAR",
//       }
//     )
//   }
// }
// pub struct Lexer<I> {
//     iterator: I,
// }
// //shell
// impl<I> Lexer<I> {
//     pub fn new(input: impl IntoIterator<Item = char, IntoIter = I>) -> Self {
//         Self {
//             iterator: input.into_iter(),
//         }
//     }
// }
// impl<I> Iterator for Lexer<I>
// where
//     I: Iterator<Item = char>,
// {
//     type Item = Result<Token, Error>;
//     fn next(&mut self) -> Option<Self::Item> {
//         let c = self.iterator.next()?;
//         match c {
//             '(' => return Some(Token::LeftParen),
//             ')' => return Some(Token::RightParen),
//             '{' => return Some(Token::LeftBrace),
//             '}' => return Some(Token::RightBrace),
//             ',' => return Some(Token::Comma),
//             '.' => return Some(Token::Dot),
//             '-' => return Some(Token::Minus),
//             '+' => return Some(Token::Plus),
//             ';' => return Some(Token::Semicolon),
//             '*' => return Some(Token::Star),
//         }
//     }
// }
// // fn lex(input:impl IntoIterator<Item = char>)->impl Iterator<Item = Result<Token, Error>>{
// //   let mut lex_state=lexer::new(input);
// // }
use std::iter::Peekable;
use std::str::Chars;
#[derive(Debug)]
pub enum Token {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Star,
}
struct Lexer<'a>{
  iter:Peekable<Chars<'a>>
}
impl<'a> Lexer<'a>{
  fn new(input:&'a str)->Self{
    Self{
      iter:input.chars().peekable(),
    }
  }
}
