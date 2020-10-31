use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser, "/ast/parser.rs");

use parser::*;

pub mod ast;
pub mod type_check;
pub mod interp;


fn main() {
    println!("minimal");
    let part1 = "
    fn _let_and_return() {
        // a function taking no arguments returning the unit type
        fn a() -> () {
            let _a: i32 = 5; // this returns a unit type
        }
    
        // a function taking two i32 arguments returning the i32 type
        fn b(_x: i32, _y: i32) -> i32 {
            3 // this returns 3 (as i32)
        }
    
        // a function taking two i32 arguments returning the i32 type
        // with some let statements
        fn c(x: i32, y: i32) -> i32 {
            let a: i32 = 5;
            let b: i32 = x + y; // this will be an infix operator +
            -a - (-b) * y // here we have prefix operator '-'
        }
    } ";

    let part2 = "
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

    }";
    let borrow_test = " 
    fn test() {
        fn test1(p: &String) {

        }
        fn test2(a: &mut String) {
            let mut b = 'oj';
            let mut e = &mut b;
            test1(&b)
            
        }
        fn test3(b: &i32) -> i32 {
            return *b + 4;
        }
    }
    ";

}
//BORROW CHECK TEST IN TYPE CHECKER
#[test]
fn test_borrow_check() {
    let test_string = " 
    fn test() {
        fn test1(p: &String) {

        }
        fn test2(a: &mut String) {
            let mut b = 'oj';
            let mut e = &mut b;
            test1(&b)
            
        }
        fn test3(b: &i32) -> i32 {
            return *b + 4;
        }
    }
    ";
    let borrow_test = StmtsParser::new().parse(test_string).unwrap();
    let mut scope = type_check::Scope::newScope(test_string.to_string());
    let result = type_check::statement_check(vec![borrow_test], &mut scope);
    if result.is_err() {
        println!("{}", result.unwrap_err());
    } else {
        println!("type check Ok for ref, borrow, and deref.")
    }
}




// TEST FOR PART 1 IN TYPE CHECKER
#[test]
fn test_type_check_part1() {
    let part1 = "
    fn _let_and_return() {
        // a function taking no arguments returning the unit type
        fn a() -> () {
            let _a: i32 = 5; // this returns a unit type
        }
    
        // a function taking two i32 arguments returning the i32 type
        fn b(_x: i32, _y: i32) -> i32 {
            3 // this returns 3 (as i32)
        }
    
        // a function taking two i32 arguments returning the i32 type
        // with some let statements
        fn c(x: i32, y: i32) -> i32 {
            let a: i32 = 5;
            let b: i32 = x + y; // this will be an infix operator +
            -a - (-b) * y // here we have prefix operator '-'
        }
    } ";
    let test = StmtsParser::new().parse(part1).unwrap();
    let mut scope = type_check::Scope::newScope(part1.to_string());
    let r = type_check::statement_check(vec![test], &mut scope);
    if r.is_err() {
        println!("{}",r.unwrap_err());
    }
    else {
        println!("Part 1 OK for type_checker");
    }
}

//TEST FOR PART 2 IN TYPE CHECKER
#[test]
fn test_type_check_part2() {
    let part2 = "
    
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
    }";
    let test = StmtsParser::new().parse(part2).unwrap();
    let mut scope = type_check::Scope::newScope(part2.to_string());
    let r = type_check::statement_check(vec![test], &mut scope);
    if r.is_err() {
        println!("{}",r.unwrap_err());
    }
    else {
        println!("Part 2 OK for type_checker");
    }
}

// COMPLETER PARSING TEST OF PART 1, PART 2, AND BORROW
#[test]
fn parse_test() {
    let test_borrow = " 
    fn test() {
        fn test1(p: &String) {

        }
        fn test2(a: &mut String) {
            let mut b = 'oj';
            let mut e = &mut b;
            test1(&b)
            
        }
        fn test3(b: &i32) -> i32 {
            return *b + 4;
        }
    }
    ";
    println!("{:?}", StmtsParser::new().parse(test_borrow));
    let part1 = "
    fn _let_and_return() {
        // a function taking no arguments returning the unit type
        fn a() -> () {
            let _a: i32 = 5; // this returns a unit type
        }
    
        // a function taking two i32 arguments returning the i32 type
        fn b(_x: i32, _y: i32) -> i32 {
            3 // this returns 3 (as i32)
        }
    
        // a function taking two i32 arguments returning the i32 type
        // with some let statements
        fn c(x: i32, y: i32) -> i32 {
            let a: i32 = 5;
            let b: i32 = x + y; // this will be an infix operator +
            -a - (-b) * y // here we have prefix operator '-'
        }
    } ";
    println!("{:?}", StmtsParser::new().parse(part1));
    let part2 = "
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

    }";
    println!("{:?}", StmtsParser::new().parse(part2));
    let test = "
    fn hej() {
        let a = 5;
        let b = 7;
        let c = a + b;
        return c;
    }";
    //println!("{:?}", StmtsParser::new().parse(test));
}
