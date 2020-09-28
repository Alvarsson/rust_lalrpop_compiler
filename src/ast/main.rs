use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser, "/ast/parser.rs");

use parser::*;

pub mod ast;
pub mod type_check;


fn main() {
    println!("minimal");
}

#[test]
fn parse_let() {
    println!("{:?}", StmtsParser::new().parse("
    // a function taking two bool arguments returning the bool type
    // with some let statements and function calls
    fn a(x: bool, y: bool) -> bool {
        if x && y {
            let a: bool = true;
            y || a
        } else {
            x && false
        }
    } 
    "));
    println!("{:?}", StmtsParser::new().parse("
    fn b(x: bool, y: bool) -> i32 {
        let a: bool = a(x, y || false);
        let mut b: i32 = 0;
        if a && y {
            let a: bool = true; // shadowing
            if y || a {
                b = b + 1;
            }
        } else {
            if !(x && false) {
                b = b - 1;
            }
        }
        b + 3
    }
    "));
    println!("{:?}", StmtsParser::new().parse("
    // a function taking two bool arguments returning the i32 type
    // while
    fn c(x: bool, y: bool) -> i32 {
        let mut b: i32 = 0;
        let mut c: i32 = 1;
        while (b < 10) {
            c = c * 2;
        }
        c
    }
    "));
}
