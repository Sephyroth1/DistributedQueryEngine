use query::lexer::Lexer;

fn main() {
    let input = "select hello from users where id = 1";
    let mut lexer = Lexer::new(String::from(input));
    println!("{:?}", lexer.tokenize());
}
