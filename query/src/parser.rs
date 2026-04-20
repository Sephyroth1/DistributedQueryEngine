use crate::base::{Expr, Query, Table, Value};
use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    column_id: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
            column_id: 0,
        }
    }

    pub fn peek(&mut self) -> Option<&Token> {
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

    pub fn op_prec(&mut self) -> u8 {
        let tk = self.peek().unwrap();
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
        let tok = self.peek().unwrap();
        match tok {
            Token::SELECT => self.parse_select(),
            _ => panic!("Unexpected Token"),
        }
    }

    pub fn parse_select(&mut self) -> Result<Query, String> {
        if self.match_token(Token::SELECT) {
            let expr = self.parse_list();
            self.advance();
            println!("Peek: {:?}", self.peek().unwrap());
            self.match_token(Token::FROM);
            println!("Peek: {:?}", self.peek().unwrap());
            let table = self.parse_table(&expr);
            self.advance();
            if self.match_token(Token::EOF) {
                Ok(Query::Select {
                    columns: expr,
                    from: table,
                    where_clause: None,
                })
            } else {
                println!("Peek: {:?}", self.peek().unwrap());
                let where_clause = self.parse_expr(0);
                Ok(Query::Select {
                    columns: expr,
                    from: table,
                    where_clause: Some(Box::new(where_clause)),
                })
            }
        } else {
            panic!("Unexpected Token");
        }
    }

    pub fn parse_table(&mut self, columns: &Vec<Expr>) -> Table {
        let table = Table {
            name: self.parse_ident(),
            table_id: 0,
            columns: columns.clone(),
        };
        self.advance();
        table
    }

    // pub fn parse_column(&mut self) -> Expr {
    //     let column = Expr::Column {
    //         column_id: self.column_id,
    //     };
    //     self.column_id += 1;
    //     self.advance();
    //     column
    // }

    pub fn parse_list(&mut self) -> Vec<Expr> {
        let mut exprs = Vec::new();

        while !self.match_token(Token::FROM) {
            exprs.push(self.parse_expr(0));
            if self.match_token(Token::COMMA) {
                self.advance();
            } else {
                break;
            }
        }
        exprs
    }

    pub fn parse_expr(&mut self, precedence: u8) -> Expr {
        let mut left = self.parse_primary();

        while self.current < self.tokens.len() {
            let op = self.op_prec();
            if op > precedence {
                self.advance();
                let right = self.parse_expr(op);
                // self.match_token(Token::RPAREN);
                self.advance();
                left = Expr::Binary {
                    left: Box::new(left),
                    op: self.parse_ident(),
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        left
    }

    pub fn parse_ident(&mut self) -> String {
        let token = self.peek().unwrap().clone();
        let ident;
        match token {
            Token::IDENT(name) => {
                ident = name;
            }
            _ => panic!("Unexpected Token {:?}", token),
        }
        ident
    }

    pub fn parse_primary(&mut self) -> Expr {
        match self.peek().unwrap() {
            Token::LPAREN => {
                self.advance();
                let expr = self.parse_expr(0);
                self.match_token(Token::RPAREN);
                self.advance();
                expr
            }
            Token::NUMBER(_) => self.parse_number(),
            Token::STRING(_) => self.parse_string(),
            Token::IDENT(value) => {
                if value == "true" {
                    Expr::Literal(Value::Bool(true))
                } else if value == "false" {
                    Expr::Literal(Value::Bool(false))
                } else {
                    let val = Expr::Column {
                        name: value.clone(),
                        column_id: self.column_id,
                    };
                    self.column_id += 1;
                    val
                }
            }
            _ => panic!("Unexpected Token {:?}", self.peek().unwrap()),
        }
    }

    pub fn parse_number(&mut self) -> Expr {
        let token = self.tokens[self.current - 1].clone();
        match token {
            Token::NUMBER(value) => {
                let expr = Expr::Literal(Value::Int(value));
                self.advance();
                expr
            }
            _ => panic!("Unexpected Token"),
        }
    }

    pub fn parse_string(&mut self) -> Expr {
        let token = self.tokens[self.current - 1].clone();
        match token {
            Token::STRING(value) => {
                let expr = Expr::Literal(Value::String(value));
                self.advance();
                expr
            }
            _ => panic!("Unexpected Token"),
        }
    }
}
