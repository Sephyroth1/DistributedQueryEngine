use query::bytecode_comp::ByteCode;
use query::lexer::Lexer;
use query::logical::LogicalPlan;
use query::parser::Parser;

fn main() {
    let input = "select hello from world where id > 1";
    let mut lexer = Lexer::new(String::from(input));
    let toks = lexer.tokenize();
    println!("{:#?}", toks);
    let mut parser = Parser::new(toks);
    let q = parser.parse();
    println!("{:#?}", q);
    let query = match q {
        Ok(query) => query,
        Err(err) => panic!("Error: {}", err),
    };
    let plan = LogicalPlan::ast_to_lplan(query);
    println!("{:#?}", plan);
    let next_reg = &mut 0;
    let bytecode = ByteCode::compile(&plan, next_reg);
    println!("{:#?}", bytecode);
}
