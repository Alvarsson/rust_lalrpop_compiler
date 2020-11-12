# Home Exam D7050E

- Fork this repo and put your answers (or links to answers) in THIS file.

## Your repo

https://github.com/Alvarsson/rust_lalrpop_compiler/tree/restart
Branch: restart

## Your syntax

- Give an as complete as possible EBNF grammar for your language.

- Give an example that showcases all rules of your EBNF. The program should "do" something as used in the next exercise.

- For your implementation, show that your compiler successfully accepts the input program.

- Give a set of examples that are syntactically illegal, and rejected by your compiler.

- Compare your solution to the requirements (as stated in the README.md). What are your contributions to the implementation.

Program
```ebnf
    : Stmt+
    ;
```
Stmt
```ebnf
    : Let ";"
    | Cond ";"?
    | While ";"?
    | Assign ";"?
    | Function ";"?
    | Block ";"?
    | Exprs ";"? 
    ;
```
Let
```ebnf
    : "let" "mut"? Id (":" Type)? ("=" Exprs)?
    ;
```
Cond
```ebnf
    : "if" Exprs Block NextCond?
    ;
```
NextCond
```ebnf
    : ElseIf
    | Else
    ;
```
ElseIf
```ebnf
    : "else if" Exprs Block NextCond?
    ;
```
Else
```ebnf
    : "else" Block
    ;
```
Block
```ebnf
    : "{" Stmt* Return? "}" 
    ;
```
While
```ebnf
    : "while" Exprs Block
    ;
```
Assign
```ebnf
    : Id "=" Exprs?
    ;
```
Return
```ebnf
    : Exprs
    | "return" Exprs ";"?
    ;
```
Function
```ebnf
    : "fn" Id "(" FuncMacro ")" ("->" Type)? Block
    | "fn" Id "()" ("->" Type)? Block
    ;
```
FuncMacro
```ebnf
    : FuncArgSep<FuncArg>
    ;
```
FuncArgSep
```ebnf
    : (<arg> ",")* arg?
    ;
```
FuncArg
```ebnf
    :Id ":" Type
    ;
```
FunctionCall
```ebnf
    : Id "(" CallMacro ")"
    | Id "()"
    ;
```
CallMacro
```ebnf
    : FuncArgSep<Exprs>
    ;
```
Type
```ebnf
    : "bool"
    | "i32"
    | "()"
    | "String"
    | "&" Type
    | "&mut" Type
    ;
```
NotOp
```ebnf
    : "!"
    | "-"
    ;
```
FacOp
```ebnf
    : "*"
    | "/"
    ;
```
ExpOp
```ebnf
    : "+"
    | "-"
    ;
```
LogOp
```ebnf
    : "&&"
    | "||"
    ;
```
RelOp
```ebnf
    : "=="
    | "!="
    | ">"
    | "<"
    | ">="
    | "<="
    ;
```
Exprs
```ebnf
    : NotOp Expr
    | Expr
    ;
```
Expr
```ebnf
    : Expr LogOp Factor
    | Expr RelOp Factor
    | Expr ExpOp Factor
    | Factor
    ;
```
Factor
```ebnf
    : Factor FacOp Term
    | Term
    ;
```
Term
```ebnf
    : Bool
    | Num
    | Id
    | Str
    | "&" Term
    | "&mut" Term
    | "*" Term
    | FunctionCall
    | "(" Exprs ")"
    ;
```
Num
```ebnf
    : r"[0-9]+"
    ;
```
Id
```ebnf
    : r"([a-z_]|[A-Z])([a-z]|[A-Z]|[0-9]|_)*"
    ;
```
Str
```ebnf
    : r"'(.*)'"
    ;
```
Bool
```ebnf
    : "true"
    | "false"
    ;
```
Showcase
```rust
fn main() {
    fn test(a: &mut String) {
        let mut b = 'oj';
        let mut e = &mut b;
        test1(&b)
    }
    test();
    fn test2(per: &i32) -> i32 {
        let axel: i32 = 10 + 2 * 3;
        axel = -1 - (1 - 1);
        return axel + per;
    }
    fn test3(foo: bool) -> bool {
        if foo && true || false {
            let a = test2(5);
            while ( a < 10) {
                a = a + 1;
            }
            return true;
        }
        return 5 > 7;
    }
}
main();
```
Inputing the code above into a string, say "test".
Set output from,
```rust
ProgramParser::new().parse(test);
```
to variable. Printing it gives:
```rust
Ok(Program([Function("main", [], None, Block([Function("test", [FuncArg("a", Ref(true, Str))], None,
Block([Let(true, "b", None, Some(Str("\'oj\'"))), Let(true, "e", None, Some(Borrow(true, Id("b"))))],
Some(Return(FunctionCall("test1", [Borrow(false, Id("b"))]))))), Exprs(FunctionCall("test", [])),
Function("test2", [FuncArg("per", Ref(false, I32))], Some(I32), Block([Let(false, "axel", Some(I32),
Some(Op(Number(10), Add, Op(Number(2), Mul, Number(3))))), Assign("axel", NotOp(Sub, Op(Number(1),
Sub, Op(Number(1), Sub, Number(1)))))], Some(Return(Op(Id("axel"), Add, Id("per")))))),
Function("test3", [FuncArg("foo", Bool)], Some(Bool), Block([Cond(If, Some(Op(Op(Id("foo"),
And, Boolean(true)), Or, Boolean(false))), Block([Let(false, "a", None,
Some(FunctionCall("test2", [Number(5)]))), While(Op(Id("a"), Lss, Number(10)),
Block([Assign("a", Op(Id("a"), Add, Number(1)))], None))],
Some(Return(Boolean(true)))), None)], Some(Return(Op(Number(5), Gtr, Number(7))))))], None)), Exprs(FunctionCall("main", []))]))
```
Syntactically illegal input examples:

This is illegal since id "1main" can't start with number.
```rust
fn 1main() {
    fn tjo() -> i32 {
        let a = 6;
    } 
}
```
This is illegal since we are missing a curly bracket that should close the inner function.
```rust
fn main() {
    fn tjo() -> i32 {
        let a = 6;
     
}
```
The showcase and EBNF above defines a subset of the complete Rust language. In comparison the course requirements the following has been met:
- Function definitions with both explicit and implicit return types.
- Statements: let, assign, if, else if, else, while, block, expressions, return(implicit/explicit).
- Expressions handle function calls, which can act as both explicit or implicit returns.
- Primitive types: boolean and i32.
- Operands and logical operands with correct evaluated precedence.
- Borrowing symbols for allowed statements. 

Future implementations:
- Error handling, including trace back to error location.
- More primitive types.

## Your semantics

- Give a (simplified) Structural Operational Semantics (SOS) for your language. You don't need to detail rules that are similar (follow the same pattern). Regarding variable environment (store) you may omit details as long as the presentation is easy to follow.

- Explain (in text) what an interpretation of your example should produce, do that by dry running your given example step by step. Relate back to the SOS rules. You may skip repetitions to avoid cluttering.

- For your implementation, give a program (or set of test programs) that cover all the semantics of your language that you have successfully implemented. (Maybe a subset of the input language accepted by the grammar.)

- Compare your solution to the requirements (as stated in the README.md). What are your contributions to the implementation.

### My semantics
The granularity of the SOS is in small-step, this allows for reduction for each step. 
A transition rule is written as:

![](../restart/images/transRule.png)

Using the following symbols to describe the Structural Operational Semantics:
- e ∈ expression
- x ∈ variable
- op ∈ Op
- ⇓, evaluates
- σ, state
- σ', derived state

Expressions can be the following:
- b ∈ Bool 
- n ∈ Num
- id ∈ Id
- st ∈ Str
- fc ∈ FunctionCall 

### Let sequence
![](../restart/images/letimg.png)

examples:
```rust
let a : i32 = 5;
let b : bool = false;
let c : i32 = func();
```
For let statements, state will change to derived state if variable already is declared. Otherwise is set to the state.

### Arithmetic expressions operands
The choice of arithmetic operands are
- "+", addition
- "-", subtraction
- "*", multiplication
- "/", division

![](../restart/images/ariOp.png)

examples:
```rust
 4 + 5;
 6 / 2;
```
### Boolean expressions operands
The choice of arithmetic operands are
- "==", equal to
- "!=", not equal to
- ">", larger than
- "<", smaller than
- ">=", larger or equal to
- "<=", smaller or equal to 

![](../restart/images/boolOp.png)

examples:
```rust
 5 < 7;
 2 == 2;
 b != 9;
```
### General command sequence
![](../restart/images/cmdSeq.png)

Close to the let statement semantic, executing the first, then the second command as to not loose the intermediate derived state.

examples:
```rust
let x = 9;
func();
```
### Function sequence
Basic function sequence from one state to derived state in the context of the called function.

![](../restart/images/funcSeq.png)

examples:
```rust
fn main() {
    func();
}
```
### Conditionals true and false
![](../restart/images/condT.png);

![](../restart/images/condF.png)

example:
```rust
if a > 6 {
    c = c + 5;
}
```
### While-loop
For while-false exiting with same state.

![](../restart/images/whileF.png);

And for while-true, concluding in derived state since block code executes.

![](../restart/images/whileT.png);

example:
```rust
while a > 6 {
    a -= 1;
}
```
### Return statement
Explicit return has the semantic,

![](../restart/images/retE.png),

which brings the return expression with the state.

Implicit return has the semantic,

![](../restart/images/retI.png),

but this does not carry the expression with the state.

### Assign statement
Since the variable has no state change and only moves the expression with the state we get conclusion,

![](../restart/images/ass.png)

example:
```rust
x = 5; 
x = b;
```

## Your type checker

- Give a simplified set of Type Checking Rules for your language (those rules look very much like the SOS rules, but over types not values). Also here you don't need to detail rules that are similar (follow the same pattern).

- Demonstrate each "type rule" by an example. You may use one or several "programs" to showcase where rules successfully apply.

- For your implementation, give a set of programs demonstrating that ill-typed programs are rejected, connect back to the Type Checking Rules to argue why these are illegal and thus should be rejected.

- Compare your solution to the requirements (as stated in the README.md). What are your contributions to the implementation.

### My type checker semantics

The types defined as possible returns are:

- bool
- i32
- unit -> "()"

### Let sequence

Let statements in the type checker is divided into type i32 and type bool.

Type i32, carrying the type with the state:

![](../restart/images/typeLeti.png).

example:
```rust
let x : i32 = 5; 
let x = b; // given that b must be of type i32.
```

Type bool does the same and carries the type bool with the state:

example:
```rust
let x : bool = false; 
let x = b; // given that b must be of type bool.
```

### Type i32 operands

Obviously these must evaluate to type i32 and the same operands is in play as the arithmetic operands.

![](../restart/images/typeIop.png).

example:
```rust
2-9; 
5*9;
```

For boolean type operands, the same operand symbols as in boolean expression operands is true and follows the same semantics as for
type i32 operands above.

example:
```rust
2 < 5; 
8 == 8;
```

### Conditional statements

The conditional statements are the same for both true and false regarding the semantics.

![](../restart/images/typeIf.png).

example:
```rust
if true {
    ...
}
```

### While statements

Just like the conditionals, while statement semtantics are the same for both true and false.

![](../restart/images/typeWhile.png).

The derived state will be evaluated as long as condition is of type boolean.

example:
```rust
while 5 > i {
    ...
}
```

### Return statements

The return statements for either type i32 or bool has the same semantics but obviously resulting in different types.

Their semantics are the following but foor bool we switch the i32 to bool.

![](../restart/images/typeRet.png).

examples:
```rust
return 5;
return false;
```

### Assign statement

As the previous statement semantics the assign types have the same semantic form.

![](../restart/images/typeAss.png).

The conclusion will be that the state carries the variable type.

examples:
```rust
x = true;
x = 5 + 5;
```

### Unit type 

The default return type of function is unit.

![](../restart/images/typeUnit.png).

examples:
```rust
fn main() -> () {
    func();
}
fn main() { // default to return unit
    ...
}
```

The function return type is held in the scope(environment) as to check that the correct block/layer returns the asked for type.

The type checker evaluates for each practical requirement in the course, as well as with borrowing references.

### Illegal examples and why

This function has the assigned return type set to i32 but the function block returns a unit type.

example Function return:
```rust
fn main() -> i32 {
    func();
}
```

Can't assign a variable to the unit type, must be either type bool or i32 in the type checker.

example Assign:
```rust
let x : i32 = 5;
x = ();
```

The while condition needs to be of type bool. Can't be a terminal number or an expression.

example While:
```rust
while 5 + 3 {
    let a = 5;
    ...
}
```

Can't evaluate the if conditional to the type unit as a correct value.

example If
```rust
if () {
    let a = 5;
    ...
}
```

Arithmetic expression must be of the same type. Can not use arithmetic operator between different types. 

example Arithmetic expression:
```rust
2-true; 
5 < false;
```

The Let statement with a explicit variable type must evaluate to the same type.

example Let:
```rust
let x : i32 = true; 
```

## Your borrrow checker

- Give a specification for well versus ill formed borrows. (What are the rules the borrow checker should check).

- Demonstrate the cases of ill formed borrows that your borrow checker is able to detect and reject.

- Compare your solution to the requirements (as stated in the README.md). What are your contributions to the implementation.

### Well formed borrows

Well formed borrows follows the general syntax that Rust implements. Some examples are:

example Good borrows, borrow-mut and dereference:
```rust
fn borrow_function() {
    let b = 'test';
    let c= &b;
}
fn borrow_mut_function() {
    let mut b = 'test':
    let mut c = &mut b;
}
fn deref_function(b: &i32) {
    return *b + 4;
}
```

### Ill formed borrows

Can only borrow a variable.

example Ill formed borrows:
```rust
fn test() {
    let s = &'hej';
} 
```

Can't borrow as mutable since s is not declared as mutable.

example Ill formed mutable borrows:
```rust
fn test() {
    let s = 'hej';
    let b = &mut s;
} 
```

Can't dereference variable that isn't a reference.

example Ill formed dereference:
```rust
fn test() {
    let s = 'hej';
    let b = *s;
} 
```

The examples above demonstrates a general type of ill formed borrowing and dereferencing that the borrow checker detects and reject.

The borrow checker, just as with the type checker and interpreter, is included in scope layer handling. That meaning that variables, references and values is scope layer dependent. The generel formulat

## Overall course goals and learning outcomes.

Comment on the alignment of the concrete course goals (taken from the course description) to the theory presented, work You have done and knowledge You have gained. (I have put some comments in [...]).

- Lexical analysis, syntax analysis, and translation into abstract syntax.

- Regular expressions and grammars, context-free languages and grammars, lexer and parser generators. [lalr-pop is a classical parser generator, it auto generated the lexer for you based on regular expressions but allows for you to define the lexer yourself for more control]

- Identifier handling and symbol table organization. Type-checking, logical inference systems. [SOS is a logical inference system]

- Intermediate representations and transformations for different languages. [If you attended, Recall lectures relating LLVM/Crane-lift, discussions on SSA (Single Static Assignment) used in LLVM/Crane-lift, and discussions/examples on high [level optimization](https://gitlab.henriktjader.com/pln/d7050e_2020/-/tree/generics_and_traits/examples)]

- Code optimization and register allocation. Machine code generation for common architectures. [Both LLVM/Crane-Lift does the "dirty work" of backend optimization/register allocation leveraging the SSA form of the LLVM-IR]

Comment on additional things that you have experienced and learned throughout the course.
