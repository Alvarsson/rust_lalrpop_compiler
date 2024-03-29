# Home Exam D7050E

- Fork this repo and put your answers (or links to answers) in THIS file.

## Your repo

Branch: master
https://github.com/Alvarsson/rust_lalrpop_compiler

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
The granularity of the SOS is generally in big-step. 
A transition rule is written as:

![](../master/images/transRule.png)

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

### Let command
![](../master/images/letimg.png)

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

![](../master/images/ariOp.png)

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

![](../master/images/boolOp.png)

examples:
```rust
 5 < 7;
 2 == 2;
 b != 9;
```
### General command
![](../master/images/cmdSeq.png)

Close to the let statement semantic, executing the first, then the second command as to not loose the intermediate derived state.

examples:
```rust
let x = 9;
func();
```
### Function command

This explains the function parameters semantics.

![](../master/images/func1.png)

With the function paramater semantic we can further explain the full function command as,

![](../master/images/func2.png)

examples:
```rust
fn main() {
    func();
}
```
### Conditionals true and false
![](../master/images/condT.png);

![](../master/images/condF.png)

example:
```rust
if a > 6 {
    c = c + 5;
}
```
### While-loop
For while-false exiting with same state.

![](../master/images/whileF.png);

And for while-true, concluding in derived state since block code executes.

![](../master/images/whileT.png);

example:
```rust
while a > 6 {
    a -= 1;
}
```
### Return statement
Explicit return has the semantic,

![](../master/images/retE.png),

which brings the return expression with the state.

### Assign statement
Since the variable has no state change and only moves the expression with the state we get conclusion,

![](../master/images/ass.png)

example:
```rust
x = 5; 
x = b;
```

### Command usage example
```rust
fn main() {
    fn test(a: i32) {
        let mut b = 6;
        b = 2+a; // Using the assign command here.
    }
    test(5);
    fn test2(per: &i32) -> i32 {
        let axel: i32 = 10 + 2 * 3; // Using the let command here where the expression evaluates to the value. This also show an arithmetic expression operation.
        axel = -1 - (1 - 1);
        return axel + per; // The return command is shown in use at this line. The expression evaluates and is returned in a derived state.
    }
    fn test3(foo: bool) -> bool { // This shows the function command with an example argument as parameter.
        if foo && true || false { // This showcases the conditional command evaluating a boolean. Also shows boolean operation.
            let a = test2(5);
            while ( a < 10) { // The while command is used here, evaluating to true leads to the assign command in the block.
                a = a + 1;
            }
            return true;
        }
        return 5 > 7;
    }
}
main();
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

### Let command 

Let statements in the type checker is divided into type i32 and type bool.

We generalize the result to type, t. This includes borth i32 and bool.

![](../master/images/typeLeti.png).

example:
```rust
let x : i32 = 5; 
let x = b; // given that b must be of type i32.
```

example:
```rust
let x : bool = false; 
let x = b; // given that b must be of type bool.
```

### Type i32 operands

Obviously these must evaluate to type i32 and the same operands is in play as the arithmetic operands.

![](../master/images/typeIop.png).

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

### Conditionals

The conditional statements are the same for both true and false regarding the semantics.

![](../master/images/typeIf.png).

example:
```rust
if true {
    ...
}
```

### While statements

Just like the conditionals, while statement semtantics are the same for both true and false.

![](../master/images/typeWhile.png).

The derived state will be evaluated as long as condition is of type boolean.

example:
```rust
while 5 > i {
    ...
}
```

### Return statements

The return statements for either type i32 or bool has the same semantics but obviously resulting in different types.

Their semantics are the following.

![](../master/images/typeRet.png).

examples:
```rust
return 5;
return false;
```

### Assign statement

As the previous statement semantics the assign types have the same semantic form.

![](../master/images/typeAss.png).

examples:
```rust
x = true;
x = 5 + 5;
```

### Function command

the a1:i32 in the example below is refered to the parameter 1 type or p1t. And so on for the function parameters.

example:
```rust
fn (a1:i32, b:&mut i32){
    ...
}
```

![](../master/images/typeFunc.png).

The function return type is held in the scope(environment) as to check that the correct block/layer returns the asked for type.

The type checker evaluates for each practical requirement in the course, as well as with borrowing references.

### Command usage example
```rust
fn main() {
    fn test(a: i32) {
        let mut b = 6;
        b = 2+a; // Using the assign command here where the variable needs to be of correct type and will evaluate as unit.
    }
    test(5);
    fn test2(per: &i32) -> i32 {
        let axel: i32 = 10 + 2 * 3; // Using the let command here where the expression evaluates to the type. This also show an arithmetic expression operation for type.
        axel = -1 - (1 - 1);
        return axel + per; // The return command is shown in use at this line. The expression evaluates to a type in the derived state.
    }
    fn test3(foo: bool) -> bool { // This shows the function command with an example argument as parameter which block evaluate to function return type.
        if foo && true || false { // This showcases the conditional command with the commands for the evaluated boolean types.
            let a = test2(5);
            while ( a < 10) { // The while command is used here, evaluating to true leads with the command as unit.
                a = a + 1;
            } else {
            return true;
            }
        }
        return 5 > 7;
    }
}
main();
```


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

## Your borrow checker

- Give a specification for well versus ill formed borrows. (What are the rules the borrow checker should check).

- Demonstrate the cases of ill formed borrows that your borrow checker is able to detect and reject.

- Compare your solution to the requirements (as stated in the README.md). What are your contributions to the implementation.

### Well formed borrows

All the examples below are testable in the the main.rs file in the ast folder and passes the borrow check built into the type_checker.

Larger example which covers most parts, this is testable by running the function "test_borrow_check" in the main.rs file.
```rust
fn test() {
        fn test1(p: &String)  {
            let b = *p;
        }
        fn test2(a: &mut String) {
            let mut b = 'oj';
            let mut e = &mut b;
            test1(&b)  
        }
        fn test3(b: &i32) -> i32 {
            return *b + 4;
        }
    }
```

The example below show that we can have any number of references to a variable, as long as the variable is immutable.
```rust
fn test() {
        fn test1()  {
            let b = 10;
            let c = &b;
            let d = &b;
        }
    }
```

The example below shows that variable scope handling is correct, you can borrow from the scope layer below. 
In the "Ill formed borrows" section below, an example shows that values are correctly dropped when leaving scope.
```rust
fn test() {
        fn test1()  {
            let a = 5;
            if true {
                let b = &a;
                let c = 89;
            }
        }
    }
```

### Ill formed borrows

All of these examples are testable in the main.rs file in the ast folder. The borrow checking will detect and reject the examples explained below.

In accordance to Rust rules of reference the borrow checker catches a case where one tries to have more than one mutable reference.
This example below returns the following: "Error: b, is already borrowed as mutable".
example: More than one mutable reference.
```rust
fn test() {
        fn test1()  {
            let mut b = 10;
            let c = &mut b;
            let d = &mut b;
        }   
    }
```

This example simply shows that the borrow checker detects that a value has been dropped out of the scope and thus "c" can'b be borrowed.
```rust
fn test() {
        fn test1()  {
            let a = 5;
            if true {
                let b = &a;
                let c = 89;
            }
            let d = &c;
        }
    }
```

The example below shows that a variable already borrowed as immutable can't later be borrowed as mutable. 
```rust
fn test() {
        let mut s = 50;
        let r1 = &s;
        let r2 = &s;
        let r3 = &mut s;
    }
```


The borrow checker, just as with the type checker and interpreter, is included in scope layer handling and the code. The examples above are testable via the main.rs file .
With referencing variables we can borrow values in different scopes without affecting the original variable value.

## Contribution summary

This code has gone through sevaral iterations with influence from other developers during the whole project. The influence regards types of implementation options but everything is built on the base of the parser and AST. The parser structure originates from the calculator example in the lalrpop git page.

## Overall course goals and learning outcomes.

- Lexical analysis, syntax analysis, and translation into abstract syntax.
    - LALRPOP is a Rust parser generator framework which greatly simplified the lexican and syntax analysis. With it, writing DRY grammar was much easier to understand, especially with the abstract syntax tree implementation. When starting there immediately was a speed bump in understanding how tokenizing worked and how it handled styntax precedence. The AST seemed much simpler to grasp than the impementation itself since all of my LALRPOP lexical versions was re-done because of ambiguity errors which only was solvable by re-doing the entire structure.
    
- Regular expressions and grammars, context-free languages and grammars, lexer and parser generators. [lalr-pop is a classical parser generator, it auto generated the lexer for you based on regular expressions but allows for you to define the lexer yourself for more control]
    - Besides what is mentioned above, writing the EBNF of this report was very simple thanks to LALRPOP. And with the way LALRPOP tokenize input program, I could understand the AST connection better and in the end better see how the terminals are reached in a clear way. 

- Identifier handling and symbol table organization. Type-checking, logical inference systems. [SOS is a logical inference system]
    - For the type checker the major problem was figuring out how to start. That's not to say that it was easy after I completed the terminal type checking. I was constantly stopping and rethinking the checking method which, fortunately, led me to a better understanding. The more difficult part was of how to to implement the scope(environment) handler as to get borrowing correct.
    - As for the SOS it was a very difficult area for me which in all honesty im not sure that I understand to its fullest even now. I tried to implement the SOS as correctly as I could in this report and I see how it is a good way to represent the language in a more formal and generic way.
    - The interpreter was much easier to implement since it is grounded in the way that the type checker works with a few tweaks. First there was some problems in figuring out in what way I was to get the actual values evaluated, but I realized that expressions and its terminals could be utilized.

Comment on additional things that you have experienced and learned throughout the course.

This course may have overshot its actuall deadline but this doesn't bother me to much at this point. Since thinking back i've learned at a higher rate and much more than in previous courses and this time regarding a subject that I really feel is interesting.
This has given an insight into the ins and outs of a real compiler and im not a stranger to working on this in the future.
The amount of paper i spent on drawing out a solid plan for the scope handler is amazing. The end product and how it works with borrowing is really worth it.
Last but not least, RUST. At first glance it seemed like nothing worth noticing, but the more I learned about how much though has been put into this language, the more i liked it. Im really excited to work with RUST in the future and will more than likely use it for my own projects.

