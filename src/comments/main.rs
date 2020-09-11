use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser, "/comments/parser.rs");

use parser::*;

fn main() {
    println!("comments");
}

#[test]
fn parse_id() {
    println!(
        "{:?}",
        IdParser::new().parse(
            "

            //a1_a //

            /* 
            a1_a
            */
            a1_a
            "
        )
    );
}

#[test]
fn parse_num() {
    println!("{:?}", NumParser::new().parse("123"));
}

#[test]
fn parse_num_or_id() {
    println!("{:?}", NumOrIdParser::new().parse("123"));
    println!("{:?}", NumOrIdParser::new().parse("a1_a"));
}
