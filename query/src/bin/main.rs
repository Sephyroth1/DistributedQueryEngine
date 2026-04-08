use query::lexer::Lexer;
use query::parser::Parser;

fn main() {
    let input = "select hello from world";
    let mut lexer = Lexer::new(String::from(input));
    let toks = lexer.tokenize();
    println!("{:?}", toks);
    let mut parser = Parser::new(toks);
    println!("{:?}", parser.parse());
}
