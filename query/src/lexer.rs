use std::vec::Vec;

// struct Token<T> {
//     value: T,
// }

#[derive(Debug, PartialEq)]
pub enum Token {
    SELECT(String),
    WHERE(String),
    FROM(String),
    IDENT(String),
    NUMBER(u64),
    STRING(String),
    EQ(String),
    ASSIGN(String),
    GE(String),
    GT(String),
    LPAREN(String),
    RPAREN(String),
    LT(String),
    LE(String),
    NE(String),
    NOT(String),
    AND(String),
    OR(String),
    EOF(String),
}

impl Clone for Token {
    fn clone(&self) -> Self {
        match self {
            Token::SELECT(value) => Token::SELECT(value.clone()),
            Token::WHERE(value) => Token::WHERE(value.clone()),
            Token::FROM(value) => Token::FROM(value.clone()),
            Token::IDENT(value) => Token::IDENT(value.clone()),
            Token::NUMBER(value) => Token::NUMBER(*value),
            Token::STRING(value) => Token::STRING(value.clone()),
            Token::GE(value) => Token::GE(value.clone()),
            Token::GT(value) => Token::GT(value.clone()),
            Token::EQ(value) => Token::EQ(value.clone()),
            Token::ASSIGN(value) => Token::ASSIGN(value.clone()),
            Token::LT(value) => Token::LT(value.clone()),
            Token::LE(value) => Token::LE(value.clone()),
            Token::NE(value) => Token::NE(value.clone()),
            Token::NOT(value) => Token::NOT(value.clone()),
            Token::AND(value) => Token::AND(value.clone()),
            Token::OR(value) => Token::OR(value.clone()),
            Token::LPAREN(value) => Token::LPAREN(value.clone()),
            Token::RPAREN(value) => Token::RPAREN(value.clone()),
            Token::EOF(value) => Token::EOF(value.clone()),
        }
    }
}

pub struct Lexer {
    input: String,
    position: usize,
    current_char: Option<char>,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {
            input,
            position: 0,
            current_char: None,
            tokens: Vec::new(),
        }
    }

    fn peek(&mut self) -> Option<char> {
        if self.position >= self.input.len() {
            None
        } else {
            self.current_char = self.input.chars().nth(self.position);
            self.current_char
        }
    }

    fn advance(&mut self) {
        if self.position >= self.input.len() {
            self.current_char = None;
        } else {
            self.position += 1;
            self.peek();
        }
    }

    fn error(&self, message: &str) {
        println!("Lexer error: {}", message);
    }

    fn parse_identifier(&mut self) {
        let mut identifier = String::new();
        while self.peek() != None
            && self.current_char.is_some()
            && self.current_char.unwrap().is_alphabetic()
        {
            identifier.push(self.current_char.unwrap());
            self.advance();
        }
        if identifier == "select" {
            self.tokens.push(Token::SELECT(identifier));
        } else if identifier == "from" {
            self.tokens.push(Token::FROM(identifier));
        } else if identifier == "where" {
            self.tokens.push(Token::WHERE(identifier));
        } else {
            let token = Token::IDENT(identifier);
            self.tokens.push(token);
        }
    }

    fn parse_number(&mut self) {
        let mut num = String::new();
        while self.peek() != None
            && self.current_char.is_some()
            && self.current_char.unwrap().is_numeric()
        {
            num.push(self.current_char.unwrap());
            self.advance();
        }
        self.tokens.push(Token::NUMBER(match num.parse::<u64>() {
            Ok(n) => n,
            Err(_) => 0,
        }));
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        self.tokens = Vec::new();
        self.current_char = self.peek();
        while self.current_char.is_some() {
            let cur = self.current_char.unwrap();
            match cur {
                cur if cur.is_alphabetic() => self.parse_identifier(),
                cur if cur.is_numeric() => self.parse_number(),
                cur if cur == '=' => {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        self.tokens.push(Token::EQ(String::from("==")));
                    } else {
                        self.tokens.push(Token::ASSIGN(String::from("=")));
                    }
                }
                cur if cur == '>' => {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        self.tokens.push(Token::GE(String::from(">=")));
                    } else {
                        self.tokens.push(Token::GT(String::from(">")));
                    }
                }
                cur if cur == '<' => {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        self.tokens.push(Token::LE(String::from("<=")));
                    } else {
                        self.tokens.push(Token::LT(String::from("<")));
                    }
                }
                cur if cur == '!' => {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        self.tokens.push(Token::NE(String::from("!=")));
                    } else {
                        self.tokens.push(Token::NOT(String::from("!")));
                    }
                }
                cur if cur == '&' => {
                    self.advance();
                    self.tokens.push(Token::AND(String::from("&")));
                }
                cur if cur == '|' => {
                    self.advance();
                    self.tokens.push(Token::OR(String::from("|")));
                }
                cur if cur == '(' => self.tokens.push(Token::LPAREN(String::from("("))),
                cur if cur == ')' => self.tokens.push(Token::RPAREN(String::from(")"))),
                ' ' | '\t' | '\n' => self.advance(),
                _ => self.error("Unexpected character"),
            }
            self.advance();
        }
        self.tokens.push(Token::EOF(String::from("EOF")));
        self.tokens.clone()
    }
}
