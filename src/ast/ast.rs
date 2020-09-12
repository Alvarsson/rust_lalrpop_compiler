// ast
use std::fmt;

#[derive(Debug)] // debug-attribut, ungefär som arv i Java
pub enum NumOrId {
    Num(i32),
    Id(String),
}
impl fmt::Display for NumOrId {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NumOrId::Num(i) => write!(f, "{}", i)?,
            NumOrId::Id(s) => write!(f, "{}", s)?,
        };
        Ok(())
    }
}

#[derive(Debug)]
pub enum Op {
    ADD,
    SUB,
    MUL,
    DIV,
    AND,
    OR,
    EQ,
    NEQ,
    GTR,
    LSS,
    GEQ,
    LEQ,
    NOT,
}
impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Op::ADD => write!(f, "{}", "+")?,
            Op::SUB => write!(f, "{}", "-")?,
            Op::MUL => write!(f, "{}", "*")?,
            Op::DIV => write!(f, "{}", "/")?,
            Op::AND => write!(f, "{}", "&&")?,
            Op::OR => write!(f, "{}", "||")?,
            Op::EQ => write!(f, "{}", "==")?,
            Op::NEQ => write!(f, "{}", "!=")?,
            Op::GTR => write!(f, "{}", ">")?,
            Op::LSS => write!(f, "{}", "<")?,
            Op::GEQ => write!(f, "{}", ">=")?,
            Op::LEQ => write!(f, "{}", "<=")?,
            Op::NOT => write!(f, "{}", "!")?,
        };
        Ok(())
    }
}

#[derive(Debug)]
pub enum Type {
    Bool,
    I32,
    Unit,
}
impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Bool => write!(f, "{}", "bool")?,
            Type::I32 => write!(f, "{}", "i32")?,
            Type::Unit => write!(f, "{}", "()")?,
        };
        Ok(())
    }
}

#[derive(Debug)]
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
//ArgVariable(String, Type),
pub enum ArgVariable {
    Argument(String, Type),
}
impl fmt::Display for ArgVariable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArgVariable::Argument(id,typ) => write!(f, "{}:{}", id,typ)?,
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum Statement {
    //  Name,   opt mutbool,    opt type,      opt expr,       opt type
    Let(String, Option<bool>, Option<Type>, Option<Box<Expr>>,Option<bool>),
    //         expression
    ReturnWith(Box<Expr>),
    //            expression
    ReturnWithout(Box<Expr>),
    // if/else/elseif,  op expr,     statementblock,      op statement   
    Cond(AllCond, Option<Box<Expr>>, Box<Statement>, Option<Box<Statement>>),
    //     statement
    Block(Vec<Box<Statement>>),
    //    expr        statementblock
    While(Box<Expr>, Box<Statement>), 
    //      id        expr
    Assign(String, Box<Expr>),
    //       id       vector of arg        op ret type   stmt block
    Function(String, Vec<Box<ArgVariable>>, Option<Type>, Box<Statement>),
    // 
    FunctionCall(String, Vec<Box<Exprs>>),
    
    
}

impl fmt::Display for Statement { //Statement with optional
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::Let(id, opmut,optype,opexpr,optype2) => {
                write!(f, "let {} ", id)?;
                if let Some(m) = opmut {
                    write!(f, "mut ")?;
                }
                if let Some(typ) = optype {
                    write!(f, ": {} ", typ)?;
                }
                if let Some(exp) = opexpr {
                    write!(f, "= {};", exp)?;
                }
                if let Some(typ) = optype2 {
                    write!(f, "= {};", typ)?;
                }
            }
            Statement::ReturnWith(expr) => {
                write!(f, "return {};", expr)?;
            }
            Statement::ReturnWithout(expr) => {
                write!(f, "{}", expr)?;
            }
            Statement::Block(state)  => {
                write!(f, "{{")?;
                for st in state.iter(){
                    write!(f, "{};", st)?;
                }
                write!(f, "}}")?;
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
                write!(f, "{} = {}", id, expr)?;
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
            Statement::FunctionCall(id,vec) => {
                write!(f, "{}", id)?;
                for(i,a) in vec.iter().enumerate(){
                    write!(f, "{}", a)?;
                    if i < vec.len()-1 {
                        write!(f, ", ")?;
                    }
                }
            }

        };
        Ok(())
    }
} 
#[derive(Debug)]
pub enum Exprs {
    Expressions(Box<Expr>)
}
impl fmt::Display for Exprs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Exprs::Expressions(e) => write!(f, "{}", e)
        }
    }
}

#[derive(Debug)]
pub enum Expr {
    Number(i32),
    Identifier(String),
    Op(Box<Expr>, Op, Box<Expr>),

}
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Number(i) => write!(f, "{}", i)?,
            Expr::Identifier(s) => write!(f, "{}", s)?,
            Expr::Op(expr,op,expr2) => write!(f, "({} {} {})", expr, op, expr2)?,

        };
        Ok(())
    }
} 







