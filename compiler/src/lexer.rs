use std::process::exit;

use anyhow::{bail, Context, Result};

use crate::token::{Keyword, Token};

pub(crate) struct Lexer {
    src_code: Vec<u8>,
    current_index: usize,
    current: u8,
}

impl Lexer {
    pub fn new(src_code: &str) -> Self {
        let src_code = src_code.to_owned().into_bytes();
        let first_char = src_code[0];

        Self { src_code, current_index: 0, current: first_char }
    }
    pub fn advance(&mut self) {
        self.current_index += 1;
        if self.current_index >= self.src_code.len() {
            self.current = b'\0';
        } else {
            self.current = self.src_code[self.current_index];
        }
    }

    fn expect(&self, expected: u8) {
        if self.current != expected {
            eprintln!("Unknown token '{}' at index {}, expected '{}'", self.current, self.current_index, expected);
            exit(1);
        }
    }

    pub fn next_token(&mut self) -> Result<Option<Token>> {
        while matches!(self.current,  b' ' | b'\t') {
            self.advance();
        }

        let token = match self.current {
            b'/' => {
                self.parse_comment();
                return self.next_token();
            }
            b'\r' => {
                self.advance();
                self.expect(b'\n');
                self.advance();
                Token::Newline
            }
            b'=' => {
                self.advance();
                if self.current == b'=' {
                    self.advance();
                    Token::DoubleEquals
                } else {
                    self.advance();
                    Token::Equals
                }
            }
            b'|' => {
                self.advance();
                self.expect(b'|');
                self.advance();
                Token::LogicalOr
            }
            b'&' => {
                self.advance();
                self.expect(b'&');
                self.advance();
                Token::LogicalAnd
            }
            b'(' => {
                self.advance();
                Token::LParen
            }
            b')' => {
                self.advance();
                Token::RParen
            }
            b'{' => {
                self.advance();
                Token::LBrace
            }
            b'}' => {
                self.advance();
                Token::RBrace
            }
            b'+' => {
                self.advance();
                Token::Plus
            }
            b'<' => {
                self.advance();
                if self.current == b'=' {
                    self.advance();
                    Token::LessEq
                } else {
                    self.advance();
                    Token::Less
                }
            }
            b'>' => {
                self.advance();
                if self.current == b'=' {
                    self.advance();
                    Token::GreaterEq
                } else {
                    self.advance();
                    Token::Greater
                }
            }
            x if x.is_ascii_digit() => self.parse_numeric_literal()?,
            x if x.is_ascii_alphabetic() => self.parse_alphabetic()?,
            b'\0' => { return Ok(None); }
            _ => bail!("Unknown token '{}'({}) at index {}", self.current as char, self.current, self.current_index),
        };

        Ok(Some(token))
    }

    fn parse_numeric_literal(&mut self) -> Result<Token> {
        let mut raw_number = String::new();
        while self.current.is_ascii_alphanumeric() {
            raw_number.push(self.current as char);
            self.advance();
        }

        Ok(Token::NumericLiteral(raw_number.parse().context(format!("Could not parse numeric literal '{}' at index {}", raw_number, self.current_index))?))
    }

    fn parse_alphabetic(&mut self) -> Result<Token> {
        let mut name = String::new();
        while self.current.is_ascii_alphanumeric() {
            name.push(self.current as char);
            self.advance();
        }


        Ok(
            Keyword::try_from_str(name.as_str())
                .map(|keyword| Token::Keyword(keyword))
                .unwrap_or(Token::Identifier(name)))
    }

    fn parse_comment(&mut self) {
        self.advance();
        self.expect(b'/');
        self.advance();
        loop {
            if self.current == b'\0' || self.current == b'\r' {
                return;
            }
            self.advance();
        }
    }
}