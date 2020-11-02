// ast
use std::fmt;

#[derive(Debug,PartialEq, Clone)]
pub enum Exprs {
    Boolean(bool), // true/false
    Number(i32), // 3
    Id(String), // a(...)
    NotOp(Op, Box<Exprs>), // !
    Op(Box<Exprs>, Op, Box<Exprs>), // && ...
    FunctionCall(String, Vec<Box<Exprs>>), // a(x,y)
    DeRef(Box<Exprs>), // *
    Borrow(bool, Box<Exprs>), //bool for mut or not, &
    Str(String), // "hello"
}

impl fmt::Display for Exprs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Exprs::Number(i) => write!(f, "{}", i)?,
            Exprs::Id(s) => write!(f, "{}", s)?,
            Exprs::Boolean(b) => write!(f, "{}", b)?,
            Exprs::NotOp(not,exp) => write!(f, "{}{}",not,exp)?,
            Exprs::FunctionCall(s,vec) => { 
                write!(f, "{}", s)?;
                for (i,v) in vec.iter().enumerate() {
                    write!(f,"{}", v)?;
                    if i < vec.len()-1 {
                        write!(f, "")?;
                    }
                }
            }
            Exprs::Op(expr,op,expr2) => write!(f, "({} {} {})", expr, op, expr2)?,
            Exprs::DeRef(expr) => write!(f, "{}", expr)?,
            Exprs::Borrow(true,expr) => write!(f, "&mut {}",expr)?,
            Exprs::Borrow(false, expr) => write!(f, "&{}", expr)?,
            Exprs::Str(st) => write!(f,"\"{}\"", st)?,
        };
        Ok(())
    }
} 

#[derive(Debug,PartialEq, Copy, Clone)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
    Eq,
    Neq,
    Gtr,
    Lss,
    Geq,
    Leq,
    Not,
}
impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Op::Add => write!(f, "{}", "+")?,//
            Op::Sub => write!(f, "{}", "-")?,//
            Op::Mul => write!(f, "{}", "*")?,//
            Op::Div => write!(f, "{}", "/")?, //
            Op::And => write!(f, "{}", "&&")?, //
            Op::Or => write!(f, "{}", "||")?, //
            Op::Eq => write!(f, "{}", "==")?, //
            Op::Neq => write!(f, "{}", "!=")?, //
            Op::Gtr => write!(f, "{}", ">")?,//
            Op::Lss => write!(f, "{}", "<")?,//
            Op::Geq => write!(f, "{}", ">=")?,//
            Op::Leq => write!(f, "{}", "<=")?,//
            Op::Not => write!(f, "{}", "!")?,
        };
        Ok(())
    }
}

#[derive(Debug,PartialEq, Clone)]
pub enum Type {
    I32,
    Bool,
    Unit,
    Ref(bool, Box<Type>), // mut/not, &
    Str,

}
impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Bool => write!(f, "{}", "bool")?,
            Type::I32 => write!(f, "{}", "i32")?,
            Type::Unit => write!(f, "{}", "()")?,
            Type::Ref(true,ref typ) => write!(f, "&mut {}", typ)?,
            Type::Ref(false,ref typ) => write!(f, "&{}",typ)?,
            Type::Str => write!(f, "{}", "String")?,
        };
        Ok(())
    }
}

#[derive(Debug,PartialEq, Clone)]
pub enum Statement {
    Let(bool, String, Option<Type>, Option<Box<Exprs>>), //
    Cond(AllCond, Option<Box<Exprs>>, Box<Statement>, Option<Box<Statement>>), //
    Block(Vec<Box<Statement>>, Option<Box<Statement>>),
    While(Box<Exprs>, Box<Statement>), //
    Assign(String, Box<Exprs>), //
    Return(Box<Exprs>),
    //Exprs(Box<Exprs>),
    Function(String, Vec<Box<Statement>>, Option<Type>, Box<Statement>), //
    FuncArg(String, Type),
}
impl fmt::Display for Statement { //Statement with optional
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::Let(m, id,optype,opexp) => {
                write!(f, "let ")?;
                if *m {
                    write!(f, "mut ")?;
                }
                write!(f, "{}", id)?;
                if let Some(typ) = optype {
                    write!(f, ": {} ", typ)?;
                }
                if let Some(exp) = opexp {
                    write!(f, "= {};", exp)?;
                }
            }
            Statement::Return(expr) => {
                write!(f, "{}", expr)?;
            }
            Statement::Block(state,r)  => {
                write!(f, "{{\n")?;
                for (i,stmt) in state.iter().enumerate(){
                    write!(f, "        {}\n", stmt)?;
                    if i <state.len()-1 {
                        write!(f, ";\n")?;
                    }
                }
                if let Some(ret) = r {
                    write!(f, "{}",ret)?;
                }
                write!(f, "    }}")?;
            } 
            Statement::Cond(cond,opexpr,stmt,opstmt) => {
                write!(f, "{} ", cond)?;
                if let Some(e) = opexpr {
                    write!(f, "{} ", e)?;
                }
                write!(f, "{}", stmt)?;
                if let Some(s) = opstmt {
                    write!(f, "{}", s)?;
                }
            }
            Statement::While(expr,block) => {
                write!(f, "while {} {}", expr, block)?;
            }
            Statement::Assign(id,expr) => {
                write!(f, "{} = {};", id, expr)?;
            }
            Statement::Function(id, arg,optyp,block) => {
                write!(f, "fn {}(", id)?;
                for (i,a) in arg.iter().enumerate(){
                    write!(f, "{}", a)?;
                    if i < arg.len()-1 {
                        write!(f, ", ")?;
                    } 
                }
                write!(f, ") ")?;
                if let Some(t) = optyp {
                    write!(f, "-> {} ", t)?;
                }
                write!(f, "{}", block)?;
            }
            //Statement::Exprs(ex) => {
            //    write!(f, "{}", ex)?;
            //}
            Statement::FuncArg(id,typ) => {
                write!(f, "{}:{}", id, typ)?;
            }
        };
        Ok(())
    }
} 

#[derive(Debug,PartialEq, Clone, Copy)]
pub enum AllCond {
    If,
    Else,
    ElseIf,
}
impl fmt::Display for AllCond {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AllCond::If => write!(f, "{}", "if")?,
            AllCond::ElseIf => write!(f, "{}", "else if")?,
            AllCond::Else => write!(f, "{}", "else")?
        };
        Ok(())
    }
}
#[derive(Debug)]
pub enum Error {

}




