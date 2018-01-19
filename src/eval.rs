use std::fs;
use std::fs::File;
use std::io::Read;
use std::collections::{HashMap, BTreeMap};
use std::rc::Rc;
use std::cell::RefCell;

use ::Datum;
use ::LispFn;
use ::LispResult;
use ::LispErr;
use ::Promise;
use ::Lambda;
use ::LambdaType;
use ::Expression;
use ::Symbol;
use ::LispErr::*;

use symbol_table::SymbolTable;
use env::{Env, EnvRef};
use parser;
use builtin;
use preprocess;

pub struct Evaluator {
    level: isize,
    symbol_table: SymbolTable,
    macros: HashMap<Symbol, Expression>,
    root_env: EnvRef,
    builtins: HashMap<String, LispFn>,
}

pub type TCOResult = Result<TCOWrapper, LispErr>;
pub enum TCOWrapper {
    Return(Datum),
    TailCall(Expression, EnvRef),
}

// pub struct ContWrapper {
//     expression: Expression,
//     environment: Env,
//     continuation: Continuation
// }

// enum Continuation {
//     If(Expression, Expression, Env, Box<Continuation>),
//     Print,
//     Do(Vec<Expression>, Env),
//     Set(Symbol, Env),
//     Define(Symbol, Env),
// }

// trait Continuation {
//     fn resume(&self, value: Datum) -> LispResult;
// }

impl Evaluator {
    pub fn new() -> Self {
        let symbol_table = SymbolTable::new();
        let mut builtins: HashMap<String, LispFn> = HashMap::new(); 
        builtin::load(&mut builtins);

        let root_env = Env::new(None);
        let env_ref = Rc::new(RefCell::new(root_env));

        let mut ev = Evaluator {
            symbol_table: symbol_table,
            macros: HashMap::new(),
            builtins: builtins,
            level: 0,
            root_env: env_ref
        };

        let paths = fs::read_dir("./stdlib").unwrap();
        for path in paths {
            let path_str = path.unwrap().path().display().to_string();
            ev.eval_file(&path_str[..]).expect("Failed to load lib");
        }

        ev
    }

    pub fn apply(&mut self, f: Datum, mut evaled_args: Vec<Datum>, env_ref: EnvRef) -> TCOResult {
        match f {
            Datum::Lambda(lambda) => {
                let mut child_env = Env::new(Some(lambda.env));

                match lambda.kind {
                    LambdaType::List => {
                        let given = evaled_args.len();
                        let takes = lambda.params.len();
                        let defaults = lambda.defaults.len();
                        let missing = takes - given;

                        if missing > defaults {
                            return Err(InvalidNumberOfArguments);
                        } else {
                            evaled_args.extend(
                                lambda.defaults.iter().cloned().skip(defaults - missing)
                            );
                            child_env.extend(
                                lambda.params,
                                evaled_args
                            );
                        }
                    },
                    LambdaType::DottedList => {
                        let given = evaled_args.len();
                        let takes = lambda.params.len() - 1;

                        if given > takes {
                            let rest = evaled_args.split_off(takes);
                            evaled_args.push(Datum::List(rest));
                            child_env.extend(
                                lambda.params,
                                evaled_args
                            );
                        } else {
                            let defaults = lambda.defaults.len();
                            let missing = takes - given;

                            if missing > defaults {
                                return Err(InvalidNumberOfArguments);
                            } else {
                                evaled_args.extend(
                                    lambda.defaults.iter().cloned().skip(defaults - missing)
                                );
                                evaled_args.push(Datum::List(vec!()));
                                child_env.extend(
                                    lambda.params,
                                    evaled_args
                                );
                            }
                        }
                    }
                }

                Ok(TCOWrapper::TailCall(*lambda.body, Rc::new(RefCell::new(child_env))))
            },
            Datum::Builtin(LispFn(fun, arity)) => {
                arity.check(evaled_args.len());
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

    pub fn lookup_macro(&self, name: Symbol) -> Option<&Expression> {
        self.macros.get(&name)
    }

    pub fn eval_str(&mut self, input: &str) -> LispResult {
        let result = parser::parse_program(input);
        let mut ret = Datum::Nil;

        for v in result.into_iter() {
            let env_ref = self.root_env.clone();
            let preprocessed = preprocess::preprocess(v, &mut self.symbol_table, &self.builtins)?;
            match self.eval(preprocessed, env_ref) {
                Ok(res) => ret = res,
                Err(msg) => println!("!! {}", msg)
            }
        }

        Ok(ret)
    }

    fn eval_list(&mut self, exprs: Vec<Expression>, env_ref: EnvRef) -> Vec<Datum> {
        exprs.into_iter().map(|a| self.eval(a, env_ref.clone()).unwrap()).collect()
    }

    fn eval_sf_if(&mut self, cond: Expression, cons: Expression, alt: Expression, env_ref: EnvRef) -> TCOResult {
        match self.eval(cond, env_ref.clone())? {
            Datum::Bool(true) => Ok(TCOWrapper::TailCall(cons, env_ref)),
            Datum::Bool(false) => Ok(TCOWrapper::TailCall(alt, env_ref)),
            _ => panic!("Condition of if must return a boolean"),
        }
    }

    fn eval_sf_do(&mut self, expressions: Vec<Expression>, last: Expression, env_ref: EnvRef) -> TCOResult {
        for e in expressions.into_iter() {
            self.eval(e, env_ref.clone())?;
        }
        Ok(TCOWrapper::TailCall(last, env_ref))
    }

    fn eval_sf_definition(&mut self, key: Symbol, value: Expression, env_ref: EnvRef) -> TCOResult {
        let value = self.eval(value, env_ref.clone())?;
        let mut env_ = env_ref.borrow_mut();

        env_.define(key, value);
        Ok(TCOWrapper::Return(Datum::Undefined))
    }

    fn eval_sf_assignment(&mut self, key: Symbol, value: Expression, env_ref: EnvRef) -> TCOResult {
        let value = self.eval(value, env_ref.clone())?;
        let env = env_ref.borrow();

        if let Some(binding) = env.find_def(&key) {
            (*binding.borrow_mut()) = value;
            Ok(TCOWrapper::Return(Datum::Undefined))
        } else {
            panic!("Definition not found: {}", self.symbol_table.name(key));
        }
    }

    fn eval_sf_list_push(&mut self, key: Symbol, value: Expression, env_ref: EnvRef) -> TCOResult {
        let value = self.eval(value, env_ref.clone())?;
        let env = env_ref.borrow();

        if let Some(binding) = env.find_def(&key) {
            let mut b = binding.borrow_mut();
            b.push(value);
            Ok(TCOWrapper::Return(Datum::Undefined))
        } else {
            panic!("Definition not found: {}", self.symbol_table.name(key));
        }
    }

    fn eval_sf_list_ref(&mut self, key: Symbol, index: Expression, env_ref: EnvRef) -> TCOResult {
        let vindex = self.eval(index, env_ref.clone())?;
        if let Datum::Integer(index) = vindex {
            let env = env_ref.borrow();

            if let Some(binding) = env.find_def(&key) {
                match *binding.borrow_mut() {
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
                panic!("Definition not found: {}", self.symbol_table.name(key));
            }
        } else {
            panic!("Usage: (list-ref <id> <index>)")
        }
    }

    fn eval_sf_list_set(&mut self, key: Symbol, index: Expression, value: Expression, env_ref: EnvRef) -> TCOResult {
        let vindex = self.eval(index, env_ref.clone())?;
        let value = self.eval(value, env_ref.clone())?;

        if let Datum::Integer(index) = vindex {
            let env = env_ref.borrow();

            if let Some(binding) = env.find_def(&key) {
                match *binding.borrow_mut() {
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
                panic!("Definition not found: {}", self.symbol_table.name(key));
            }
        } else {
            Err(InvalidTypeOfArguments)
        }
    }

    pub fn eval_datum(&mut self, datum: Datum, env_ref: EnvRef) -> LispResult {
        let preprocessed = preprocess::preprocess(datum, &mut self.symbol_table, &self.builtins)?;
        self.eval(preprocessed, env_ref)
    }

    pub fn full_apply(&mut self, fun: Datum, mut evaled_args: Vec<Datum>, env_ref: EnvRef) -> Datum {
        match self.apply(fun, evaled_args, env_ref.clone()).unwrap() {
            TCOWrapper::Return(result) => result,
            TCOWrapper::TailCall(expr, env) => {
                self.eval(expr, env).unwrap()
            }
        }
    }

    pub fn eval(&mut self, expr: Expression, mut env_ref: EnvRef) -> LispResult {
        self.level += 1;
        // println!("Evaling on level {} and env {}", self.level, ienv_ref);
        let mut maybe_expr = Some(expr);

        while let Some(e) = maybe_expr {
            let res = match e {
                Expression::If(cond, cons, alt) => self.eval_sf_if(*cond, *cons, *alt, env_ref),
                Expression::Do(es, last) => self.eval_sf_do(es, *last, env_ref),
                // Expression::Case(e, cases, else_case) => self.eval_sf_case(*e, cases, *else_case, env_ref),
                Expression::Quote(datum) => Ok(TCOWrapper::Return(*datum)),
                Expression::Definition(name, value) => self.eval_sf_definition(name, *value, env_ref),
                Expression::MacroDefinition(name, value) => {
                    self.macros.insert(name, *value);
                    Ok(TCOWrapper::Return(Datum::Undefined))
                },
                Expression::Assignment(name, value) => self.eval_sf_assignment(name, *value, env_ref),
                Expression::ListPush(name, value) => self.eval_sf_list_push(name, *value, env_ref),
                Expression::ListRef(name, value) => self.eval_sf_list_ref(name, *value, env_ref),
                Expression::ListSet(name, index, value) => self.eval_sf_list_set(name, *index, *value, env_ref),
                Expression::Symbol(key) => {
                    let env = env_ref.borrow();
                    match env.find_def(&key) {
                        Some(value) => Ok(TCOWrapper::Return(value.borrow().clone())),
                        None => {
                            panic!("Key not found: {}", self.symbol_table.name(key));
                        }
                    } 
                },
                Expression::FunctionCall(fun, args) => {
                    let f = self.eval(*fun, env_ref.clone())?;
                    let evaled_args = self.eval_list(args, env_ref.clone());
                    self.apply(f, evaled_args, env_ref)
                },
                Expression::BuiltinFunctionCall(fun, args) => {
                    let mut evaled_args = self.eval_list(args, env_ref.clone());
                    Ok(TCOWrapper::Return(fun(evaled_args.as_mut_slice(), self, env_ref)?))
                },
                Expression::LambdaDef(params, defaults, body, lambda_type) => {
                    Ok(TCOWrapper::Return(Datum::Lambda(
                        Lambda{
                            env: env_ref,
                            params: params,
                            defaults: defaults,
                            body: body,
                            kind: lambda_type
                        }
                    )))
                },
                Expression::SelfEvaluating(v) => Ok(TCOWrapper::Return(*v)),
                _ => panic!("Expression not valid in this context")
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
