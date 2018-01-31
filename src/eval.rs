use std::fs;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use ::Datum;
use ::LispFn;
use ::LispResult;
use ::LispErr;
use ::Lambda;
use ::Meaning;
use ::Symbol;
use ::BindingRef;
use ::LispErr::*;

use symbol_table::SymbolTable;
use syntax_rule::SyntaxRule;
use env::{Env, EnvRef, AEnv, AEnvRef};
use parser::Parser;
use builtin;
use preprocess;

pub struct Evaluator {
    level: isize,
    symbol_table: SymbolTable,
    pub syntax_rules: HashMap<Symbol, SyntaxRule>,
    pub root_env: EnvRef,
    root_aenv: AEnvRef,
    builtins: HashMap<String, LispFn>,
}

pub type TCOResult = Result<TCOWrapper, LispErr>;
pub enum TCOWrapper {
    Return(Datum),
    TailCall(Meaning, EnvRef),
}

// pub struct ContWrapper {
//     expression: Meaning,
//     environment: Env,
//     continuation: Continuation
// }

// enum Continuation {
//     If(Meaning, Meaning, Env, Box<Continuation>),
//     Print,
//     Do(Vec<Meaning>, Env),
//     Set(Symbol, Env),
//     Define(Symbol, Env),
// }

// trait Continuation {
//     fn resume(&self, value: Datum) -> LispResult;
// }

impl Evaluator {
    pub fn new(stdlib: bool) -> Self {
        let symbol_table = SymbolTable::new();
        let mut builtins: HashMap<String, LispFn> = HashMap::new(); 
        builtin::load(&mut builtins);

        let root_env = Env::new(None);
        let env_ref = Rc::new(RefCell::new(root_env));

        let root_aenv = AEnv::new(None);
        let aenv_ref = Rc::new(RefCell::new(root_aenv));

        let mut ev = Evaluator {
            symbol_table: symbol_table,
            syntax_rules: HashMap::new(),
            builtins: builtins,
            level: 0,
            root_env: env_ref,
            root_aenv: aenv_ref
        };

        if stdlib {
            let paths = fs::read_dir("./stdlib").unwrap();
            // TODO: is there a better way to get a sorted list
            // of strings with the paths of files in a dir?
            let mut string_paths: Vec<String> = paths.map(
                |p| p.unwrap().path().display().to_string()
            ).collect();
            string_paths.sort();
            for path in string_paths {
                // println!("Loading {}", path);
                ev.eval_file(&path[..]).expect("Failed to load lib");
            }
        }

        ev
    }

    pub fn apply(&mut self, f: Datum, mut evaled_args: Vec<Datum>, env_ref: EnvRef) -> TCOResult {
        match f {
            Datum::Lambda(mut lambda) => {
                let mut child_env = Env::new(Some(lambda.env.clone()));

                if lambda.dotted {
                    let given = evaled_args.len();
                    let takes = lambda.arity - 1;

                    if given > takes {
                        let rest = evaled_args.split_off(takes);
                        evaled_args.push(Datum::List(rest));
                        child_env.extend(evaled_args);
                    } else {
                        let defaults = lambda.defaults.len();
                        let missing = takes - given;

                        if missing > defaults {
                            return Err(InvalidNumberOfArguments);
                        } else {
                            let mut defs = lambda.defaults.split_off(defaults - missing);
                            evaled_args.append(&mut defs);
                            evaled_args.push(Datum::List(vec!()));
                            child_env.extend(evaled_args);
                        }
                    }
                } else {
                    let given = evaled_args.len();
                    let takes = lambda.arity;

                    if given == takes {
                        child_env.extend(evaled_args);
                    } else {
                        let defaults = lambda.defaults.len();
                        let missing = takes - given;

                        if missing > defaults {
                            return Err(InvalidNumberOfArguments);
                        } else {
                            let mut defs = lambda.defaults.split_off(defaults - missing);
                            evaled_args.append(&mut defs);
                            child_env.extend(evaled_args);
                        }
                    }
                }

                Ok(TCOWrapper::TailCall((*lambda.body), Rc::new(RefCell::new(child_env))))
            },
            Datum::Builtin(LispFn(fun, arity, name)) => {
                arity.check(evaled_args.len(), &name);
                Ok(TCOWrapper::Return(fun(evaled_args.as_mut_slice(), self, env_ref)?))
            },
            a => panic!("Tried to apply {}", a)
        } 
    }

    pub fn eval_file(&mut self, path: &str) -> LispResult {
        // TODO: Add IOError type
        let mut f = File::open(path).expect("Could not open file");
        let mut input = String::new();
        f.read_to_string(&mut input).expect("Could not read file");
        self.eval_str(&input[..])
    }

    pub fn eval_str(&mut self, input: &str) -> LispResult {
        let string = String::from(input);
        let mut parser = Parser::from_string(&string);
        let mut ret = Datum::Nil;

        // TODO: convert parser errors to lisp errors
        while let Some(next) = parser.next_datum(&mut self.symbol_table).expect("Failed to parse") {
            let env_ref = self.root_env.clone();
            let preprocessed = preprocess::preprocess(next, &mut self.symbol_table, &self.builtins, &self.syntax_rules, self.root_aenv.clone())?;
            match self.eval(preprocessed, env_ref) {
                Ok(res) => ret = res,
                Err(msg) => println!("!! {}", msg)
            }
        }

        Ok(ret)
    }

    fn eval_list(&mut self, exprs: Vec<Meaning>, env_ref: EnvRef) -> Vec<Datum> {
        exprs.into_iter().map(|a| self.eval(a, env_ref.clone()).unwrap()).collect()
    }

    fn eval_sf_if(&mut self, cond: Meaning, cons: Meaning, alt: Meaning, env_ref: EnvRef) -> TCOResult {
        let result = self.eval(cond, env_ref.clone())?;
        if result.is_false() {
            Ok(TCOWrapper::TailCall(alt, env_ref))
        } else {
            Ok(TCOWrapper::TailCall(cons, env_ref))
        }
    }

    fn eval_sf_do(&mut self, expressions: Vec<Meaning>, last: Meaning, env_ref: EnvRef) -> TCOResult {
        for e in expressions.into_iter() {
            self.eval(e, env_ref.clone())?;
        }
        Ok(TCOWrapper::TailCall(last, env_ref))
    }

    fn eval_sf_definition(&mut self, value: Meaning, env_ref: EnvRef) -> TCOResult {
        // TODO: Check if this would use the correct binding?
        // (env.counter + 1 == key.index)
        let value = self.eval(value, env_ref.clone())?;
        let mut env_ = env_ref.borrow_mut();

        env_.extend(vec![value]);
        Ok(TCOWrapper::Return(Datum::Undefined))
    }

    fn eval_sf_assignment(&mut self, key: BindingRef, value: Meaning, env_ref: EnvRef) -> TCOResult {
        let value = self.eval(value, env_ref.clone())?;
        let env = env_ref.borrow();

        let binding = env.get_binding(key);
        (*binding.borrow_mut()) = value;
        Ok(TCOWrapper::Return(Datum::Undefined))
    }

    fn eval_sf_list_push(&mut self, key: BindingRef, value: Meaning, env_ref: EnvRef) -> TCOResult {
        let value = self.eval(value, env_ref.clone())?;
        let env = env_ref.borrow();

        let binding = env.get_binding(key);
        (*binding.borrow_mut()).push(value);
        Ok(TCOWrapper::Return(Datum::Undefined))
    }

    fn eval_sf_list_ref(&mut self, key: BindingRef, index: Meaning, env_ref: EnvRef) -> TCOResult {
        let vindex = self.eval(index, env_ref.clone())?;
        if let Datum::Integer(index) = vindex {
            let env = env_ref.borrow();

            let binding = env.get_binding(key);
            let mut bm = binding.borrow_mut();
            match *bm {
                Datum::List(ref mut elements) => {
                    if let Some(elem) = elements.get(index as usize) {
                        Ok(TCOWrapper::Return(elem.clone()))
                    } else {
                        panic!("Index out of bounds")
                    }
                },
                _ => {
                    panic!("Usage: (list-ref <id> <index>)")
                }
            }
        } else {
            panic!("Usage: (list-ref <id> <index>)")
        }
    }

    fn eval_sf_list_set(&mut self, key: BindingRef, index: Meaning, value: Meaning, env_ref: EnvRef) -> TCOResult {
        let vindex = self.eval(index, env_ref.clone())?;
        let value = self.eval(value, env_ref.clone())?;

        if let Datum::Integer(index) = vindex {
            let env = env_ref.borrow();

            let binding = env.get_binding(key);
            let mut bm = binding.borrow_mut();
            match *bm {
                Datum::List(ref mut elements) => {
                    if let Some(elem) = elements.get_mut(index as usize) {
                        *elem = value;
                        Ok(TCOWrapper::Return(Datum::Undefined))
                    } else {
                        // TODO: Index out of bounds
                        Err(InvalidTypeOfArguments)
                    }
                },
                _ => {
                    Err(InvalidTypeOfArguments)
                }
            }
        } else {
            Err(InvalidTypeOfArguments)
        }
    }

    pub fn eval_datum(&mut self, datum: Datum, env_ref: EnvRef) -> LispResult {
        // let preprocessed = preprocess::preprocess(datum, &mut self.symbol_table, &self.builtins, &self.syntax_rules, self.root_aenv.clone())?;
        // self.eval(preprocessed, env_ref)

        // TODO: because the preprocessor works with Expressions,
        // but eval is called with Datums, this can't work right now
        panic!("This doesn't work right now")
    }

    pub fn full_apply(&mut self, fun: Datum, evaled_args: Vec<Datum>, env_ref: EnvRef) -> Datum {
        match self.apply(fun, evaled_args, env_ref).unwrap() {
            TCOWrapper::Return(result) => result,
            TCOWrapper::TailCall(expr, env) => {
                self.eval(expr, env).unwrap()
            }
        }
    }

    pub fn eval(&mut self, expr: Meaning, mut env_ref: EnvRef) -> LispResult {
        self.level += 1;
        let mut maybe_expr = Some(expr);

        while let Some(e) = maybe_expr {
            let res = match e {
                Meaning::If(cond, cons, alt) => self.eval_sf_if(*cond, *cons, *alt, env_ref),
                Meaning::Do(es, last) => self.eval_sf_do(es, *last, env_ref),
                Meaning::Quote(datum) => Ok(TCOWrapper::Return(*datum)),
                Meaning::Definition(value) => self.eval_sf_definition(*value, env_ref),
                Meaning::SyntaxRuleDefinition(name, rule) => {
                    self.syntax_rules.insert(name, *rule);
                    Ok(TCOWrapper::Return(Datum::Nil))
                },
                Meaning::Assignment(name, value) => self.eval_sf_assignment(name, *value, env_ref),
                Meaning::ListPush(name, value) => self.eval_sf_list_push(name, *value, env_ref),
                Meaning::ListRef(name, value) => self.eval_sf_list_ref(name, *value, env_ref),
                Meaning::ListSet(name, index, value) => self.eval_sf_list_set(name, *index, *value, env_ref),
                Meaning::BindingRef(key) => {
                    let env = env_ref.borrow();
                    let binding = env.get_binding(key);
                    let value = binding.borrow().clone();

                    Ok(TCOWrapper::Return(value))
                },
                Meaning::FunctionCall(fun, args) => {
                    let f = self.eval(*fun, env_ref.clone())?;
                    let evaled_args = self.eval_list(args, env_ref.clone());
                    self.apply(f, evaled_args, env_ref)
                },
                Meaning::BuiltinFunctionCall(fun, args) => {
                    let mut evaled_args = self.eval_list(args, env_ref.clone());
                    Ok(TCOWrapper::Return(fun(evaled_args.as_mut_slice(), self, env_ref)?))
                },
                Meaning::LambdaDef(arity, defaults, body, dotted) => {
                    Ok(TCOWrapper::Return(Datum::Lambda(
                        Lambda{
                            env: env_ref,
                            arity: arity,
                            defaults: defaults,
                            body: body,
                            dotted: dotted
                        }
                    )))
                },
                Meaning::SelfEvaluating(v) => Ok(TCOWrapper::Return(*v)),
                _ => panic!("Meaning not valid in this context")
            };

            match res? {
                TCOWrapper::Return(v) => {
                    self.level -= 1;
                    return Ok(v)
                },
                TCOWrapper::TailCall(a, e) => {
                    maybe_expr = Some(a);
                    env_ref = e;
                    continue;
                }
            }
        }

        self.level -= 1;
        Ok(Datum::Undefined)
    }
}
