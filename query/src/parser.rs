use crate::base::{Expr, Query};
use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    pub fn advance(&mut self) {
        if self.current < self.tokens.len() {
            self.current += 1;
        } else {
            panic!("End of input reached");
        }
    }

    pub fn consume(&mut self, expected: Token) {
        if let Some(token) = self.peek() {
            if token == &expected {
                self.advance();
            } else {
                panic!("Unexpected token");
            }
        } else {
            panic!("Unexpected end of input");
        }
    }

    pub fn match_token(&mut self, expected: Token) -> bool {
        if let Some(token) = self.peek() {
            if token == &expected {
                self.advance();
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn op_prec(&mut self, tk: Token) -> u8 {
        match tk {
            Token::OR => 1,
            Token::AND => 2,
            Token::EQ | Token::NE => 3,
            Token::LT | Token::GT | Token::LE | Token::GE => 4,
            Token::PLUS | Token::MINUS => 5,
            Token::STAR | Token::SLASH => 6,
            Token::WILDCARD => 7,
            _ => 0,
        }
    }

    pub fn parse(&mut self) -> Result<Query, String> {
        let mut tok = self.peek().unwrap();
        match tok {
            Token::SELECT => self.parse_select()?,
            _ => panic!("Unexpected Token"),
        }
    }

    pub fn parse_select(&mut self) -> Result<Query, String> {
        if self.match_token(Token::SELECT) {
            self.advance();
            let expr = self.parse_expr();
            
        } else {
            panic!("Unexpected Token");
        }
    }
}
