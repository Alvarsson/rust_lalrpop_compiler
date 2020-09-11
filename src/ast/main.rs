use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser, "/ast/parser.rs");

use parser::*;

pub mod ast;

fn main() {
    println!("minimal");
    println!("{:?}", NumOrIdParser::new().parse("123"));
    println!("{:?}", NumOrIdParser::new().parse("a1_a"));
    println!("{:?}", ExprParser::new().parse("1+2+3"));
}

#[test]
fn parse_num_id() {
    assert_eq!(format!("{}", NumOrIdParser::new().parse("123").unwrap()), "123");
    assert_eq!(format!("{}", NumOrIdParser::new().parse("a1_a").unwrap()), "a1_a");
}
#[test]
fn parse_rel_op() {
    println!("{:?}", ExprParser::new().parse("5 < 7"));
}
#[test]
fn parse_number_expr() {
    assert_eq!(format!("{}", parser::ExprParser::new().parse("1+2+3").unwrap()), "((1+2)+3)");
    assert_eq!(format!("{}", parser::ExprParser::new().parse("1+2*3").unwrap()), "(1+(2*3))");
}

#[test]
fn parse_let() {
    assert_eq!(format!("{}", StmtParser::new().parse("
    let a = 5;
    let b = 6;
    ").unwrap()), "
    let a = 5;
    let b = 6
    ");
    assert_eq!(format!("{}", StmtParser::new().parse("let _a = 5;").unwrap()), "let _a = 5;");
    assert_eq!(format!("{}", StmtParser::new().parse("let a = true;").unwrap()), "let a = true;");
}
#[test]
fn parse_return() {
    //println!("{:?}", StmtParser::new().parse("return 4;"));
    assert_eq!(format!("{}", StmtParser::new().parse("return 4;").unwrap()), "return 4;");
    assert_eq!(format!("{}", StmtParser::new().parse("4").unwrap()), "4");
}
#[test]
fn parse_if() {
    println!("{:?}", StmtParser::new().parse(
        "if 5 < 7 {
            5
        }"
    ));
    assert_eq!(format!("{}", StmtParser::new().parse(
        "if 5<7 {
            5
        }"
    ).unwrap()), "if (5 < 7) { 5 }");
}

#[test]
fn parse_while() {
    println!("{:?}", StmtParser::new().parse(
        "while a < b {
            9
        }"
    ));
}
#[test]
fn parse_assign() {
    println!("{:?}", StmtParser::new().parse(
        "a = b"
    ));
}

#[test]
fn parse_function() {
    println!("{:?}", StmtParser::new().parse(
        "fn a() {
            5
        }"
    ));
    println!("{:?}", StmtParser::new().parse(
        "fn a(b:i32, c:i32) {
            5
        }"
    ));
    println!("{:?}", StmtParser::new().parse(
        "fn a(b:i32, c:i32) -> i32 {
            5
        }"
    ));
    assert_eq!(format!("{}", StmtParser::new().parse(
        "fn a(b:i32, c:i32) {5}"
    ).unwrap()), "fn a(b:i32, c:i32) {5}");
    assert_eq!(format!("{}", StmtParser::new().parse(
        "fn a() {5}"
    ).unwrap()), "fn a() {5}");
    
}

#[test]
fn test_function_call() {
    println!("{:?}", StmtParser::new().parse(
        "hejsan(1,2,3)"
    ));
}

#[test]
fn _let_and_return() {
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
    println!("{:?}", StmtParser::new().parse("
        fn c(x: i32, y: i32) -> i32 {
            let a: i32 = 5;
            let b: i32 = x + y; 
            -a - (-b) * y 
        }"
    ));
}

// More advanced statements
#[test]
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
