
use std::collections::HashMap;

use crate::ast::*;

type Error = String;

// this will most probablt look alot like the type_checker.
//TODO: Implement the Notop Sub
#[derive(Clone)]
struct SymbolTags {
    symbolbase: Exprs,
    mutable: bool,
    referencedmut: bool,
    reference: bool,
    borrowed: bool,
    moved: bool,
}

pub struct Scope {
    scope_layer: i32, // Scope Layer identification
    table: HashMap<i32, HashMap<String, Signature>>, //Two maps with the signature type in one.
    symbolTable: HashMap<i32, HashMap<String, SymbolTags>>,
    func_scope: Vec<i32>,
    src: String,
}

#[derive(Debug)]
struct Signature { // either an argument or return
    arg: Vec<String>,
    block: Box<Statement>,
}
impl Scope {
    pub fn newScope(src: String) -> Scope {
        let mut s = Scope{scope_layer: 0, table: HashMap::new(), symbolTable: HashMap::new(), func_scope: vec![0], src: src};
        s.table.insert(0, HashMap::new()); //new layer of hashmap to track next layer.
        s.symbolTable.insert(0, HashMap::new());
        s
    }
    fn addLayer(&mut self, func_scope: bool) {
        self.scope_layer += 1;
        if func_scope { // push in the function scope 
            self.func_scope.push(self.scope_layer);
        }
        self.table.insert(self.scope_layer, HashMap::new());
        self.symbolTable.insert(self.scope_layer, HashMap::new());
        //Must have a hashmap for handeling layers.

    }
    fn backLayer(&mut self) {
        self.table.remove(&self.scope_layer);
        self.symbolTable.remove(&self.scope_layer);
        if self.func_scope.contains(&self.scope_layer) { // if that scope_layer id in func scope, pop dat shiet.
            self.func_scope.pop();
        }
        self.scope_layer -= 1;
    }
    fn register(&mut self, id: &String, args: Vec<String>, stmts: Statement) {
        let scope_layer = self.table.get_mut(&self.scope_layer).unwrap();
        let stmt_box = Box::new(stmts);
        scope_layer.insert(id.to_string(), Signature{arg: args.to_vec(), block: stmt_box});
    }
    fn register_symbol(&mut self, id: &String, retur: Exprs, mutable: bool) { // Instead of type i run with expression value, bool/int
        let scope_layer = self.symbolTable.get_mut(&self.scope_layer).unwrap();
        scope_layer.insert(id.to_string(),SymbolTags{
            symbolbase: retur,
            mutable: mutable,
            referencedmut: false,
            reference: false,
            borrowed: false,
            moved: false,
        });
    }

    // TODO: Write so that this returns expression value.
    fn get_symbol(&mut self, id: &String) -> Result<Exprs, Error> { //Check variable in scope.
        let mut currentSymbol = self.scope_layer; // current scope layer
        let func_scope = *self.func_scope.last().unwrap(); 
        while currentSymbol >= func_scope{
            let scope_layer = self.symbolTable.get(&currentSymbol).unwrap();
            if scope_layer.contains_key(id) {
                let symb = scope_layer.get(id).unwrap();
                
                return Ok(symb.symbolbase.clone());
            }
            currentSymbol -= 1;
        } 
        Err(format!("Symbol, {:?} not i scope", id))
    }
    fn get_func(&mut self, id: &String, args: Vec<Exprs>) -> Result<Exprs, Error> {
        let mut currentfunc = self.scope_layer;
        while currentfunc >= 0 {
            let scope_layer = self.table.get(&currentfunc).unwrap();
            if scope_layer.contains_key(id) {
                let sign = scope_layer.get(id).unwrap();
                let mut arg_data: HashMap<String, Exprs> = HashMap::new();
                for (count, str) in sign.arg.iter().enumerate() {
                    let str_clone = str.clone();
                    let arg_clone = args.get(count).unwrap().clone();
                    arg_data.insert(str_clone, arg_clone);
                }
                let result = func_eval(arg_data, sign.block.clone(), self);
                println!("Function {}, returns {:?}.", id, result);
                return result;
                //return func_eval(arg_data, sign.block.clone(), self);
            }
            currentfunc -= 1;
        }
        Err(format!("Function, {}({:?}) not in correct scope layer", id, args))
    }
    fn set_symb_val(&mut self, id: &String, expr: Exprs) { // set value of symbol/variable
        let mut current_scope = self.scope_layer;
        let func_scope = *self.func_scope.last().unwrap(); 
        while current_scope >= func_scope {
            let mut scope = self.symbolTable.get_mut(&current_scope).unwrap();
            if scope.contains_key(id) { // if id found in map
                let mut symb = scope.get_mut(id).unwrap();
                symb.symbolbase = expr.clone();
            }
            current_scope -= 1;
        }
    }
} 


fn expr_eval(expr: Box<Exprs>, scope: &mut Scope) -> Result<Exprs, Error>{
    match *expr {
        Exprs::Boolean(b) => Ok(*expr),
        Exprs::Number(i) => Ok(*expr),
        Exprs::Str(_) => Ok(*expr),
        Exprs::Id(id) => {
            let symb_exp = scope.get_symbol(&id);
            let exp = symb_exp.unwrap(); // TODO: Do i need to recursivly call expr_eval?
            let result = expr_eval(Box::new(exp), scope);
            return result;
        },
        
        Exprs::Op(exp1, op, exp2) => {
            let left_exp = expr_eval(exp1, scope).unwrap();
            let right_exp = expr_eval(exp2, scope).unwrap();
            match op {
                Op::Add => {
                    if let Exprs::Number(left) = left_exp {
                        if let Exprs::Number(right) = right_exp {
                            return Ok(Exprs::Number(left + right));
                        } else {
                            return Err(format!("Error at Add"));
                        }
                    } else {
                        return Err(format!("Error at Add."));
                    }
                },
                Op::Sub => {
                    if let Exprs::Number(left) = left_exp {
                        if let Exprs::Number(right) = right_exp {
                            return Ok(Exprs::Number(left - right));
                        } else {
                            return Err(format!("Error at Sub"));
                        }
                    } else {
                        return Err(format!("Error at Sub."));
                    }
                },
                Op::Mul => {
                    if let Exprs::Number(left) = left_exp {
                        if let Exprs::Number(right) = right_exp {
                            return Ok(Exprs::Number(left * right));
                        } else {
                            return Err(format!("Error at Mul"));
                        }
                    } else {
                        return Err(format!("Error at Mul."));
                    }
                },
                Op::Div => {
                    if let Exprs::Number(left) = left_exp {
                        if let Exprs::Number(right) = right_exp {
                            if right == 0 {
                                return Err(format!("Cant divide by zero"));
                            }
                            return Ok(Exprs::Number(left / right));
                        } else {
                            return Err(format!("Error at Div"));
                        }
                    } else {
                        return Err(format!("Error at Div."));
                    }
                },
                Op::Geq => {
                    if let Exprs::Boolean(left) = left_exp {
                        if let Exprs::Boolean(right) = right_exp {
                            return Ok(Exprs::Boolean(left >= right));
                        } else {
                            return Err(format!("Error at Geq"));
                        }
                    } else {
                        return Err(format!("Error at Geq."));
                    }
                }
                Op::Leq => {
                    if let Exprs::Boolean(left) = left_exp {
                        if let Exprs::Boolean(right) = right_exp {
                            return Ok(Exprs::Boolean(left <= right));
                        } else {
                            return Err(format!("Error at Leq"));
                        }
                    } else {
                        return Err(format!("Error at Leq."));
                    }
                },
                Op::Gtr => {
                    if let Exprs::Boolean(left) = left_exp {
                        if let Exprs::Boolean(right) = right_exp {
                            return Ok(Exprs::Boolean(left > right));
                        } else {
                            return Err(format!("Error at Gtr"));
                        }
                    } else {
                        return Err(format!("Error at Gtr."));
                    }
                },
                Op::Lss => {
                    if let Exprs::Boolean(left) = left_exp {
                        if let Exprs::Boolean(right) = right_exp {
                            return Ok(Exprs::Boolean(left < right));
                        } else {
                            return Err(format!("Error at Lss"));
                        }
                    } else {
                        return Err(format!("Error at Lss."));
                    }
                },
                Op::Eq => {
                    if let Exprs::Boolean(left) = left_exp {
                        if let Exprs::Boolean(right) = right_exp {
                            return Ok(Exprs::Boolean(left == right));
                        } else {
                            return Err(format!("Error at Eq"));
                        }
                    } else {
                        return Err(format!("Error at Eq."));
                    }
                },
                Op::Neq => {
                    if let Exprs::Boolean(left) = left_exp {
                        if let Exprs::Boolean(right) = right_exp {
                            return Ok(Exprs::Boolean(left != right));
                        } else {
                            return Err(format!("Error at Neq"));
                        }
                    } else {
                        return Err(format!("Error at Neq."));
                    }
                },
                Op::And => {
                    if let Exprs::Boolean(left) = left_exp {
                        if let Exprs::Boolean(right) = right_exp {
                            return Ok(Exprs::Boolean(left && right));
                        } else {
                            return Err(format!("Error at And"));
                        }
                    } else {
                        return Err(format!("Error at And."));
                    }
                },
                Op::Or => {
                    if let Exprs::Boolean(left) = left_exp {
                        if let Exprs::Boolean(right) = right_exp {
                            return Ok(Exprs::Boolean(left || right));
                        } else {
                            return Err(format!("Error at Or"));
                        }
                    } else {
                        return Err(format!("Error at Or."));
                    }
                },
                _ => Err("No Op's caugt expression".to_string())
            }
        },
        Exprs::NotOp(Op::Not, exp) => {
            let expression = expr_eval(exp, scope).unwrap();
            if let Exprs::Boolean(e) = expression {
                return Ok(Exprs::Boolean(!e));
            } else {
                return Err(format!("Error at ! operand"));
            }
        }, // TODO: ADD NOTOP SUB
        Exprs::FunctionCall(id,expressions) => {
            let mut arguments = vec![];
            scope.addLayer(false);
            for expr in expressions {
                let ret = expr_eval(expr, scope);
                if ret.is_err() {
                    scope.backLayer();
                    return ret;
                } else {
                    let r = ret.unwrap();
                    arguments.push(r);
                }
            }
            let res_func = scope.get_func(&id, arguments);
            scope.backLayer();
            if res_func.is_err() {
                return res_func;
            } else {
                let result = res_func.unwrap();
                Ok(result)
            }
        },
        // Wont need to handle borrow and deref Exprs!

        _ => Err(format!("Not able to evaluate expression: {:?}.",*expr))
    }
}

pub fn block_eval(block: Box<Statement>, scope: &mut Scope) -> Result<Exprs, Error> {
    scope.addLayer(false);
    let result = match *block {
        Statement::Block(stmt, Some(ret)) => {
            let st = evaluate_program(stmt, scope);
            if st.is_err() {
                st
            } else {
                if let Statement::Return(e) = *ret {
                    let ret = expr_eval(e, scope);
                    ret
                } else {
                    Err(format!("Error att block evaluation"))
                }
            }
        },
        Statement::Block(stmt, None) => {
            evaluate_program(stmt, scope)
        },
        _ => Err(format!("No block eval able, Error"))
    };
    scope.backLayer();
    result
}
pub fn func_eval(arg_map: HashMap<String, Exprs>, block: Box<Statement>, scope: &mut Scope) -> Result<Exprs, Error> {
    // get map of func id/val -> evaluate
    scope.addLayer(true);
    // register the symbols/variables
    for (id, ex) in arg_map.iter() {
        scope.register_symbol(&id, ex.clone(), false);
    }
    let result = block_eval(block, scope);
    //println!("Function, {}, returns {}.", id, result);
    scope.backLayer();
    return result;
}

pub fn condition_eval(stmt: Box<Statement>, scope: &mut Scope) -> Result<Exprs, Error> {
    match *stmt {
        Statement::Cond(AllCond::ElseIf, Some(ex), block, Some(op_next)) => {
            let ex_res = expr_eval(ex, scope);
            if ex_res.is_err() {
                ex_res
            } else {
                // eval the condition
                let ex_bool = ex_res.unwrap();
                if let Exprs::Boolean(b) = ex_bool {
                    if b {
                        block_eval(block, scope)
                    } else {
                        condition_eval(op_next, scope)
                    }
                } else {
                    Err(format!("Condition error"))
                }
            }
        },
        Statement::Cond(AllCond::ElseIf, Some(ex), block, None) => { // no next statement
            let ex_res = expr_eval(ex, scope);
            if ex_res.is_err() {
                ex_res
            } else {
                let value = ex_res.unwrap(); // value
                if let Exprs::Boolean(b) = value {
                    if b {
                        block_eval(block, scope)
                    } else {
                        Ok(Exprs::Unit)
                    }
                } else {
                    Err(format!("Condition error."))
                }
            }
        },
        Statement::Cond(AllCond::Else, None, block, None) => {
            block_eval(block, scope) // else only need to evaluate the block
        },
        _ => Err(format!("Cannot eval condition"))
    }
}

pub fn evaluate_program(stmts: Vec<Box<Statement>>, scope: &mut Scope) -> Result<Exprs, Error> {
    let vec_len = stmts.len();
    let mut counter = 1;
    let mut deref_statements = vec![];
    for stmt in stmts {
        deref_statements.push(*stmt); // get the derefed statements so we can traverse it instead.
    }
    for stmt in &deref_statements { // register the signature of func statements
        if let Statement::Function(id, vec,_,block) = stmt {
            let mut args = vec![];
            for symb in vec {
                if let Statement::FuncArg(id,_) = &**symb { // not interested in type but instead id
                    args.push(id.clone());
                }
            }
            scope.register(&id, args,*block.clone()); // register function layer
            
        }
    }

    for stmt in deref_statements {
        let last_element = counter == vec_len; // set bool for when last element is reacherd
        let stmt_result: Result<Exprs, Error> = match stmt {
            Statement::Assign(id, ex) => { 
                let s_assign = scope.get_symbol(&id);
                if s_assign.is_err() { // dont actually need error handler since type check.
                    s_assign
                } else {
                    let ex_eval = expr_eval(ex, scope); // get expr eval to variable
                    if ex_eval.is_err() {
                        ex_eval
                    }
                    else {
                        let res = ex_eval.unwrap(); // get expr val
                        scope.set_symb_val(&id, res);
                        Ok(Exprs::Unit)
                    }
                }
            },
            Statement::While(ex, block) => { // wont need borrow handling since its handled at internal parts
                loop { // while true, wait for break
                    let ex_res = expr_eval(ex.clone(), scope);
                    if ex_res.is_err() {
                        break;
                    } else {
                        let ex_value = ex_res.unwrap();
                        if let Exprs::Boolean(b) = ex_value {
                            if !b { // not true
                                break
                            } else {
                                block_eval(block.clone(), scope); // evaluate inside block
                            }
                        } else {
                            break;
                        }
                    }
                }
                Ok(Exprs::Unit)
            },
            Statement::Let(mutable, id,_,op_e) => {
                // no need for type check
                if let Some(ex) = op_e { // expression optional eval
                    let expr = *ex;
                    let ret = expr_eval(Box::new(expr), scope);
                    if ret.is_err() {
                        ret
                    } else {
                        scope.register_symbol(&id, ret.unwrap(), mutable);
                        Ok(Exprs::Unit) // return unit since let statement 
                    }
                } else {
                    Err(format!("Let expression error."))
                }
            },
            Statement::Function(id, vec, Some(t), block) => {
                // only need to return unit type, only a function check
                Ok(Exprs::Unit)
            },
            // No return type
            Statement::Function(id, vec,None, block) => {
                Ok(Exprs::Unit)
            },
            Statement::Cond(AllCond::If, Some(opEx),block,Some(opNext)) => { // 
                //let Some(ex) = opEx;
                let ex_eval = expr_eval(opEx, scope);
                if ex_eval.is_err() { // check the expression
                    ex_eval
                }
                else {// go on to block check
                    let ex_value = ex_eval.unwrap();
                    if let Exprs::Boolean(b) = ex_value {
                        if b { // if true need to eval the block
                            block_eval(block, scope)
                        } else { // otherwise eval next possible condition
                            condition_eval(opNext, scope)
                        }
                    } else {
                        Err(format!("Condition If Error"))
                    }
                }
            },
            //Without next statement 
            Statement::Cond(AllCond::If, Some(opEx), block,None) => {
                let ex_eval = expr_eval(opEx, scope);
                if ex_eval.is_err() {
                    ex_eval
                } else {
                    let ex_value = ex_eval.unwrap();
                    if let Exprs::Boolean(b) = ex_value {
                        if b { // if expression eval to true eval block
                            block_eval(block, scope)
                        } else {
                            Ok(Exprs::Unit)
                        }
                    } else {
                        Err(format!("Error at no next condition"))
                    }
                }
            },
            Statement::Block(_,_) => {
                block_eval(Box::new(stmt), scope) // run through block evaluate function
            },
            Statement::Exprs(exp) => {
                expr_eval(exp, scope)
            },
            Statement::Program(stmt_vec) => {
                evaluate_program(stmt_vec, scope)
            }

            _ => Err(format!("Error no caught statements")),
        };
        if stmt_result.is_err() {
            return stmt_result
        }
        /* if last_element {
            return stmt_result
        }  */
        
        counter += 1;
    }
    return Ok(Exprs::Unit)
        
}


