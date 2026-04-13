pub mod base;
pub mod bytecode_comp;
pub mod lexer;
pub mod logical;
pub mod parser;

use lexer::Lexer;
use lexer::Token;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let input = "select hello";
        let mut lexer = Lexer::new(String::from(input));
        let tokens = lexer.tokenize();
        assert_eq!(
            tokens,
            vec![Token::SELECT, Token::IDENT("hello".to_string()), Token::EOF]
        );
    }
}
