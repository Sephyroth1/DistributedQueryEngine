use std::vec::Vec;

// struct Token<T> {
//     value: T,
// }

#[derive(Debug, PartialEq)]
pub enum Token {
    SELECT,
    WHERE,
    FROM,
    IDENT(String),
    NUMBER(u64),
    STRING(String),
    EQ,
    ASSIGN,
    GE,
    GT,
    LPAREN,
    RPAREN,
    LT,
    LE,
    NE,
    NOT,
    AND,
    OR,
    PLUS,
    MINUS,
    STAR,
    SLASH,
    COMMA,
    EOF,
    WILDCARD,
}

impl Clone for Token {
    fn clone(&self) -> Self {
        match self {
            Token::SELECT => Token::SELECT,
            Token::WHERE => Token::WHERE,
            Token::FROM => Token::FROM,
            Token::IDENT(value) => Token::IDENT(value.clone()),
            Token::NUMBER(value) => Token::NUMBER(*value),
            Token::STRING(value) => Token::STRING(value.clone()),
            Token::GE => Token::GE,
            Token::GT => Token::GT,
            Token::EQ => Token::EQ,
            Token::ASSIGN => Token::ASSIGN,
            Token::LT => Token::LT,
            Token::LE => Token::LE,
            Token::NE => Token::NE,
            Token::NOT => Token::NOT,
            Token::AND => Token::AND,
            Token::OR => Token::OR,
            Token::LPAREN => Token::LPAREN,
            Token::RPAREN => Token::RPAREN,
            Token::COMMA => Token::COMMA,
            Token::WILDCARD => Token::WILDCARD,
            Token::STAR => Token::STAR,
            Token::PLUS => Token::PLUS,
            Token::MINUS => Token::MINUS,
            Token::SLASH => Token::SLASH,
            Token::EOF => Token::EOF,
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
            self.tokens.push(Token::SELECT);
        } else if identifier == "from" {
            self.tokens.push(Token::FROM);
        } else if identifier == "where" {
            self.tokens.push(Token::WHERE);
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
                        self.tokens.push(Token::EQ);
                    } else {
                        self.tokens.push(Token::ASSIGN);
                    }
                }
                cur if cur == '>' => {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        self.tokens.push(Token::GE);
                    } else {
                        self.tokens.push(Token::GT);
                    }
                }
                cur if cur == '<' => {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        self.tokens.push(Token::LE);
                    } else {
                        self.tokens.push(Token::LT);
                    }
                }
                cur if cur == '!' => {
                    self.advance();
                    if self.current_char == Some('=') {
                        self.advance();
                        self.tokens.push(Token::NE);
                    } else {
                        self.tokens.push(Token::NOT);
                    }
                }
                cur if cur == '&' => {
                    self.advance();
                    self.tokens.push(Token::AND);
                }
                cur if cur == '|' => {
                    self.advance();
                    self.tokens.push(Token::OR);
                }
                cur if cur == ',' => {
                    self.advance();
                    self.tokens.push(Token::COMMA);
                }
                cur if cur == '(' => self.tokens.push(Token::LPAREN),
                cur if cur == ')' => self.tokens.push(Token::RPAREN),
                ' ' | '\t' | '\n' => self.advance(),
                _ => self.error("Unexpected character"),
            }
            self.advance();
        }
        self.tokens.push(Token::EOF);
        self.tokens.clone()
    }
}
