use miette::{Error, WrapErr};
use std::fmt::{Display, write};
use std::iter::Peekable;
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
    Eof,
    Slash,
    Equal,
    Less,
    LessEqual,
    Identifier(String),
    Number(i64),
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While
}
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::LeftParen => write!(f, "Left_Paren"),
            Token::RightParen => write!(f, "Right_Paren"),
            Token::LeftBrace => write!(f, "Left_Brace"),
            Token::RightBrace => write!(f, "Right_Brace"),
            Token::Comma => write!(f, "Comma"),
            Token::Dot => write!(f, "Dot"),
            Token::Minus => write!(f, "Minus"),
            Token::Plus => write!(f, "Plus"),
            Token::Semicolon => write!(f, "Semicolon"),
            Token::Star => write!(f, "Star"),
            Token::Eof => write!(f, "Eof"),
            Token::Slash => write!(f, "Slash"),
            Token::Equal => write!(f, "Equal"),
            Token::Less => write!(f, "Less"),
            Token::LessEqual => write!(f, "LessEqual"),
            Token::Identifier(s) => write!(f, "{}", s),
            Token::Number(n)=>write!(f,"{}",n),
            Token::And=>write!(f, "AND"),
            Token::Class=>write!(f, "CLASS"),
            Token::Else=>write!(f, "ELSE"),
            Token::False=>write!(f, "FALSE"),
            Token::For=>write!(f, "FOR"),
            Token::Fun=>write!(f, "FUN"),
            Token::If=>write!(f, "IF"),
            Token::Nil=>write!(f, "NIL"),
            Token::Or=>write!(f, "OR"),
            Token::Print=>write!(f, "PRINT"),
            Token::Return=>write!(f, "RETURN"),
            Token::Super=>write!(f, "SUPER"),
            Token::This=>write!(f, "THIS"),
            Token::True=>write!(f, "TRUE"),
            Token::Var=>write!(f, "VAR"),
            Token::While=>write!(f, "WHILE"),
        }
    }
}
//Graceful handling of error
#[derive(Debug)]
pub struct LexError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}
impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} at line {}, column {}",
            self.message, self.line, self.column
        )
    }
}
//Miette for Error
impl From<LexError> for Error {
    fn from(err: LexError) -> Self {
        Error::msg(err.to_string())
    }
}
//To hold the state
pub struct Lexer<I>
where
    I: Iterator<Item = char>,
{
    iterator: Peekable<I>,
    eof: bool,
    line: usize,
    column: usize,
}
impl<I: Iterator<Item = char>> Lexer<I> {
    pub fn new(input: impl IntoIterator<Item = char, IntoIter = I>) -> Self {
        Self {
            iterator: input.into_iter().peekable(), //To cast IntoIterator to Iterator
            eof: false,                             //To avoid the infinite eof
            line: 1,
            column: 0,
        }
    }
}
impl<I> Iterator for Lexer<I>
where
    I: Iterator<Item = char>,
{
    type Item = Result<Token, Error>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.eof {
            return None;
        }
        let c = match self.iterator.next() {
            Some(c) => {
                self.column += 1;
                if c == '\n' {
                    self.line += 1;
                    self.column = 0;
                }
                c
            }
            None => {
                self.eof = true;
                return Some(Ok(Token::Eof));
            }
        };

        match c {
        //*10 for converting a single digit to mutli digit number
            c if c.is_ascii_digit()=>{
                let mut val:i64=c.to_digit(10).map(|d| d as i64).unwrap_or(-1);
                while let Some(&z)=self.iterator.peek(){
                    if z.is_ascii_digit(){
                        self.iterator.next();
                        let internal_value:i64=z.to_digit(10).map(|d| d as i64).unwrap_or(-1);
                        val=val*10+internal_value;  
                    }else{
                        break;
                    }
                }
                return Some(Ok(Token::Number(val)));
            }
            c if c.is_whitespace() => self.next(),
            'a'..'z' | 'A'..'Z' => {
                let mut word = String::from(c);
                while let Some(&next_v) = self.iterator.peek() {
                    if next_v.is_alphanumeric() {
                        word.push(self.iterator.next().unwrap())
                    } else {
                        break;
                    }
                }
                let token=match word.as_str(){
                    "and"=>Token::And,
                    "class"=>Token::Class,
                    "else"=>Token::Else,
                    "false"=>Token::False,
                    "for"=>Token::For,
                    "fun"=>Token::Fun,
                    "if"=>Token::If,
                    "nil"=>Token::Nil,
                    "or"=>Token::Or,
                    "print"=>Token::Print,
                    "return"=>Token::Return,
                    "super"=>Token::Super,
                    "this"=>Token::This,
                    "true"=>Token::True,
                    "var"=>Token::Var,
                    "while"=>Token::While,
                    _=>Token::Identifier(word),
                };
                return Some(Ok(token))
            }
            
            '(' => return Some(Ok(Token::LeftParen)),
            ')' => return Some(Ok(Token::RightParen)),
            '{' => return Some(Ok(Token::LeftBrace)),
            '}' => return Some(Ok(Token::RightBrace)),
            '.' => return Some(Ok(Token::Dot)),
            ',' => return Some(Ok(Token::Comma)),
            ';' => return Some(Ok(Token::Semicolon)),
            '+' => return Some(Ok(Token::Plus)),
            '-' => return Some(Ok(Token::Minus)),
            '*' => return Some(Ok(Token::Star)),
            // '/' => return Some(Ok(Token::Slash)),
            '/' => {
                if let Some('/') = self.iterator.peek() {
                    self.iterator.next();
                    while let Some(&c) = self.iterator.peek() {
                        if c == '\n' {
                            break;
                        }
                        self.iterator.next();
                    }
                    self.next()
                } else {
                    return Some(Ok(Token::Slash));
                }
            }
            //"Sarvil Hello"-> String "Sarvil Hello" Consume untill next "
            // '"'=>return Some(Ok(Token::Identifier(String::from("found something")))),
            '"'=>{
              let mut word=String::new();
              word.push('"');
              while let Some(&c)=self.iterator.peek(){
                  if c=='"'{
                      word.push('"');
                      break;
                  }
                  word.push(c);
                  self.iterator.next();
              }
              self.iterator.next();
              return Some(Ok(Token::Identifier(word)))
            }
            '=' => return Some(Ok(Token::Equal)),
            '<' => {
                if let Some('=') = self.iterator.peek() {
                    self.iterator.next();
                    return Some(Ok(Token::LessEqual));
                } else {
                    return Some(Ok(Token::Less));
                }
            }
            _ => {
                return Some(Err(LexError {
                    message: format!("Unexpected character '{}'", c),
                    line: self.line,
                    column: self.column,
                }
                .into()));
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    //Checking for string
    fn test_identifier() {
        let mut lexer = Lexer::new("hello".chars());
        let token = lexer.next().unwrap().unwrap();
        match token {
            Token::Identifier(s) => assert_eq!(s, "hello"),
            _ => panic!("Expected identifer"),
        }
    }
    #[test]
    //Checking for comments
    fn test_comment_skipped() {
        let mut lexer = Lexer::new("// comment\n;".chars());
        let token = lexer.next().unwrap().unwrap();
        assert!(matches!(token, Token::Semicolon));
    }
    #[test]
    //Checking for total cutoff of comments and EOF ommition
    fn test_eof() {
        let mut lexer = Lexer::new("// Comment".chars());
        let token = lexer.next().unwrap().unwrap();
        assert!(matches!(token, Token::Eof))
    }
}
