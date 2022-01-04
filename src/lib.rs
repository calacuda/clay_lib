// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }

use libloading::{Library, Symbol};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum Token {
    //pub enum Token<'input> {
    LParen,
    RParen,
    Symbol(String),
    Str(String),
    Number(String),
    Bool(bool),
    // Tick,
    Form(Vec<Token>),
    EOF,
}

#[derive(Debug, Clone)]
pub enum Nargs {
    INF,
    Num(usize),
}

/*
pub enum LangLibrary {
    Compiled(
        HashMap<
            String,
            (
                Nargs,
                String,
                // &'a (dyn Fn(&Vec<Token>) -> Result<Option<Token>, &'a str>),
            ),
        >,
    ),
    Lisp(String),
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum KnownThing {
    // LispFunc(parser::Node),
    LispFunc(Token),
    CompiledFunc((PathBuf, String)), // (library, function name)
    Var(Token),
}

pub fn def_let(
    known_things: &mut HashMap<String, KnownThing>,
    tmp_known_things: &mut HashMap<String, KnownThing>,
    libs: &mut HashMap<String, LangLibrary>,
    s_exp: &Token,
) -> Token {
    let (_, mut vars, code) = match s_exp {
        Token::Form(list) => (
            list[0].clone(),
            match &list[1] {
                Token::Form(l) => l,
                _ => panic!("let takes a list of variables to assign."),
            },
            list[2..].to_vec(),
        ),
        _ => panic!("thats not a let statement!"),
    };

    for var in vars {
        let (var_name, val) = match var {
            Token::Form(list) => {
                let evaled = match list[1] {
                    Token::Form(_) => evaluate(known_things, tmp_known_things, libs, &list[1]),
                    _ => list[1].clone(),
                };

                let name = match &list[0] {
                    Token::Symbol(sym) => sym.clone(),
                    _ => panic!("var name cant be a datatype."),
                };

                (name, evaled)
            }
            _ => panic!("let claus needs to be a form of forms."),
        };

        // println!("name: {:?} --- val: {:?}", var_name, val);
        tmp_known_things.insert(var_name, KnownThing::Var(val));
    }

    // for statement in code {
    //     evaluate(known_things, tmp_known_things, libs, &statement);
    // }
    return call_code(known_things, tmp_known_things, libs, code);
}

pub fn def(
    known_things: &mut HashMap<String, KnownThing>,
    tmp_known_things: &mut HashMap<String, KnownThing>,
    libs: &mut HashMap<String, LangLibrary>,
    s_exp: &Token,
) {
    /*
     * used to define functons or variables
     */
    // todo: write it!
    match s_exp {
        Token::Form(list) if list[0] == Token::Symbol("let".to_string()) => {
            def_let(known_things, tmp_known_things, libs, s_exp);
            return;
        }
        Token::Form(list) if list[0] == Token::Symbol("defun".to_string()) => {}
        _ => panic!("non form recieved by interpreter function def as s_exp"),
    }

    let thing = KnownThing::LispFunc(Token::Form(match s_exp {
        Token::Form(list) => list[2..].to_vec(),
        _ => panic!("ERROR Will Robinson!"),
    }));
    // println!("{:?}", s_exp);
    let name = match s_exp {
        Token::Form(l) => match &l[1] {
            Token::Symbol(name) => name.clone(),
            _ => panic!("function name can not be a datatype."),
        },
        _ => panic!("ERROR Will Robinson!"),
    };
    // println!("{:?}", thing);
    known_things.insert(name, thing);
}

pub fn call_code(
    known_things: &mut HashMap<String, KnownThing>,
    tmp_known_things: &mut HashMap<String, KnownThing>,
    libs: &mut HashMap<String, LangLibrary>,
    lisp_code: Vec<Token>,
) -> Token {
    let mut res = Token::Bool(false);

    for statement in lisp_code {
        res = evaluate(known_things, tmp_known_things, libs, &statement);
    }

    return res;
}

pub fn call_lisp(
    known_things: &mut HashMap<String, KnownThing>,
    tmp_known_things: &mut HashMap<String, KnownThing>,
    libs: &mut HashMap<String, LangLibrary>,
    lisp_code: &Token,
    args: Vec<Token>,
) -> Token {
    /*
     * calls a lisp function.
     */
    // println!("{:?}", lisp_code);
    let (params, algorithm) = match lisp_code {
        Token::Form(list) => {
            let p = match &list[0] {
                Token::Form(l) => l,
                _ => panic!("param list must be a form"),
            };
            let algo = list[1..].to_vec();
            (p, algo)
        }
        _ => panic!("not a lisp function"),
    };

    if args.len() != params.len() {
        panic!("you must have the same number of arguyments as you do parameters");
    }

    for i in 0..args.len() {
        tmp_known_things.insert(
            match &params[i] {
                Token::Symbol(sym) => sym.clone(),
                _ => panic!("parameter must not be a datatype"),
            },
            KnownThing::Var(args[i].clone()),
        );
    }

    call_code(known_things, tmp_known_things, libs, algorithm)
}

pub fn call_comp<'a>(lib_name: &PathBuf, func_name: &String, args: Vec<Token>) -> Token {
    /*
     * calls a compiled rust/c/golang/whatever function from the
     * .so file stored in lib_name.
     */
    use std::path::{Path, PathBuf};

    let result = unsafe {
        let lib = Library::new(lib_name).unwrap();
        let func: Symbol<fn(&Vec<Token>) -> Result<Option<Token>, &'a str>> =
            lib.get(func_name.as_bytes()).unwrap();
        func(&args)
    };

    return match result {
        Ok(Some(data)) => data,
        Ok(None) => Token::Bool(false),
        Err(err) => panic!(
            "function: {} form library: {:?} return with error <{}>",
            func_name, lib_name, err
        ),
    };
    // return result;
}

pub fn get_dual_hashmap(
    known_things: &mut HashMap<String, KnownThing>,
    tmp_known_things: &mut HashMap<String, KnownThing>,
    sym: &String,
) -> KnownThing {
    return match (tmp_known_things.get(sym), known_things.get(sym)) {
        (Some(thing), None) => thing,
        (None, Some(thing)) => thing,
        (Some(thing1), Some(thing2)) => thing1,
        (None, None) => panic!(
            "that function or variable does not exist. function/variable name [{:?}]",
            sym
        ),
        // tmp_known_things.get(&sym)
    }
    .clone();
}

pub fn evaluate(
    known_things: &mut HashMap<String, KnownThing>,
    tmp_known_things: &mut HashMap<String, KnownThing>,
    libs: &mut HashMap<String, LangLibrary>,
    s_exp: &Token,
) -> Token {
    // println!("s_exp :  {:?}", s_exp);
    let (mut action, args) = match s_exp {
        Token::Form(list) => (&list[0], list[1..].to_vec()),
        _ => return s_exp.clone(), // panic!("trying to eval a non-form data type from the global scope as an action."),
    };

    match action {
        Token::Symbol(s) if s == "let" => {
            return def_let(known_things, tmp_known_things, libs, s_exp);
            // Token::Bool(false)
        }
        _ => {}
    }

    // let args = &s_exp.children;
    let mut evaled_args = Vec::new();

    for arg in args {
        match arg {
            Token::Form(_) => {
                evaled_args.push(evaluate(known_things, tmp_known_things, libs, &arg))
            }
            Token::Symbol(ref sym) => {
                let var_val = get_dual_hashmap(tmp_known_things, known_things, sym);

                match var_val {
                    KnownThing::Var(val) => evaled_args.push(val.clone()),
                    KnownThing::LispFunc(_) => evaled_args.push(arg),
                    KnownThing::CompiledFunc(_) => evaled_args.push(arg),
                };
            }
            _ => evaled_args.push(arg),
        }
    }

    // println!("evaluate :  {:?}", action);
    // println!("known_things :  {:?}", known_things.keys());

    // println!("action :  {:?}", action);

    return match action {
        Token::Symbol(sym) => {
            let thing: KnownThing = get_dual_hashmap(known_things, tmp_known_things, sym);

            // match known_things.get(sym) {
            //     Some(thing) => thing.clone(),
            //     None => panic!("that function or variable does not exist."),
            // };

            match thing {
                KnownThing::LispFunc(s_exp) => {
                    // println!("calling a lisp function");
                    call_lisp(known_things, tmp_known_things, libs, &s_exp, evaled_args)
                } // evaluate(known_things, libs, &s_exp),
                KnownThing::CompiledFunc(f) => call_comp(&f.0, &f.1, evaled_args),
                KnownThing::Var(name) => panic!("you cant call a variable!"),
            }
        }
        Token::Str(_) | Token::Bool(_) | Token::Number(_) => action.clone(),
        Token::LParen | Token::RParen | Token::EOF => {
            panic!("there should be no parens or EOF tokens here.")
        }
        Token::Form(list) => {
            panic!("trying to eval a form as an action.");
        }
    };

    // return call_func(known_things, libs, &action, evaled_args);
}
*/
