use crate::lexer::tokens::{Token, TokenKind};
use crate::lexer::tokens::TokenKind::*;

pub mod tokens;

#[derive(Debug)]
pub struct MonkeyCLexer {
    source: Vec<char>,
    currently_at: usize,
}

impl MonkeyCLexer {
    pub fn new(source: Vec<char>) -> Self {
        Self {
            source,
            currently_at: 0,
        }
    }

    pub fn lex(&mut self) {
        let mut tokens: Vec<Token> = Vec::new();
        while self.source.len() > self.currently_at {
            let c = self.current_char();

            match c {
                '{' => {
                    tokens.push(Token::new(OpeningBracket, c.to_string()));
                    self.currently_at += 1;
                }
                '}' => {
                    tokens.push(Token::new(ClosingBracket, c.to_string()));
                    self.currently_at += 1;
                }
                '(' => {
                    tokens.push(Token::new(OpeningBrace, c.to_string()));
                    self.currently_at += 1;
                }
                ')' => {
                    tokens.push(Token::new(ClosingBrace, c.to_string()));
                    self.currently_at += 1;
                }
                '~' => {
                    tokens.push(Token::new(Tilde, c.to_string()));
                    self.currently_at += 1;
                }
                '=' => {
                    tokens.push(Token::new(TokenKind::Assign, c.to_string()));
                    self.currently_at += 1;
                }
                ';' => {
                    tokens.push(Token::new(TokenKind::Semicolon, c.to_string()));
                    self.currently_at += 1;
                }
                '!' => {
                    tokens.push(Token::new(TokenKind::Bang, c.to_string()));
                    self.currently_at += 1;
                }
                _ if c.is_alphabetic() => {
                    // Creating a buffer and writing a first character to it
                    let mut buffer = String::new();
                    buffer.push(c);
                    self.currently_at += 1;

                    // Writing everything that is alphabetic to the buffer
                    while self.current_char().is_alphabetic() {
                        buffer.push(self.current_char());
                        self.currently_at += 1;
                    }

                    // Then matching for keywords. If is not a keyword, then it's an identifier
                    let kind: TokenKind = match buffer.as_str() {
                        "as" => As,
                        "break" => Break,
                        "case" => Case,
                        "catch" => Catch,
                        "class" => Class,
                        "const" => Const,
                        "continue" => Continue,
                        "default" => Default,
                        "do" => Do,
                        "else" => Else,
                        "enum" => Enum,
                        "extends" => Extends,
                        "finally" => Finally,
                        "for" => For,
                        "function" => Function,
                        "has" => Has,
                        "hidden" => Hidden,
                        "if" => If,
                        "instanceof" => InstanceOf,
                        "import" => Import,
                        "me" => Me,
                        "module" => Module,
                        "new" => New,
                        "private" => Private,
                        "protected" => Protected,
                        "public" => Public,
                        "return" => Return,
                        "self" => SelfK,
                        "static" => Static,
                        "switch" => Switch,
                        "throw" => Throw,
                        "try" => Try,
                        "using" => Using,
                        "var" => Var,
                        "while" => While,
                        _ => Identifier,
                    };
                    tokens.push(Token::new(kind, buffer))
                }
                '\'' => {
                    self.currently_at += 1;
                    tokens.push(Token::new(TokenKind::Char, self.current_char().to_string()));
                    self.currently_at += 2;
                }
                '\"' => {
                    self.currently_at += 1;
                    let mut buffer: String = String::new();
                    while self.current_char() != '\"' {
                        buffer.push(self.current_char());
                        self.currently_at += 1;
                    }
                    tokens.push(Token::new(TokenKind::String, buffer));
                    self.currently_at += 1;
                }
                _ => {
                    self.currently_at += 1;
                }
            }
        }
        println!("{:?}", tokens)
    }

    fn current_char(&self) -> char {
        *self.source.get(self.currently_at).unwrap()
    }
}
