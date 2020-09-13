use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser, "/ast/parser.rs");

use parser::*;

pub mod ast;

fn main() {
    println!("minimal");
}

#[test]
fn parse_let() {
    println!("{:?}", StmtParser::new().parse("
    fn a(b:i32, c:i32) {5}")); 
    println!("{:?}", StmtParser::new().parse("
    fn a() {
        let a = 5;
        let b = 6;
        let c = 7;
    }"));
    assert_eq!(format!("{}", StmtParser::new().parse("
    fn a() {
        let a = 5;
        let b = 6;
        let c = 7;
    }").unwrap()), 
    "fn a() {
        let a = 5;
        let b = 6;
        let c = 7;
    }");
}

//#[test]
//fn _let_and_return() {
    /* // a function taking no arguments returning the unit type
    println!("{:?}", StmtParser::new().parse("
        fn a() -> i32 {
            let _a: i32 = 5; 
        }"
    ));
    // a function taking two i32 arguments returning the i32 type
    println!("{:?}", StmtParser::new().parse("
        fn b(_x: i32, _y: i32) -> i32 {
            3
        }"
    ));
    // a function taking two i32 arguments returning the i32 type
    // with some let statements */
//    println!("{:?}", MultStmtsParser::new().parse("
//        fn c(x: i32, y: i32) -> i32 {
//            let a: i32 = 5;
//            let b: i32 = x + y; 
//            -a - (-b) * y 
//        }"
//    ));
//}
/* 
// More advanced statements

fn _if_then_else_and_while() {
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

    // a function taking two bool arguments returning the i32 type
    // with some let statements and function calls
    fn b(x: bool, y: bool) -> i32 {
        let a: bool = a(x, y || false);
        let mut b: i32 = 0;
        if a && y {
            let a: bool = true; // shadowing
            if y || a {
                b = b + 1;
            };
        } else {
            if !(x && false) {
                b = b - 1;
            }
        };
        b + 3
    }

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
}
 */