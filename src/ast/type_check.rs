
use std::collections::HashMap;

use crate::ast::*;
//TODO: Fix check for Id in expr_check.
//TODO: I'll need something, maybe a struct, for scope handling!
//      Hashmaps is recommended so go with that. Maybe maps x2?

type Error = String;

fn expr_check(expr: Box<Exprs>, scope: &mut Scope) -> Result<Type, Error> {
    match *expr {
        ast::Exprs::Boolean(_) => Ok(ast::Type::Bool),
        ast::Exprs::Number(_) => Ok(ast::Type::I32),
        ast::Exprs::Id(id) => scope.get_symbol(&id), // Go through depending on type. This means early reterun though.
        ast::Exprs::Op(e1,o,e2) => {
            let recurExpr1 = expr_check(e1, scope);
            let recurExpr2 = expr_check(e2, scope);
            if recurExpr1.is_err() {
                return recurExpr1;
            }
            if recurExpr2.is_err() {
                return recurExpr2;
            }
            let type1 = recurExpr1.unwrap(); // tar ut typerna om inget err.
            let type2 = recurExpr2.unwrap();
            match o { 
                ast::Op::Gtr | ast::Op::Lss | ast::Op::Geq | ast::Op::Leq => {
                    if type1 == ast::Type::I32 && type2 == ast::Type::I32 {
                        return Ok(ast::Type::Bool)
                    } 
                    return Err(format!("Could not do {:?} for types {:?} and {:?}", o, e1,e2))
                },
                ast::Op::Add | ast::Op::Sub | ast::Op::Mul | ast::Op::Div => {
                    if type1 == ast::Type::I32 && type2 == ast::Type::I32 {
                        return Ok(ast::Type::Bool)
                    }
                    return Err(format!("Cound not do {:?} for types {:?} and {:?}", o, e1,e2))
                },
                ast::Op::And | ast::Op::Or => {
                    if type1 == ast::Type::Bool && type2 == ast::Type::Bool {
                        return Ok(ast::Type::Bool)
                    }
                    return Err(format!("Could not do {:?} for types {:?} and {:?}", o, e1,e2))
                },
                ast::Op::Eq => {
                    if type1 == type2 {
                        Ok(ast::Type::Bool)
                    }
                    return Err(format!("Could not do {:?} for types {:?} and {:?}", o, e1,e2))
                },
                ast::Op::Neq => {
                    if type1 != type2 {
                        Ok(ast::Type::Bool)
                    }
                    return Err(format!("Could not do {:?} for types {:?} and {:?}", o, e1,e2))
                },
                _ => Err("NONE OF OP'S".to_string())
            }
        },
        ast::Exprs::NotOp(ast::Op::Not ,e) => {
            let recEx1 = expr_check(e, scope);
            if recEx1.is_err() {
                return recEx1;
            }
            let typ = recEx1.unwrap();
            if typ == ast::Type::Bool {
                return Ok(ast::Type::Bool)
            }
            return Err(format!("Expression {:?} do not support !", typ));
        },
        ast::Exprs::NotOp(ast::Op::Sub, e) => {
            let recEx1 = expr_check(e, scope);
            if recEx1.is_err() {
                return recEx1;
            }
            let typ = recEx1.unwrap();
            if typ == ast::Type::I32 {
                return Ok(ast::Type::I32)
            }
            return Err(format!("The type {:?} does not support negative inverting", typ))
        },
        ast::Exprs::FunctionCall(id, expressions) => {
            let mut arguments = vec![]; //instansiate an empty vector for arguments.
            for expr in expressions { // expr_check for each expr.
                let recur = expr_check(expr, scope);
                if recur.is_err() {
                    return recur;
                } 
                arguments.push(recur.unwrap()); //Push values into vector.
            }
            //Check that function is is this scope.
            let fScope = scope.get_func(&id, arguments);
            if fScope.is_err() {
                Err(format!("Function, {} not in this scope", fScope))
            }
            Ok(fScope.unwrap())
        },
        _ => Err(format!("Expression {:?} not checkable", *expr)),
    }
}

//For checking the if, else if, and else statement conditionals.
pub fn if_else_check(stmt: Box<ast::Statement>, scope: &mut Scope) -> Result<ast::Type, Error> {
    match *stmt {
        ast::Statement::Cond(ast::AllCond::If, Some(e), block, None) => {
            let rec = expr_check(e, scope);
            if rec.is_err() {
                Err(format!("Incorrect if statement, {:?}", rec))
            }
            else {
                //TODO: BLOCK TYPE CHECK
            }
        },
        ast::Statement::Cond(ast::AllCond::ElseIf, Some(e), block, Some(st)) => {
            let rec = expr_check(e, scope);
            if rec.is_err() {
                Err(format!("Incorrect else if statement, {:?}", rec))
            }
            else {
                //TODO: Block type check... ill need that everywhere so ill do it now.
            }
        }

    }
    
}
pub fn block_check(block: Box<ast::Statement>, scope: &mut Scope) -> Result<ast::Type, Error> {
    //Will need scope...need to do that before block.

    //First we enter block, ie go into new scope
    scope.addLayer();

    let opReturn = match *block {
        ast::Statement::Block(stmt, Some(ret)) => { //with  return
            //check that only statements are in the scope with the type of expl/impl return
        },
        ast::Statement::Block(stmt, None) => { //No implicit/explicit return
            //need only recurse to statement check.
        }
    };
    

    


    //check the returntype against the function explicit return

    //Exit scope layer when block finished.
    scope.backLayer();


    
}

pub fn function_check() -> Result<ast::Type, Error> {


    //The return statement should check against the block check...

}

pub fn statement_check(stmts: Vec<Box<Statement>>, scope: &mut Scope) -> Result<ast::Type, Error> {
    for stmt in stmts {
        match *stmt {
            ast::Statement::Assign(id, ex) => {
                let sAssign = scope.get_symbol(&id);
                if sAssign.is_err() {
                    sAssign
                } // cant oneline since it will panic at err.
                else {
                    let s2Assign = expr_check(ex, scope);
                    if s2Assign.is_err() {
                        s2Assign
                    }
                    Ok(ast::Type::Unit)
                }
            },
            ast::Statement::While(ex, block) => {
                //First check expression 
                let testEx = expr_check(ex);
                if testEx.is_err() {
                    testEx
                }
                else {
                    let typ = testEx.unwrap();
                    if typ == ast::Type::Bool {
                        let testBlock = block_check(block, scope);
                        if testBlock.is_err() {
                            testBlock
                        }
                        else {
                            let typ2 = testBlock.unwrap();//should be unit for block unless return
                            if typ2 == ast::Type::Unit {
                                Ok(ast::Type::Unit)
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
            ast::Statement::Let(m,id,)

        }
    }
}





pub struct Scope {
    scope_layer: i32, // Scope Layer identification
    table: HashMap<i32, HashMap<String, SignatureType>>, //Two maps with the signature type in one.
    symbolTable: HashMap<i32, HashMap<String, ast::Type>>,
    src: String,
}

struct SignatureType { // either an argument or return
    arg: Vec<ast::Type>,
    retur: ast::Type,
}

impl Scope {
    fn newScope(&mut self) -> Scope {
        let mut s = Scope{scope_layer: 0, table: HashMap::new(), symbolTable: HashMap::new(), src: src};
        s.table.insert(0, HashMap::new()); //new layer of hashmap to track next layer.
        s.symbolTable.insert(0, HashMap::new());
        s
    }
    fn addLayer(&mut self) {
        self.scope_layer += 1;
        self.table.insert(self.scope_layer, HashMap::new());
        self.table.insert(self.scope_layer, HashMap::new());
        //Must have a hashmap for handeling layers.

    }
    fn backLayer(&mut self) {
        self.table.remove(&self.scope_layer);
        self.table.remove(&self.scope_layer);
        self.scope_layer -= 1;
    }
    fn register(&mut self, id: &String, args: Vec<::ast::Type>, ret: ast::Type) {
        let scope_layer = self.table.get_mut(&self.scope_layer).unwrap();
        scope_layer.insert(id.to_string(), SignatureType{arg: arg, retur: retur});
    }
    fn register_symbol(&mut self, id: &String, ret: ast::Type) { // Add symbol, this will make shadowing/borrow possible aswell
        let scope_layer = self.symbolTable.get_mut(&self.scope_layer).unwrap();
        scope_layer.insert(id.to_string(), SignatureType{arg: arg, retur: retur});
    }





    fn get_symbol(&mut self, id: &String) -> Result<ast::Type, Error> { //Check variable in scope.
        let mut currentSymbol = self.scope_layer;
        while currentSymbol >= 0 {
            let scope_layer = self.symbolTable.get(&currentSymbol).unwrap();
            if scope_layer.contains_key(id) {
                return Ok(*scope_layer.get(id).unwrap());
            }
            currentSymbol -= 1;
        }
        Err(format!("Symbol, {:?} not i scope", id))
    }






    fn get_func(&mut self, id: &String, args: Vec<ast::Type>) -> Result<ast::Type, Error> {
        let mut currentfunc = self.scope_layer;
        while currentfunc >= 0 {
            let scope_layer = self.table.get(&currentfunc).unwrap();
            if scope_layer.contains_key(id) {
                let sign = scope_layer.get(id).unwrap();
                let matchset = args.iter().zip(sign.args.iter()).filter(|&(a,b)| a == b).count();
                if matchset == args.len() && matchset == sign.args.len() {
                    return Ok(sign.ret);
                }
            }
            currentfunc -= 1;
        }
        Err(format!("Function, {}({:?}) not in correct scope layer", id, args))
    }
    

} 
