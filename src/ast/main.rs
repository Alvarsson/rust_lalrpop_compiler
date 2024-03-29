use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser, "/ast/parser.rs");

use parser::*;

pub mod ast;

pub mod type_check;
pub mod interpreter;
// Add comments about where the SOS is performed in the show_case code. For semantics and type checker semantics.
// Edit the SOS according to pictures.
// Add better borrow check examples which demonstrates what is accepted and what is not.
// Change away from sequence
fn main() {
}

#[test]
fn test_ACCEPTED_complete_borrow() {
    let test_string = " 
    fn test() {
        fn test1(p: &String)  {
            let b = *p;
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
    let borrow_test = ProgramParser::new().parse(test_string).unwrap();
    let mut scope = type_check::Scope::newScope(test_string.to_string());
    let result = type_check::statement_check(vec![borrow_test], &mut scope);
    if result.is_err() {
        println!("{}", result.unwrap_err());
    } else {
        println!("type check Ok for ref, borrow, and deref.")
    }
}

#[test]
fn test_ACCEPTED_immutable_borrow() {
    let test_string = " 
    fn test() {
        fn test1()  {
            let b = 10;
            let c = &b;
            let d = &b;
        }
    }
    ";
    let borrow_test = ProgramParser::new().parse(test_string).unwrap();
    let mut scope = type_check::Scope::newScope(test_string.to_string());
    let result = type_check::statement_check(vec![borrow_test], &mut scope);
    if result.is_err() {
        println!("{}", result.unwrap_err());
    } else {
        println!("type check Ok for ref, borrow, and deref.")
    }
}

#[test]
fn test_ACCEPTED_drop_variable() {
    let test_string = " 
    fn test() {
        fn test1()  {
            let a = 5;
            if true {
                let b = &a;
                let c = 89;
            }
        }
    }
    ";
    let borrow_test = ProgramParser::new().parse(test_string).unwrap();
    let mut scope = type_check::Scope::newScope(test_string.to_string());
    let result = type_check::statement_check(vec![borrow_test], &mut scope);
    if result.is_err() {
        println!("{}", result.unwrap_err());
    } else {
        println!("type check Ok for ref, borrow, and deref.")
    }
}

#[test]
fn test_ILLEGAL_several_mut_borrow() {
    let test_string = " 
    fn test() {
        fn test1()  {
            let mut b = 10;
            let c = &mut b;
            let d = &mut b;
        }   
    }
    ";
    let borrow_test = ProgramParser::new().parse(test_string).unwrap();
    let mut scope = type_check::Scope::newScope(test_string.to_string());
    let result = type_check::statement_check(vec![borrow_test], &mut scope);
    if result.is_err() {
        println!("{}", result.unwrap_err());
    } else {
        println!("type check Ok for ref, borrow, and deref.")
    }
}

#[test]
fn test_ILLEGAL_out_of_scope_borrow() {
    let test_string = " 
    fn test() {
        fn test1()  {
            let a = 5;
            if true {
                let b = &a;
                let c = 89;
            }
            let d = &c;
        }
    }
    ";
    let borrow_test = ProgramParser::new().parse(test_string).unwrap();
    let mut scope = type_check::Scope::newScope(test_string.to_string());
    let result = type_check::statement_check(vec![borrow_test], &mut scope);
    if result.is_err() {
        println!("{}", result.unwrap_err());
    } else {
        println!("type check Ok for ref, borrow, and deref.")
    }
}

#[test]
fn test_() {
    let test_string = " 
    fn test() {
        let ref_to_nothing = dangle();
        fn dangle() -> &i32 {
            let s = 50;
            &s
        }
    }
    
    ";
    let borrow_test = ProgramParser::new().parse(test_string).unwrap();
    let mut scope = type_check::Scope::newScope(test_string.to_string());
    let result = type_check::statement_check(vec![borrow_test], &mut scope);
    if result.is_err() {
        println!("{}", result.unwrap_err());
    } else {
        println!("type check Ok for ref, borrow, and deref.")
    }
}


















//------------Interpreter with type checker test ---------------------
#[test]
fn test_interp_with_type_check() {
    
    let test_string = "
    fn main() -> () {
        fn tjo(p:i32) -> i32 {
            let a = 6;
            let b = 3;
            let c = a + b + p;
            return c;
        }
        tjo(4);
        fn hoj() -> i32 {
            let k = 8;
            return 8;
        }
        hoj();
    }
    main();
    ";
    let test = ProgramParser::new().parse(test_string).unwrap();
    let mut type_scope = type_check::Scope::newScope(test_string.to_string());
    let result = type_check::statement_check(vec![test], &mut type_scope);
    if result.is_err() {
        let type_check_result = result.unwrap();
        println!("{}", type_check_result);
    } else {    
        println!("TypeCheck OK");
        let parsing = ProgramParser::new().parse(test_string).unwrap();
        let mut scope = interpreter::Scope::newScope(test_string.to_string());
        let interp_result = interpreter::evaluate_program(vec![parsing], &mut scope);
        if interp_result.is_err() {
            let eval_result = interp_result.unwrap_err();
            println!("{:?}", eval_result);
        } else {
            println!("Evaluation OK");
        }
    }
}


//--------------BORROW CHECK TEST---------------
#[test]
fn test_borrow_check() {
    let test_string = " 
    fn test() {
        fn test1(p: &String)  {
            let b = *p;
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
    let borrow_test = ProgramParser::new().parse(test_string).unwrap();
    let mut scope = type_check::Scope::newScope(test_string.to_string());
    let result = type_check::statement_check(vec![borrow_test], &mut scope);
    if result.is_err() {
        println!("{}", result.unwrap_err());
    } else {
        println!("type check Ok for ref, borrow, and deref.")
    }
}

// ---------------TEST FOR PART 1 IN TYPE CHECKER----------------------
#[test]
fn test_type_check_part1() {
    let test_string = "
    fn main() -> i32 {
        fn tjo(p:i32) -> i32 {
            let a = 6;
            let b = 3;
            let c = a + b + p;
            return c;
        }
        tjo(4);
        fn hoj() -> i32 {
            let k = 8;
            return 8;
        }
        return hoj();
    }
    main();
    ";
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
    let test = ProgramParser::new().parse(test_string).unwrap();
    let mut scope = type_check::Scope::newScope(test_string.to_string());
    let r = type_check::statement_check(vec![test], &mut scope);
    if r.is_err() {
        println!("{}",r.unwrap_err());
    }
    else {
        println!("Part 1 OK for type_checker");
    }
}

//---------------------TEST FOR PART 2 IN TYPE CHECKER------------------------
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
    let test = ProgramParser::new().parse(part2).unwrap();
    let mut scope = type_check::Scope::newScope(part2.to_string());
    let r = type_check::statement_check(vec![test], &mut scope);
    if r.is_err() {
        println!("{}",r.unwrap_err());
    }
    else {
        println!("Part 2 OK for type_checker");
    }
}

// -------------------COMPLETER PARSING TEST OF PART 1, PART 2, WITH BORROW------------------
#[test]
fn parse_test() {
    let test_borrow = " 
    fn main() {
        fn test(a: &mut String) {
            let mut b = 'oj';
            let mut e = &mut b;
            test1(&b)
        }
        test();
        fn test2(per: &i32) -> i32 {
            let axel: i32 = 10 + 2 * 3;
            axel = -1 - (1 - 1);
            return axel + per;
        }
        fn test3(foo: bool) -> bool {
            if foo && true || false {
                let a = test2(5);
                while ( a < 10) {
                    a = a + 1;
                }
                return true;
            }
            return 5 > 7;
        }
    }
    main();
    ";
    //println!("{:?}", StmtsParser::new().parse(test_borrow));
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
    //println!("{:?}", StmtsParser::new().parse(part1));
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
    //println!("{:?}", ProgramParser::new().parse(part2));
    let test_string = "
    fn main() {
        fn tjo(p:i32) -> i32 {
            let a = 5;
    } 
    ";
    println!("{:?}", ProgramParser::new().parse(test_string));
}

