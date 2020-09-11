use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser, "/minimal/parser.rs");

use parser::*;

fn main() {
    println!("minimal");
    println!("{:?}", NumOrIdParser::new().parse("123"));
    println!("{:?}", NumOrIdParser::new().parse("a1_a"));
}

#[test]
fn parse_num_or_id() {
    println!("{:?}", NumOrIdParser::new().parse("123"));
    println!("{:?}", NumOrIdParser::new().parse("a1_a"));
}
