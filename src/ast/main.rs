use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser, "/ast/parser.rs");

use parser::*;

pub mod ast;
use ast::*;

fn main() {
    println!("minimal");
    //println!("{:?}", ProgramParser::new().parse(tester));
}
type Error = String;
#[test]
fn type_check(stmts: &Stmts) -> Result<Id, Error> {
    let resul = stmts.iter().try_fold("".to_string(), |_, stmt| match stmt {
        Statement::Let(id, _) => {
            println!("let {} ...", id);
            Ok(id.to_string())
        }
        Statement::Cond(_, _, __) => {
            println!("if has an error");
            Err(format!("error found in statement {:?}", stmt))
        }

    })?;
    println!("here we can do something before returning");
    Ok(resul)

}

#[test]
fn test_typeCheck() {
    let stmts = vec![
        Statement::Let("a".to_string(), false, None, NumOrId::Num(1),None),
        Statement::Let("b".to_string(), false, None, NumOrId::Num(2),None),
        Statement::If("b".to_string(), Stmts::new(), None),
        Statement::Let("c".to_string(),false, None, NumOrId::Num(3), None),
    ];

}

#[test]
fn parse_let() {
    println!("{:?}", StmtParser::new().parse("
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
        }"));
    println!("{:?}", StmtParser::new().parse("
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
        //CHECK" 
));
    println!("{:?}", StmtParser::new().parse("
    // a function taking two bool arguments returning the i32 type
    // while
    fn c(x: bool, y: bool) -> i32 {
        let mut b: i32 = 0;
        let mut c: i32 = 1;
        while (b < 10) {
            c = c * 2;
        };
        c
    }" 
));
}
