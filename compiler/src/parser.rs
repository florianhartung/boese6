use anyhow::bail;
use anyhow::Result;

use crate::ast::{AstCompound, AstFunction, AstRoot, AstRootCompound, AstStatement};
use crate::token::{Keyword, Token};

pub struct Parser {
    tokens: Vec<Token>,
    current_index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current_index: 0,
        }
    }

    fn current(&self) -> Option<&Token> {
        if self.current_index < self.tokens.len() {
            Some(&self.tokens[self.current_index])
        } else {
            None
        }
    }

    fn advance(&mut self) {
        self.current_index += 1;
    }

    fn expect(&self, matcher: fn(&Token) -> bool) {
        let current = self
            .current()
            .expect("Expected token but reached end of token stream");

        if !matcher(current) {
            eprintln!("Unexpected token '{:?}'", current);
        }
    }

    pub fn parse(&mut self) -> Result<AstRootCompound> {
        let mut top_level_compound = Vec::new();

        while let Some(current) = self.current() {
            match current {
                Token::Newline => {
                    self.advance();
                }
                Token::Keyword(Keyword::Function) => {
                    let function = AstRoot::Function(self.parse_function()?);
                    top_level_compound.push(function);
                }
                _ => bail!("Found unexpected token '{:?}' while parsing", current),
            }
        }

        Ok(top_level_compound)
    }

    fn parse_function(&mut self) -> Result<AstFunction> {
        self.advance();
        if let Some(current) = self.current() {
            if let Token::Identifier(name) = current.clone() {
                self.advance();

                self.expect(|t| matches!(t, Token::LParen));
                self.advance();
                self.expect(|t| matches!(t, Token::RParen));
                self.advance();

                self.expect(|t| matches!(t, Token::LBrace));

                let compound = self.parse_compound()?;

                Ok(AstFunction { name, compound })
            } else {
                bail!(
                    "Expected a function name in form of a valid identifier, found token '{:?}'",
                    self.current()
                )
            }
        } else {
            bail!(
                "Expected a function name in form of a valid identifier, found end of token stream"
            )
        }
    }

    fn parse_compound(&mut self) -> Result<AstCompound> {
        self.advance();

        let mut nodes = Vec::new();
        while let Some(current) = self.current() {
            match current {
                Token::Keyword(Keyword::Continue) => {
                    self.advance();
                    nodes.push(AstStatement::Continue);
                }
                Token::Keyword(Keyword::End) => {
                    self.advance();
                    nodes.push(AstStatement::End);
                }
                Token::Newline => {
                    self.advance();
                }
                Token::RBrace => {
                    self.advance();
                    return Ok(AstCompound { nodes });
                }
                _ => {
                    bail!(
                        "Unexpected token '{:?}', expected a statement or '}}'",
                        current
                    )
                }
            }
        }
        bail!("Reached end of token stream while trying to parse a compound")
    }
}
