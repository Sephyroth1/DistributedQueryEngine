use crate::base::{Expr, Node, Op, SelectNode};
use crate::lexer::Token;
use std::error::Error;

pub struct Parser {
    position: usize,
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            position: 0,
            tokens,
        }
    }

    fn peek(&mut self) -> Option<&Token> {
        if self.position < self.tokens.len() {
            Some(&self.tokens[self.position])
        } else {
            None
        }
    }

    fn advance(&mut self) -> Option<&Token> {
        if self.position < self.tokens.len() {
            self.position += 1;
            self.peek()
        } else {
            None
        }
    }

    fn op_precedence(&mut self) -> u8 {
        let token = self.peek().unwrap();
        const PRECEDENCE_LOWEST: u8 = 1;
        const PRECEDENCE_COMPARISON: u8 = 4;
        const PRECEDENCE_LOGICAL: u8 = 5;

        match token {
            Token::LT(_) => PRECEDENCE_COMPARISON,
            Token::GT(_) => PRECEDENCE_COMPARISON,
            Token::EQ(_) => PRECEDENCE_COMPARISON,
            Token::NE(_) => PRECEDENCE_COMPARISON,
            Token::AND(_) => PRECEDENCE_LOGICAL,
            Token::OR(_) => PRECEDENCE_LOGICAL,
            _ => PRECEDENCE_LOWEST,
        }
    }

    fn parse_number(&mut self) -> Expr {
        let token = self.advance().unwrap();
        match token {
            Token::NUMBER(value) => Expr::Number(*value),
            _ => unreachable!(),
        }
    }

    fn parse_ident(&mut self) -> Expr {
        let token = self.advance().unwrap();
        match token {
            Token::IDENT(name) => Expr::Identifier(name.clone()),
            Token::SELECT(_) => Expr::Identifier("select".to_string()),
            _ => unreachable!("Unexpected token {:?}", token),
        }
    }

    pub fn match_token(&mut self, token: &Token) -> bool {
        let tok = self.peek().unwrap();
        if tok == token { true } else { false }
    }

    pub fn token_to_op(&mut self, token: &Token) -> Option<Op> {
        match token {
            Token::AND(_) => Some(Op::AND),
            Token::OR(_) => Some(Op::OR),
            Token::EQ(_) => Some(Op::EQ),
            Token::NE(_) => Some(Op::NE),
            Token::LT(_) => Some(Op::LT),
            Token::GT(_) => Some(Op::GT),
            Token::ASSIGN(_) => Some(Op::ASSIGN),
            _ => None,
        }
    }

    pub fn parse_expr(&mut self) -> Expr

    pub fn parse_columns(&mut self) -> Vec<Expr> {
        let mut columns = Vec::new();
        loop {
            columns.push(self.parse_ident().to_string());
            if self.match_token(&Token::COMMA) {
                self.advance();
            } else {
                panic!("Expected comma after column name")
            }
        }
    }

    pub fn parse_select(&mut self) -> SelectNode {
        if self.match_token(&Token::SELECT((String::from("Select")))) {
            self.advance();
            let columns = self.parse_columns();
        }
    }
    pub fn parse(&mut self) -> Result<Node, String> {
        let token = self.peek().unwrap_or(&Token::EOF);
        match token {
            Token::SELECT(_) => Ok(Node::Select(self.parse_select())),
            _ => Err(format!("Unexpected token {:?}", token)),
        }
    }
}
