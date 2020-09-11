# Repo for the D7050E course 2020

The repo will be updated througout the course and includes a draft outline of the course and hints towards reaching the learning goals.

# https://discord.gg/JtcjBfG

## Course Aim

Fundamental theories about computation and different models of computation. Construction of compilers. Lexical analysis, syntax analysis, and translation into abstract syntax.Regular expressions and grammars, context-free languages and grammars, lexer and parser generators. Identifier handling and symbol table organization. Type-checking, logical inference systems. Intermediate representations and transformations for different languages. Code optimization and register allocation. Machine code generation for common architectures.

In the course you will learn and develop your skills through hands on implementation work building your own complier from scratch. In this way theoretical aspects such as formal grammars, Structural Operational Semantics (SOS), and type rule formalizations becomes tangible. We will even touch upon memory safety and how guarantees can be achieved through static (compile time) borrow checking. Compiler backend (code optimization etc.) will be discussed in context of LLVM, which you will optionally interface as a library for code generation.

## Draft outline

### W1 The big picture, parsing, semantic analysis, code generation.

Theory
- From input language to executable
  - Lexing/Parsing to Abstract Syntax Tree (AST)
  - Semantic Analysis (type checking/well formedness)
  - Interpretation and high level optimizations
  - Code generation
  - Linking and run-time system

Practical assignment:

- We start with a minimal subset of Rust, comprising only 
  - Function definitions
  - Commands/statements (let, assignment, if then (else), while)
  - Expressions (including function calls)
  - Primitive types (boolean, i32) and their literals
  - Explicit types everywhere
  - Explicit return(s)

- Write a parser for expressions in Rust using [lalrpop](https://github.com/lalrpop/lalrpop) (which generates a parser implementation from a grammar specification) .

### W2 Regular expressions, automata and grammars 

Theory:

- Regular expressions and automata
- EBNF grammars
  
Practical assignment:

- Formulate an EBNF for your language (optional)
- Continue on the parser implementation, statements, functions, whole programs 

### W3 Context Free Grammars, Push Down Automata and Type Checking

Theory:

- DFA/NFA (regular expressions)
- Push Down Automata (PDA) for Context Free Grammars (CFG)
- Parsing strategies, pros and cons. L(1), LALR, parsing combinators, Parsing Expression Grammars (PEG), etc.

- Typing Rules and their Derivations

Practical assignment:
- Formulate typing rules for your language (optional)
- Finish parser

### W4 Structural Operational Semantics

Theory:

- Structural Operational Semantics (SOS)

Practical assignment:

- Formulate SOS rules for your language (optional)
- Implement interpreter 

### W6 Mutability and Memory References

Theory:

- Mutability and memory references
- Linear types and memory safety
- The Rust borrow model

Practical assignment

- Formalize type rules for your language (optional)
- Implement simple borrow checker
- Extend parser/AST/interpreter to support `&` and `&mut`. 

### W7 Type system extensions

Theory:

- Structured data (structs, enums, algebraic data types, arrays, slices, generics)
- Access methods
- Traits (type classes)

Practical assignment

- Extend parser/AST/type rules accordingly (optional)
- Extend formalizations, EBNF, typing rules and SOS (optional)

### W8 LLVM Backend, linking and run-time system

Theory:

- SSA form
- Concept of `unique`
- Code optimization techniques (performed by LLVM)
- LLVM API (a minimal subset)
- Linking and run-time system support

Practical assignment

- Use LLVM as library for code generation (optional)

---

### Home Exam

You will get the home exam to work on during the exam week. This may imply further theoretical exercises and experiments on your compiler.

### Examination

You will each be scheduled 30 minutes to present Your home exam to us, based on which Your final grade will be determined. Schedule will be agreed on later using Doodle.

---

## Your parser

- You are NOT required to account for operator precedence in expressions, however you MUST support parenthesized sub expressions. (+ for precedence towards higher grades)
- You are NOT required to account for location information (spans), but your error messages will be better if you do. (+ for spans, towards higher grades)
- Error recovery is NOT required (+ for recovery towards higher grades)
- You or NOT required to account for comments (neither single nor multiline). Doing

## Your type checker

- Your type checker should reject ill-typed programs according to your typing rules.
- (+ for higher grades)
  - span information in type errors
  - multiple error reporting
  - type inference (relaxing explicit typing where possible)

## Your interpreter

- Your interpreter should be able to correctly execute programs according to your SOS.
- Your interpreter should panic (with an appropriate error message) when encountering an evaluation error (e.g., 1 + false). Notice, such panics should never occur given that your type checker is correctly implemented.


## Your borrow checker

- Your borrow checker should reject borrow errors according to lexical scoping
- (+ for higher grades)
  - Introduce life time annotations and extend the borrow checker accordingly
  - Non Lexical Lifetimes (likely hard)

## Your type system

- Should support i32, bool, and their references at a minimum
- (+ for higher grades)
  - structs, enums, algebraic data types
- (++ for even higher grades)
  - generics
  - traits/subclasses

## Your LLVM bindings (Optional)

Implement for higher grades
- Basic code generation.
- Pass `noalias` where possible allowing for better optimization (assuming your borrow checker prevents aliasing).
- Other attributes, intrinsics, etc. that enables further LLVM optimizations.
- Bindings to external code, (this could allow for )

---

## Your Ambition, Your Work

Building a compiler from scratch is challenging, expect some trial and error until you end upp with nice/suitable abstractions and implementations. Also Rust as a language per se maybe new to you, and implies some learning in its own right. To that end the [Rust book](https://doc.rust-lang.org/book/) is a great asset. 

The standard library and `crates` that you will use, typically comes with great documentation (executable examples etc.) 

Rust also comes with a built in test framework, which allows to make unit tests directly in code and make integration tests in separate files/folders. The use of tests allows you to effectively partition large problems into smaller ones and validate the functionality along the way. Later those tests helps you keep your code base free of regressions (when you revise your codebase.) 

The `vscode` integration is excellent. Using the [rust analyser](https://github.com/rust-analyzer/rust-analyzer) you get type inference, code completion, cross-referencing, and access to documentation for free. You can even run tests directly in the editor, which is great during development.

Using git to manage your code base brings additional benefits, to share code, give feedback and collaborate. We encourage collaboration and sharing code is fine. However, in order for You to be awarded for your contributions towards the final grade You need to show us what parts you have originated/contributed to. 

For higher grades, there are optional assignments/extensions. Some of those are introduced later on in the course, making it hard to implement those in time for the examination. E.g., type system extensions and LLVM based code generation. If you feel that you want to dig deeper into those, we will help You to start earlier (already from day one.) 

You can choose to share a repo with your fellow students, keep your work in separate branches and merge into a common master. Git will track your contributions. In this way, your final compiler will benefits from contributions made by your fellow students. If some of you focus on e.g. code generation, and some of you on type system extensions, you will need to collaborate and decide on common data structures. Like wise, if some of you are interested in introducing spans for the tokens (location information), this will affect the common data structures. So there is a trade off, doing everything yourself may impose a higher workload but less of "synchronization" efforts, and the other way around. A sensible trade-off may be to share a repo between 2-4 students. Keeping the number of contributors down, makes it easier for You to pinpoint your individual contributions, but even for a group of 15 students there will be plenty of opportunities for each of you to shine.


# rust_lalrpop_compiler
