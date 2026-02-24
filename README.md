# WhitePaper

  This is the implementation of interpreter for Lox language in Rust
  
  https://craftinginterpreters.com/introduction.html
  https://craftinginterpreters.com/a-map-of-the-territory.html
  https://craftinginterpreters.com/the-lox-language.html


# Lib.rs--**Lexer**
> A lexer (lexical analyzer or scanner) is a computer program or component that breaks raw input code (a stream of characters) into meaningful, categorized units called tokens
**Token**: https://craftinginterpreters.com/scanning.html#recognizing-lexemes
> To perform Lazy iterator I decided to use Iterator trait
`rust 
  pub struct Lexer<I>
where
    I:Iterator<Item=char>
{
    iterator:I,
    eof:bool,
}
`

