use mysql_parser_rs::lexer::Lexer;
use mysql_parser_rs::parser::Parser;

fn main() {
    let query = String::from("SELECT 1+1+2, id, name FROM user WHERE id = 1;");
    println!("query: {}", query);
    let mut lexer = Lexer::new(query);
    let mut parser = Parser::new(lexer);
    let stmts = parser.parse();
    println!("ast:\n{:?}", stmts);
}
