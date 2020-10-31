
use std::collections::HashMap;

use crate::ast::*;

type Error = String;

// this will most probablt look alot like the type_checker.

/* #[derive(Clone)]
struct VariableValues {
    symbolbase: Type,
    mutable: bool,
    referencedmut: bool,
    reference: bool,
    borrowed: bool,
    moved: bool,
    var_value: Value,
} */
#[derive(Clone)]
struct Values {
    Bool: bool,
    Int: i32,
}

pub struct VariableValues {
    scope_layer: i32,
    func_table: HashMap<i32, HashMap<String, SignatureType>>,
    var_table: HashMap<i32, HashMap<String, Values>>,
    func_scope: Vec<i32>,
}

pub fn evaluate(str: &str) {
    fn type_check_test(str: &str) {
        let test = StmtsParser::new(str).unwrap();
        let mut type_scope = type_check::Scope::newScope(str.to_string());
        let result = type_check::statement_check(vec![test], &mut type_scope);
        if result.is_err() {
            println!("{}", result.unwrap_err());
        } else {
            evaluate_program();
        }
    }
    fn eval_expr(expr: Box<Exprs>, scope: &mut VariableValues) -> Values{
        match *expr {
            Exprs::Boolean(_) => Type::Bool,
            Exprs::Str(st) => Type::Str,
            Exprs::Number(_) => Type::i32,
            Exprs::Id(id) => match scope.get_symbol(&id) {
                Some(var) => match var.value {
                    Values::Int => Exprs::Number()
                }
            }
        }
    }

    fn evaluate_program() {
        
    }
}
