
use std::collections::HashMap;

use crate::ast::*;

type Error = String;

// this will most probablt look alot like the type_checker.

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
                /* if symb.moved {
                    return Err(format!("Cannot use, {} here since it has been moved", id))
                } */ // wont need since type_check is done with borrow
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
                
                return func_eval(arg_data, sign.block.clone(), self);
                /* let matchset = args.iter().zip(sign.arg.iter()).filter(|&(a,b)| a == b).count();
                if matchset == args.len() && matchset == sign.arg.len() {
                    return Ok(sign.retur.clone());
                } */ // wont need this either since we look at func arguments in dif way.
            }
            currentfunc -= 1;
        }
        Err(format!("Function, {}({:?}) not in correct scope layer", id, args))
    }
    fn symb_val(&mut self, id: &String, expr: Exprs) { // set value of symbol/variable
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

        // Wont need this since checked in type_check right
    //TODO: Comment this for user understanding.
    /* fn borrow_symb(&mut self, id: &String, mutable: bool) -> Result<Exprs, Error> {
        let mut current_scope = self.scope_layer;
        let func_scope = *self.func_scope.last().unwrap(); // get the func scope value
        while current_scope >= func_scope {
            let mut scope_layer = self.symbolTable.get_mut(&current_scope).unwrap();
            if scope_layer.contains_key(id) {
                let symb = scope_layer.get(id).unwrap();
                if mutable && symb.referencedmut {
                    return Err(format!("{}, is already borrowed as mutable", id))
                }
                if mutable && symb.borrowed {
                    return Err(format!("{}, is already borrowed as immutable", id))
                }
                if mutable && !symb.mutable {
                    return Err(format!("{}, not declared mutable thus cant be borrowed as mutable", id))
                }
                let mut symbol_type = symb.symbolbase.clone();
                if let Type::Ref(ref_mut,typ) = symbol_type {
                    if mutable && !ref_mut {
                        return Err(format!("{}'s value can't be borrowed as mutable", id))
                    }
                    symbol_type = *typ;
                }
                let mut new_symb = symb.clone();
                if mutable {
                    new_symb.referencedmut = true; // if mutable set ref as mut true.
                } else {
                    new_symb.borrowed = true; // else set as borrowed true.
                }
                let t = Type::Ref(mutable, Box::new(symbol_type));
                new_symb.symbolbase = t.clone(); 
                self.symbolTable.get_mut(&self.scope_layer).unwrap().insert(id.to_string(), new_symb);
                return Ok(t);
            }
            current_scope -=1;
        }
        Err(format!("The variable {}, not found in scope ", id))
    }
    fn is_moved(&mut self, expr: Exprs) -> Result<bool, Error>{
        if let Exprs::Id(id) = expr {
            self.move_owner(&id)
        } else {
            return Ok(false);
        }
    }
    fn move_owner(&mut self, id: &String) -> Result<bool, Error> {
        let mut current_scope = self.scope_layer;
        let func_scope = *self.func_scope.last().unwrap(); // grab last number in vector
        while current_scope >= func_scope { // while this scope layer is larger or equal to func_layer
            let scope = self.symbolTable.get_mut(&current_scope).unwrap();
            if scope.contains_key(id) { // look for id in scope hashmap
                let mut symb = scope.get_mut(id).unwrap();
                return match symb.symbolbase {
                    Type::Ref(_,_) => { // dont care about bool or type so might aswell go for _
                        Ok(false)
                    },
                    Type::Str => {
                        if symb.moved { // if moved true throw error
                            Err(format!("{}, already moved", id))
                        } else {
                            symb.moved = true;
                            Ok(true)
                        }
                    },
                    _ => Ok(false) // rest is false
                }
            }
            current_scope -= 1;
        }
        Err(format!("{}, not found in scope", id))
    } */
} 


fn expr_eval(expr: Box<Exprs>, scope: &mut Scope) -> Result<Exprs, Error>{
    match *expr {
        Exprs::Boolean(b) => Ok(Exprs::Boolean(b)),
        Exprs::Number(i) => Ok(Exprs::Number(i)),
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
        },
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
                return st;
            } else {
                if let Statement::Return(e) = *ret {
                    let ret = expr_eval(e, scope);
                    println!("Function returns: {:?}", ret); // prints the return value
                    return ret;
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
    return result;
}
pub fn func_eval(arg_map: HashMap<String, Exprs>, block: Box<Statement>, scope: &mut Scope) -> Result<Exprs, Error> {
    // get map of func id/val -> evaluate
    scope.addLayer(true);
    // register the symbols/variables
    for (id, ex) in arg_map.iter() {
        scope.register_symbol(&id, ex.clone(), false);
    }
    let result = block_eval(scope, block);
    scope.backLayer();
    return result;
}

pub fn condition_eval(stmt: Box<Statement>, scope: &mut Scope) -> Result<Exprs, Error> {
    match *stmt {
        Statement::Cond(AllCond::ElseIf, Some(ex), block, Some(op_next)) => {
            let ex_res = expr_eval(ex, scope);
            if ex_res.is_err() {
                return ex_res;
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
            
        },
        Statement::Cond(AllCond::Else, None, block, None) => {

        },
        _ => Err(format!("Cannot eval condition"))
    }
}

fn evaluate_program(stmts: Vec<Box<Statement>>, scope: &mut Scope) -> Result<Values, Error> {
    let vec_len = stmts.len();
    let mut counter = 1;
    let mut deref_statements = vec![];
    for stmt in stmts {
        deref_statements.push(*stmt); // get the derefed statements so we can traverse it instead.
    }
    for stmt in &deref_statements {
        if let Statement::Function(id, vec,o_typ,_) = stmt {
            let mut args = vec![];
            for symb in vec {
                if let Statement::FuncArg(_,typ) = &**symb {
                    args.push(typ.clone());
                }
            }
            if o_typ.is_some() { 
                let r = o_typ.as_ref().unwrap().clone();
                scope.register(&id, &args, r)
            } else {
                let r = Type::Unit;
                scope.register(&id, &args, r)
            }
        }
    }

    for stmt in deref_statements {
        let last_element = counter == vec_len;
        let stmt_result: Result<Type, Error> = match stmt {
            Statement::Assign(id, ex) => { // No borrow handle since internal parts do that.
                let s_assign = scope.get_symbol(&id);
                if s_assign.is_err() {
                    return s_assign
                } // cant one-line since it will panic at err.
                else {
                    let s2_assign = expr_check(ex, scope);
                    if s2_assign.is_err() {
                        return s2_assign
                    }
                    else {
                        return Ok(Type::Unit)
                    }
                }
            },
            Statement::While(ex, block) => { // wont need borrow handling since its handled at internal parts
                //First check expression 
                let test_ex = expr_check(ex, scope);
                if test_ex.is_err() {
                    return test_ex
                }
                else {
                    let typ = test_ex.unwrap();
                    if typ == Type::Bool {
                        let test_block = block_check(block, scope);
                        if test_block.is_err() {
                            return test_block
                        }
                        else {
                            let typ2 = test_block.unwrap();//should be unit for block unless return
                            if typ2 == Type::Unit {
                                return Ok(Type::Unit)
                            }
                            else {
                                return Err(format!("Not a Unit return, instead got: {}", typ2))
                            }
                        }
                    }
                    else {
                        return Err(format!("Not a Bool, instead got: {}", typ))
                    }
                }
            },
            Statement::Let(mutable, id,op_typ,op_e) => {
                if let Some(typ) = op_typ {
                    if let Some(ex) = op_e {
                        let expr = *ex;
                        let ex_clone = expr.clone();
                        let ret = expr_check(Box::new(expr), scope);
                        if ret.is_err() {
                            return ret;
                        } else {
                            let can_move = scope.is_moved(ex_clone);
                            if can_move.is_err() {
                                return Err(format!("Cannot move, erro: {}", can_move.unwrap_err()));
                            }
                            let t = ret.unwrap();
                            if typ == t {
                                scope.register_symbol(&id, t, mutable);
                                return Ok(Type::Unit);
                            } else {
                                return Err(format!("Expected expression type {}, but got {}", t, typ));
                            }
                        }
                    } else {
                        Err(format!("Let expression error."))
                    }
                } else {
                    Err(format!("Let type error."))
                }
            },
            
            Statement::Function(id, vec, Some(opTyp), block) => {
                scope.addLayer(true);

                for symb in vec {
                    if let Statement::FuncArg(id,typ) = *symb {
                        scope.register_symbol(&id, typ, false); // false as not mutable.
                    }
                }
                let retur = function_check(opTyp, block, scope);
                scope.backLayer();
                return retur;
            },
            // No return type
            Statement::Function(id, vec,None, block) => {
                scope.addLayer(true);
                for symb in vec {
                    if let Statement::FuncArg(id,typ) = *symb {
                        scope.register_symbol(&id, typ, false); // mutable as false
                    }
                }
                let retur = function_check(Type::Unit, block, scope);
                scope.backLayer();
                return retur;
            },
            Statement::Cond(AllCond::If, Some(opEx),block,Some(opNext),) => { // 
                //let Some(ex) = opEx;
                let ret = expr_check(opEx, scope);
                if ret.is_err() { // check the expression
                    return ret
                }
                else {// go on to block check
                    let retBlock = block_check(block, scope);
                    if retBlock.is_err() {
                        return retBlock
                    }
                    else { // go on to next statement
                        //let Some(next) = opNext;
                        let retNext = condition_check(opNext,scope);
                        if retNext.is_err() {
                            return retNext
                        }
                        else {
                            let block_type = retBlock.unwrap();
                            let next_type = retNext.unwrap();
                            if block_type != next_type {
                                return Err(format!("Missmatching types of block and statement, expected {:?} but got {:?}.", block_type, next_type));
                            } else {
                                Ok(next_type)
                            }
                        }
                    }
                }
            },
            //Without next statement 
            Statement::Cond(AllCond::If, Some(opEx), block,None) => {
                //let Some(ex) = opEx;
                let rec_ex = expr_check(opEx, scope);
                if rec_ex.is_err(){
                    return rec_ex
                }
                else {
                    return block_check(block, scope)
                }
            },
            Statement::Block(_,_) => {
                block_check(Box::new(stmt), scope)
            },

            _ => Err(format!("Error no caught statements")),
        };
        if stmt_result.is_err() {
            return stmt_result;
        }
        if last_element {
            return stmt_result;
        } else {
            let err_result = stmt_result.unwrap();
            if err_result != Type::Unit {
                return Err(format!("Return type not Unit, instead got: {:?}",err_result));
            }
        }
        counter += 1;
    }
    return Ok(Type::Unit)
        
}


