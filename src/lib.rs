use miette::{Error, WrapErr};
use std::fmt::Display;
pub enum Token{
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
    Eof,
    Slash
}
impl Display for Token{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Token::LeftParen=>write!(f,"Left_Paren"),
            Token::RightParen=>write!(f,"Right_Paren"),
            Token::LeftBrace=>write!(f,"Left_Brace"),
            Token::RightBrace=>write!(f,"Right_Brace"),
            Token::Comma=>write!(f,"Comma"),
            Token::Dot=>write!(f,"Dot"),
            Token::Minus=>write!(f,"Minus"),
            Token::Plus=>write!(f,"Plus"),
            Token::Semicolon=>write!(f,"Semicolon"),
            Token::Star=>write!(f,"Star"),
            Token::Eof=>write!(f,"Eof"),
            Token::Slash=>write!(f,"Slash"),
        }
    }
}
pub struct Lexer<I>
where
    I:Iterator<Item=char>
{
    iterator:I,
    eof:bool,
}
impl<I:Iterator<Item = char>> Lexer<I>{
    pub fn new(input:impl IntoIterator<Item=char,IntoIter = I >)->Self{
        Self{
            iterator:input.into_iter(),
            eof:false,
        }
    }
}
impl<I> Iterator for Lexer<I>
where
    I:Iterator<Item=char>,
{
    type Item = Result<Token,Error>;
    fn next(&mut self)->Option<Self::Item>{
         if self.eof { return None; }
        let c=match self.iterator.next(){
            Some(c)=>c,
            None=>{
                self.eof = true;
                return Some(Ok(Token::Eof));
                }
        };
        
        match c{
            '('=>return Some(Ok(Token::LeftParen)),
            ')'=>return Some(Ok(Token::RightParen)),
            '{'=>return Some(Ok(Token::LeftBrace)),
            '}'=>return Some(Ok(Token::RightBrace)),
            '.'=>return Some(Ok(Token::Dot)),
            ','=>return Some(Ok(Token::Comma)),
            ';'=>return Some(Ok(Token::Semicolon)),
            '+'=>return Some(Ok(Token::Plus)),
            '-'=>return Some(Ok(Token::Minus)),
            '*'=>return Some(Ok(Token::Star)),
            '/'=>return Some(OK(Token::Slash)),
            _ => return Some(Err(Error::msg(format!("Unexpected char: '{}'", c)))),
        }
    }
}



// use miette::{Error, WrapErr};
// #[derive(Debug)]
// pub enum Token{
//     LeftParen,
//     RightParen,
//     Eof,
// }
// #[derive(Debug)]
// struct Lexer<I>
// where
// I:Iterator<Item = char>
// {
//     iterator:I,
//     eof:bool
// }
// impl<I:Iterator<Item=char>> Lexer<I>{
//     pub fn new(input:impl IntoIterator<Item = char, IntoIter=I>)->Self{
//         Self{
//             iterator:input.into_iter(),eof:false,
//         }
//     }
// }
// impl <I> Iterator for Lexer<I>
// where
//     I:Iterator<Item = char>
// {
//     type Item=Result<Token,Error>;
//     fn next(&mut self)->Option<Self::Item>{
//         if self.eof{return None;}
//         let c=match self.iterator.next(){
//             Some(c)=>c,
//             None=>{
//                 self.eof=true;
//                 return Some(Ok(Token::Eof));
//             }
//         };
//         match c{
//             '('=>return Some(Ok(Token::LeftParen)),
//             ')'=>return Some(Ok(Token::RightParen)),
//             _ => return Some(Err(Error::msg(format!("Unexpected char: '{}'", c)))),
//         };
//     }
// }
// fn main(){
//     //To make-- a generic lazy iterator
//     let a = "())".chars();
//     let c=Lexer::new(a);
//     for ans in c{
//         println!("{:?}",ans);
//     }
// }
