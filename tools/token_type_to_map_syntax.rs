use mysql_parser_rs::token_type::{str_for_tool_map_syntax, TokenType};
use strum::IntoEnumIterator;

fn main() {
    for tt in TokenType::iter() {
        let to_str = str_for_tool_map_syntax(&tt);
        println!("m.insert({:?}, TokenType::{:?});", to_str, tt);
    }
}
