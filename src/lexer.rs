use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    LParen,
    RParen,
    Symbol,
    Int,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    kind: TokenKind,
    val: String,
}

impl Token {
    pub fn new(kind: TokenKind, val: String) -> Token {
        Token{
            kind: kind,
            val: val,
        }
    }
}

#[derive(Debug)]
pub struct Lexer<'a> {
    src: Peekable<Chars<'a>>,
    token_stream: Vec<Token>
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Lexer<'a> {
        Lexer{
            src: src.chars().peekable(),
            token_stream: vec![]
        }
    }

    pub fn lex(&mut self) -> Result<&Vec<Token>, String> {
        while let Some(c) = self.src.next() {
            match c {
                '(' => self.token_stream.push(Token::new(TokenKind::LParen, "(".to_string())),
                ')' => self.token_stream.push(Token::new(TokenKind::RParen, ")".to_string())),
                n if c.is_numeric() => {
                    let mut num = String::new();
                    num.push(n);
                    while let Some(&i) = self.src.peek() {
                        if i.is_numeric() {
                            num.push(i);
                            self.src.next();
                        }
                        else{
                            break;
                        }
                    }
                    self.token_stream.push(Token::new(TokenKind::Int, num))
                },
                _w if c.is_whitespace() => continue,
                s if c.is_ascii() => {
                    let mut symbol = String::new();
                    symbol.push(s);
                    while let Some(&w) = self.src.peek() {
                        if w.is_whitespace() {
                            break;
                        }
                        else {
                            symbol.push(w);
                            self.src.next();
                        }
                    }
                    self.token_stream.push(Token::new(TokenKind::Symbol, symbol))
                },
                _ => return Err("invalid character detected.".to_string())
            };
        }

        Ok(&self.token_stream)
    }
}
