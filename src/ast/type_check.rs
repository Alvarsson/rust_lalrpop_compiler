
use std::collections::HashMap;

use crate::ast::*;

type Error = String;

fn expr_check(expr: Box<Exprs>, scope: &mut Scope) -> Result<Type, Error> {
    match *expr {
        Exprs::Boolean(_) => Ok(Type::Bool),
        Exprs::Str(st) => Ok(Type::Str),
        Exprs::Number(_) => Ok(Type::I32),
        Exprs::Id(id) => scope.get_symbol(&id), // Go through depending on type. This means early return though.
        Exprs::Op(e1,o,e2) => {
            let recur_expr1 = expr_check(e1, scope);
            let recur_expr2 = expr_check(e2, scope);
            if recur_expr1.is_err() {
                return recur_expr1;
            }
            if recur_expr2.is_err() {
                return recur_expr2;
            }
            let type1 = recur_expr1.unwrap(); // tar ut typerna om inget err.
            let type2 = recur_expr2.unwrap();
            match o { 
                Op::Gtr | Op::Lss | Op::Geq | Op::Leq => {
                    if type1 == Type::I32 && type2 == Type::I32 {
                        return Ok(Type::Bool)
                    } 
                    return Err(format!("Could not do {:?} for types {:?} and {:?}", o, type1,type2))
                },
                Op::Add | Op::Sub | Op::Mul | Op::Div => {
                    if type1 == Type::I32 && type2 == Type::I32 {
                        return Ok(Type::I32)
                    }
                    return Err(format!("Cound not do {:?} for types {:?} and {:?}", o, type1,type2))
                },
                Op::And | Op::Or => {
                    if type1 == Type::Bool && type2 == Type::Bool {
                        return Ok(Type::Bool)
                    }
                    return Err(format!("Could not do {:?} for types {:?} and {:?}", o, type1,type2))
                },
                Op::Eq => {
                    if type1 == type2 {
                        return Ok(Type::Bool)
                    }
                    return Err(format!("Could not do {:?} for types {:?} and {:?}", o, type1,type2))
                },
                Op::Neq => {
                    if type1 != type2 {
                        return Ok(Type::Bool)
                    }
                    return Err(format!("Could not do {:?} for types {:?} and {:?}", o, type1,type2))
                },
                _ => Err("NONE OF OP'S".to_string())
            }
        },
        Exprs::NotOp(Op::Not ,e) => {
            let rec_ex1 = expr_check(e, scope);
            if rec_ex1.is_err() {
                return rec_ex1;
            }
            let typ = rec_ex1.unwrap();
            if typ == Type::Bool {
                return Ok(Type::Bool)
            }
            return Err(format!("Expression {:?} do not support !", typ));
        },
        Exprs::NotOp(Op::Sub, e) => {
            let rec_ex1 = expr_check(e, scope);
            if rec_ex1.is_err() {
                return rec_ex1;
            }
            let typ = rec_ex1.unwrap();
            if typ == Type::I32 {
                return Ok(Type::I32)
            }
            return Err(format!("The type {:?} does not support negative inverting", typ))
        },
        Exprs::FunctionCall(id, expressions) => {
            let mut arguments = vec![]; //instansiate an empty vector for arguments.
            scope.addLayer(false);
            for expr in expressions { // expr_check for each expr.
                let expr_moved = expr.clone();
                let rec_expr = expr_check(expr, scope);
                if rec_expr.is_err() {
                    scope.backLayer();
                    return rec_expr;
                } 
                else {
                    let mv = scope.is_moved(*expr_moved);
                    if mv.is_err() {
                        scope.backLayer();
                        let move_err = mv.unwrap_err();
                        return Err(format!("{}", move_err));
                    }
                    arguments.push(rec_expr.unwrap());
                }
            }
            //Check that function is is this scope.
            let f_scope = scope.get_func(&id, arguments);
            scope.backLayer();
            return if f_scope.is_err() {
                let f_err = f_scope.unwrap_err();
                Err(format!("{}", f_err))
            } else {
                Ok(f_scope.unwrap())
            }
            
        },
        Exprs::Borrow(mutable,e) => { // borrow check expressions
            //check if id
            if let Exprs::Id(id) = *e {
                let ret = scope.get_symbol(&id); // symbol scope check
                if ret.is_err() { 
                    let ret_err = ret.unwrap_err();
                    Err(format!("{}", ret_err))
                } else {
                    let ret = scope.borrow_symb(&id, mutable); // symbol borrow check
                    if ret.is_err() {
                        let ret_err = ret.unwrap_err();
                        Err(format!("Error: {}", ret_err))
                    } else {
                        return ret;
                    }
                }
            } else {
                Err(format!("Not a variable, cant borrow"))
            }
        },
        Exprs::DeRef(exp) => {
            if let Exprs::Id(id) = *exp {
                let ret = scope.get_symbol(&id); // symbol scope check
                if ret.is_err() {
                    let ret_err = ret.unwrap_err();
                    Err(format!("{}", ret_err))
                } else {
                    let r = ret.unwrap();
                    if let Type::Ref(_, typ) = r { // check ref type correct
                        Ok(*typ)
                    } else {
                        Err(format!("{}, is not a reference, cant dereference", id))
                    }
                }
            } else {
                Err(format!("Can not deref expression: {}", *exp))
            }
        },
        _ => Err(format!("Expression {:?} not checkable", *expr)),
    }
}

//For checking the if, else if, and else statement conditionals.
pub fn condition_check(stmt: Box<Statement>, scope: &mut Scope) -> Result<Type, Error> {
    match *stmt {
        Statement::Cond(AllCond::ElseIf, Some(ex), block, Some(op_next)) => {
            let rec = expr_check(ex, scope);
            if rec.is_err() {
                return rec
            }
            else { // check block correctness
                let ret_block = block_check(block, scope);
                if ret_block.is_err(){
                    return ret_block
                }
                else { //check next condition
                    let ret_next = condition_check(op_next, scope);
                    if ret_next.is_err() {
                        return ret_next
                    }
                    else { // check matching types in conditions
                        let typ_block = ret_block.unwrap();
                        let typ_next = ret_next.unwrap();
                        if typ_block == typ_next {
                            Ok(typ_next)
                        }
                        else {
                            return Err(format!("Types of block and conditions does not match, {:?} & {:?} ",typ_block,typ_next));
                        }
                    }
                }
            }
        },
        //No next condition, only need to check expression and block
        Statement::Cond(AllCond::ElseIf, Some(ex), block,None) => {
            let rec = expr_check(ex, scope);
            if rec.is_err(){
                rec
            }
            else {
                return block_check(block, scope) // will throw err in itself.
            }
        },
        Statement::Cond(AllCond::Else, None, block, None) => {
            return block_check(block, scope)
        },
        _ => Err(format!("No condition check thrown, cannot check this."))

    }
    
}
pub fn block_check(block: Box<Statement>, scope: &mut Scope) -> Result<Type, Error> {
    //First we enter block, ie go into new scope
    scope.addLayer(false);
    let op_return = match *block {
        Statement::Block(stmt, Some(ret)) => { //with  return
            //check that only statements are in the scope with the type of expl/impl return
            let st = statement_check(stmt, scope);
            if st.is_err(){
                st
            }
            else { // check the return statement type
                if let Statement::Return(e) = *ret{
                    let ret = expr_check(e, scope);
                    return ret;
                }
                else {
                    Err(format!("Error at block return"))
                }
            }
        },
        Statement::Block(stmt, None) => { //No implicit/explicit return
            //need only recurse to statement check.
            statement_check(stmt, scope)
        },
        _ => Err(format!("Nothing caught in block, Error."))
    };

    //check the returntype against the function explicit return
    //Exit scope layer when block finished.
    scope.backLayer();
    op_return // return to match 


    
}
    //Check function return type with the block return type.
pub fn function_check(ret: Type, block: Box<Statement> ,scope: &mut Scope) -> Result<Type, Error> {
    let ret_value = block_check(block, scope);
    if ret_value.is_err() {
        return ret_value
    }
    else {
        let ret_type = ret_value.unwrap();
        if ret_type != ret {
            return Err(format!("Function return type doesn't match with block return {} != {}", ret, ret_type))   
        }
    }
    Ok(Type::Unit)
}


// Need to restruct since borrow scope needs value for ref, deref and such
//Let check
//Assign check
//While check
//Function check
//Cond check
//Block check

pub fn statement_check(stmts: Vec<Box<Statement>>, scope: &mut Scope) -> Result<Type, Error> {
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
                    s_assign
                } 
                else {
                    let s2_assign = expr_check(ex, scope);
                    if s2_assign.is_err() {
                        s2_assign
                    }
                    else {
                        Ok(Type::Unit)
                    }
                }
            },
            Statement::While(ex, block) => { // wont need borrow handling since its handled at internal parts
                //First check expression 
                let test_ex = expr_check(ex, scope);
                if test_ex.is_err() {
                    test_ex
                }
                else {
                    let typ = test_ex.unwrap();
                    if typ == Type::Bool {
                        let test_block = block_check(block, scope);
                        if test_block.is_err() {
                            test_block
                        }
                        else {
                            let typ2 = test_block.unwrap();//should be unit for block unless return
                            if typ2 == Type::Unit {
                                Ok(Type::Unit)
                            }
                            else {
                                Err(format!("Not a Unit return, instead got: {}", typ2))
                            }
                        }
                    }
                    else {
                        Err(format!("Not a Bool, instead got: {}", typ))
                    }
                }
            },
            Statement::Let(mutable, id, None, op_e) => {
                if let Some(ex) = op_e {
                    let expr = *ex;
                    let ex_clone = expr.clone();
                    let ret = expr_check(Box::new(expr), scope);
                    if ret.is_err() {
                        ret
                    } else {
                        let can_move = scope.is_moved(ex_clone);
                        if can_move.is_err() {
                            Err(format!("Cannot move, error: {}", can_move.unwrap_err()))
                        } else {
                            scope.register_symbol(&id, ret.unwrap(), mutable);
                            Ok(Type::Unit)
                        }
                    }
                } else {
                    Err(format!("Let with no type expression error"))
                }
            },
            Statement::Let(mutable, id,op_typ,op_e) => {
                if let Some(typ) = op_typ {
                    if let Some(ex) = op_e {
                        let expr = *ex;
                        let ex_clone = expr.clone();
                        let ret = expr_check(Box::new(expr), scope);
                        if ret.is_err() {
                            ret
                        } else {
                            let can_move = scope.is_moved(ex_clone);
                            if can_move.is_err() {
                                Err(format!("Cannot move, erro: {}", can_move.unwrap_err()))
                            } else {
                                let t = ret.unwrap();
                                if typ == t {
                                    scope.register_symbol(&id, t, mutable);
                                    Ok(Type::Unit)
                                } else {
                                    Err(format!("Expected expression type {}, but got {}", t, typ))
                                }
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
                retur
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
                retur
            },
            Statement::Cond(AllCond::If, Some(opEx),block,Some(opNext),) => { // 
                //let Some(ex) = opEx;
                let ret = expr_check(opEx, scope);
                if ret.is_err() { // check the expression
                    ret
                }
                else {// go on to block check
                    let retBlock = block_check(block, scope);
                    if retBlock.is_err() {
                        retBlock
                    }
                    else { // go on to next statement
                        //let Some(next) = opNext;
                        let retNext = condition_check(opNext,scope);
                        if retNext.is_err() {
                            retNext
                        }
                        else {
                            let block_type = retBlock.unwrap();
                            let next_type = retNext.unwrap();
                            if block_type != next_type {
                                Err(format!("Missmatching types of block and statement, expected {:?} but got {:?}.", block_type, next_type))
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
                    rec_ex
                }
                else {
                    block_check(block, scope)
                }
            },
            Statement::Block(_,_) => {
                block_check(Box::new(stmt), scope)
            },
            Statement::Program(stmt_vec) => {
                statement_check(stmt_vec, scope)
            },
            Statement::Exprs(expr) => {
                expr_check(expr, scope);
                Ok(Type::Unit)
            }

            _ => Err(format!("Error no caught statements")),
        };
        if stmt_result.is_err() {
            return stmt_result
        }
        if last_element {
            return stmt_result
        } else {
            let err_result = stmt_result.unwrap();
            if err_result != Type::Unit {
                return Err(format!("Return type not Unit, instead got: {:?}",err_result))
            }
        }
        counter += 1;
    }
    return Ok(Type::Unit)
}
    


#[derive(Clone)]
struct SymbolTags {
    symbolbase: Type,
    mutable: bool,
    referencedmut: bool,
    reference: bool,
    borrowed: bool,
    moved: bool,
}

pub struct Scope {
    scope_layer: i32, // Scope Layer identification
    table: HashMap<i32, HashMap<String, SignatureType>>, //Two maps with the signature type in one.
    symbolTable: HashMap<i32, HashMap<String, SymbolTags>>,
    func_scope: Vec<i32>,
    src: String,
}

#[derive(Debug)]
struct SignatureType { // either an argument or return
    arg: Vec<Type>,
    retur: Type,
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
    fn register(&mut self, id: &String, args: &Vec<Type>, ret: Type) {
        let scope_layer = self.table.get_mut(&self.scope_layer).unwrap();
        scope_layer.insert(id.to_string(), SignatureType{arg: args.to_vec(), retur: ret});
    }
    fn register_symbol(&mut self, id: &String, retur: Type, mutable: bool) { // Add symbol
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

    fn get_symbol(&mut self, id: &String) -> Result<Type, Error> { //Check variable in scope.
        let mut currentSymbol = self.scope_layer; 
        let func_scope = *self.func_scope.last().unwrap();
        while currentSymbol >= func_scope{
            let scope_layer = self.symbolTable.get(&currentSymbol).unwrap();
            if scope_layer.contains_key(id) {
                let symb = scope_layer.get(id).unwrap();
                if symb.moved {
                    return Err(format!("Cannot use, {} here since it has been moved", id))
                }
                return Ok(symb.symbolbase.clone());
            }
            currentSymbol -= 1;
        } 
        Err(format!("Symbol, {:?} not i scope", id))
    }

    fn get_func(&mut self, id: &String, args: Vec<Type>) -> Result<Type, Error> {
        let mut currentfunc = self.scope_layer;
        while currentfunc >= 0 {
            let scope_layer = self.table.get(&currentfunc).unwrap();
            if scope_layer.contains_key(id) {
                let sign = scope_layer.get(id).unwrap();
                let matchset = args.iter().zip(sign.arg.iter()).filter(|&(a,b)| a == b).count();
                if matchset == args.len() && matchset == sign.arg.len() {
                    return Ok(sign.retur.clone());
                }
            }
            currentfunc -= 1;
        }
        Err(format!("Function, {}({:?}) not in correct scope layer", id, args))
    }

    fn borrow_symb(&mut self, id: &String, mutable: bool) -> Result<Type, Error> {
        let mut current_scope = self.scope_layer; // current scope layer
        let func_scope = *self.func_scope.last().unwrap(); // current function scope layer
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
                if let Type::Ref(ref_mut,typ) = symbol_type { // check if is a reference but and mutable.
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
    }
} 



