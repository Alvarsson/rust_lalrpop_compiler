
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
enum Values {
    Bool(bool),
    Int(i32),
}
impl std::convert::From<&Values> for String {
    fn from(val: &Values) -> Self {
        match val {
            Values::Bool(boolean) => boolean.to_string(),
            Values::Int(integer) => integer.to_string(),
        }
    }
}
impl std::convert::TryFrom<Exprs> for Values {
    type Error = &'static str;
    fn try_from(exp: Exprs) -> Result<Self, Self::Error> {
        match exp {
            Exprs::Boolean(b) => Ok(Values::Bool(b)),
            Exprs::Number(i) => Ok(Values::Int(i)),
            _ => Err("Cannot conver expression to value"),
        }
    }
}
#[derive(Debug)]
struct SignatureType { // either an argument or return
    arg: Vec<Type>,
    retur: Type,
}

pub struct VariableValues {
    scope_layer: i32,
    func_table: HashMap<i32, HashMap<String, SignatureType>>,
    var_table: HashMap<i32, HashMap<String, Values>>,
    func_scope: Vec<i32>,
}

pub fn evaluate(result: Result<Type, String>) {
    type_check_test(result);
    fn type_check_test(result: Result<Type, String>) {
        if result.is_err() {
            println!("{}", result.unwrap_err());
        } else {
            evaluate_program(); // this will act as statements evaluator
        }
    }
    fn expr_eval(expr: Box<Exprs>, scope: &mut VariableValues) -> Values{
        match *expr {
            Exprs::Boolean(b) => Values::Bool(b),
            Exprs::Number(i) => Values::Int(i),
            Exprs::Id(id) => match scope.get_symbol(&id) {
                Some(var) => match var.value {
                    Values::Int(i) => Exprs::Number(i),
                    Values::Bool(b) => Exprs::Bool(b),
                },
                None => panic!("Undefined var: {}", *var),
            },
            Exprs::Op(exp1, op, exp2) => {
                let left_exp = expr_eval(exp1, scope);
                let right_exp = expr_eval(exp2, scope);
                match op {
                    Op::Add => left_exp + right_exp,
                    Op::Sub => left_exp - right_exp,
                    Op::Mul => left_exp * right_exp,
                    Op::Div => left_exp / right_exp,
                    Op::Geq => Values::Bool(left_exp >= right_exp), 
                    Op::Leq => Values::Bool(left_exp <= right_exp),
                    Op::Gtr => Values::Bool(left_exp > right_exp),
                    Op::Lss => Values::Bool(left_exp < right_exp),
                    Op::Eq => Values::Bool(left_exp == right_exp),
                }
            }
        }
    }

    fn evaluate_program() {
        
    }
}

