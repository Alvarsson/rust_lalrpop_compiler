use std::io;
use std::io::*;
use crate::parser::*;
use crate::type_check;


// The interpreter should send a user input
// into the type_checker for evaluation


// 1- take an input program from the user

// 2- Run it through the typechecker to see if valid

// 3- Code an evaluator to give correct values and returns


pub fn interpreter() {
    let mut s = String::new(); // input
    let stdin = io::stdin();
    let mut run = true;
    let mut stdout = io::stdout();
    let mut tabs = 0;
    let mut user_prgm = vec![];
    while (run) {
        if tabs == 0 {
            print!(">>>"); // bottom tab depth
        } else {
            print!("..."); 
        }
        stdout.flush(); //empty the sink
        stdin.read_line(&mut s).expect("Can't read the user input");
        //need to count { and } to keep track of tab depth
        tabs += s.matches("{").count() as i32 - s.matches("}").count() as i32; //count is usize, set as i32
        if tabs < 0 {
            panic!("Unexpected closing delimiter");
        }
        if tabs == 0 {
            let input = s.clone();
            user_prgm.push(format!("{}", input)); // push in the code
            let main = format!("{}", user_prgm.join("\n"));
            //Test in type checker
            println!("{}", main);
            let test = StmtsParser::new().parse(&main).unwrap();
            let eval = test.clone();
            let mut scope = type_check::Scope::newScope(main.to_string());
            let r = type_check::statement_check(vec![test], &mut scope);
            println!("{:?}", r);
            if r.is_err() {
                println!("{}", r.unwrap_err());
                s.clear();
                run = false;
            } else {
                println!("Type checking OK!");
                println!("{}", eval);
                run = false;
            }
        } else {
            user_prgm.push(s.clone());
        }
        s.clear(); // empty input string.
    }
}

