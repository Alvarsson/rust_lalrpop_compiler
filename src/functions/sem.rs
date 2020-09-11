

//Semantic structure of things such as functions, if, let, etc...
pub enum Sem {
    Var(String),
    OpCode(Box<Sem>, Op, Box<Sem>),
    Let(String, Option<Type>, bool,Box<Sem>, Option<Box<Sem>>),
    Return(Box<Sem>, Option<Box<Sem>>), //Expr then another instr.
    If(Box<Sem>, Box<Sem>, Option<Box<Sem>>, Option<Box<Sem>>), //First a condition, {body of}  TODO
    Else(Box<Sem>, Option<Box<Sem>>), //{Body of} then... Do i need this?
    While(Box<Sem>, Box<Sem>, Option<Box<Sem>>),
    Bool(bool),
    Number(i32),

}