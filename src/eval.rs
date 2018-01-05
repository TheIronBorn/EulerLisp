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

use std::fs;
use std::fs::File;
use std::io::Read;
use std::collections::{HashMap, BTreeMap};
use std::rc::Rc;
use std::cell::RefCell;

use env::{Env, EnvRef};
use parser;
use desugar;
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
            println!("Loading {}", path_str);
            ev.eval_file(&path_str[..]).expect("Failed to load lib");
        }

        ev
    }

    fn eval_sf_case(&mut self, expr: Expression, cases: BTreeMap<Datum, Expression>, else_case: Expression, env_ref: EnvRef) -> TCOResult {
        let res = self.eval(expr, env_ref.clone())?;
        if let Some(cons) = cases.get(&res) {
            Ok(TCOWrapper::TailCall(cons.clone(), env_ref))
        } else {
            Ok(TCOWrapper::TailCall(else_case, env_ref))
        }
    }

    // fn sf_load(&mut self, args: &[Datum], env_ref: EnvRef) -> TCOResult {
    //     check_arity!(args, 1);

    //     if let Datum::Str(ref path) = self.eval(&args[0], env_ref)? {
    //         Ok(TCOWrapper::Return(self.eval_file(path, env_ref)?))
    //     } else {
    //         Err(InvalidTypeOfArguments)
    //     }
    // }

    // fn sf_delay(&mut self, args: &[Datum], env_ref: EnvRef) -> TCOResult {
    //     check_arity!(args, 1);
    //     Ok(TCOWrapper::Return(Datum::Promise(Promise::Delayed(env_ref, Box::new(args[0].clone())))))
    // }

    // fn sf_force(&mut self, args: &[Datum], env_ref: EnvRef) -> TCOResult {
    //     check_arity!(args, 1);

    //     match args[0] {
    //         Datum::Symbol(ref name) => {
    //             let val = self.envs.get(env_ref, name).clone();
    //             match val {
    //                 Datum::Promise(ref p) => {
    //                     let res = self.force_promise(p, env_ref)?;
    //                     let new = Datum::Promise(Promise::Result(Box::new(res.clone())));
    //                     self.envs.set(env_ref, name, new);
    //                     Ok(TCOWrapper::Return(res))
    //                 },
    //                 ref other => Ok(TCOWrapper::Return(self.eval(other, env_ref)?)),
    //             }
    //         },
    //         ref other => {
    //             match self.eval(other, env_ref)? {
    //                 Datum::Promise(ref p) => Ok(TCOWrapper::Return(self.force_promise(p, env_ref)?)),
    //                 ref other_ => Ok(TCOWrapper::Return(other_.clone())),
    //             }
    //         }
    //     }
    // }

    // fn force_promise(&mut self, p: &Promise, env_ref: EnvRef) -> LispResult {
    //     match *p {
    //         Promise::Result(ref r) => Ok(*r.clone()),
    //         Promise::Delayed(env_ref_, ref r) => self.eval(&(*r.clone()), env_ref_),
    //     }
    // }

    pub fn apply(&mut self, f: Datum, mut evaled_args: Vec<Datum>) -> TCOResult {
        match f {
            Datum::Lambda(lambda) => {
                let mut child_env = Env::new(Some(lambda.env));

                match lambda.kind {
                    LambdaType::Var => {
                        child_env.define(lambda.params[0], Datum::List(evaled_args));
                    },
                    LambdaType::List => {
                        if evaled_args.len() != lambda.params.len() {
                            return Err(InvalidNumberOfArguments);
                        } else {
                            child_env.extend(lambda.params, evaled_args);
                        }
                    },

                    LambdaType::DottedList => {
                        panic!("This syntax is not supported jet");
                    //     // The last param can be a list of 0... args
                    //     if evaled_args.len() < (params.len() - 1) {
                    //         return Err(InvalidNumberOfArguments);
                    //     } else {
                    //         for (p, value) in params[0..(params.len() - 1)].iter().zip(evaled_args.clone()) {
                    //             self.envs.define(child_env, p.clone(), value);
                    //         }
                    //         self.envs.extend(child_env, params[0..(params.len() - 1)].iter().collect(), evaled_args);
                    //         let rest: Vec<Datum> = evaled_args.iter().skip(params.len() - 1).cloned().collect();
                    //         self.envs.define(child_env, params[params.len() - 1].clone(), Datum::List(evaled_args));
                    //     }
                    }
                }

                Ok(TCOWrapper::TailCall(*lambda.body, Rc::new(RefCell::new(child_env))))
            },
            Datum::Builtin(LispFn(fun, arity)) => {
                arity.check(evaled_args.len());
                Ok(TCOWrapper::Return(fun(evaled_args.as_mut_slice())?))
            },
            _ => Err(InvalidTypeOfArguments),
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

        for v in result.iter() {
            let env_ref = self.root_env.clone();

            let desugared = desugar::desugar(v);
            let preprocessed = preprocess::preprocess(desugared, &mut self.symbol_table,
                                                      &self.builtins)?;
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

    fn eval_sf_vector_push(&mut self, key: Symbol, value: Expression, env_ref: EnvRef) -> TCOResult {
        let value = self.eval(value, env_ref.clone())?;
        let env = env_ref.borrow();

        if let Some(binding) = env.find_def(&key) {
            match *binding.borrow_mut() {
                Datum::Vector(ref mut elements) => {
                    elements.push(value);
                    Ok(TCOWrapper::Return(Datum::Undefined))
                },
                _ => {
                    Err(InvalidTypeOfArguments)
                }
            }
        } else {
            panic!("Definition not found: {}", self.symbol_table.name(key));
        }
    }

    fn eval_sf_vector_ref(&mut self, key: Symbol, index: Expression, env_ref: EnvRef) -> TCOResult {
        let vindex = self.eval(index, env_ref.clone())?;
        if let Datum::Number(index) = vindex {
            let env = env_ref.borrow();

            if let Some(binding) = env.find_def(&key) {
                match *binding.borrow_mut() {
                    Datum::Vector(ref mut elements) => {
                        if let Some(elem) = elements.get(index as usize) {
                            Ok(TCOWrapper::Return(elem.clone()))
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

    fn eval_sf_vector_set(&mut self, key: Symbol, index: Expression, value: Expression, env_ref: EnvRef) -> TCOResult {
        let vindex = self.eval(index, env_ref.clone())?;
        let value = self.eval(value, env_ref.clone())?;

        if let Datum::Number(index) = vindex {
            let env = env_ref.borrow();

            if let Some(binding) = env.find_def(&key) {
                match *binding.borrow_mut() {
                    Datum::Vector(ref mut elements) => {
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

    fn eval_sf_symbol_function_call(&mut self, fun: Symbol, args: Vec<Expression>, env_ref: EnvRef) -> TCOResult {
        let f = self.eval(Expression::Symbol(fun), env_ref.clone())?;
        let evaled_args = self.eval_list(args, env_ref);
        self.apply(f, evaled_args)
    }

    fn eval_special_apply(&mut self, args: Vec<Datum>) -> TCOResult {
        let f = args.get(0).unwrap();
        let argslist = args.get(1).unwrap();
        if let Datum::List(ref args_) = *argslist {
            self.apply(f.clone(), args_.clone())
        } else {
            Err(InvalidTypeOfArguments)
        }
    }

    fn eval_special_read(&mut self, args: Vec<Datum>) -> TCOResult {
        let arg = args.get(0).unwrap();
        if let Datum::Str(ref input) = *arg {
            let result = parser::parse_datum(input.as_ref());
            Ok(TCOWrapper::Return(result))
        } else {
            Err(InvalidTypeOfArguments)
        }
    }

    fn eval_special_eval(&mut self, args: Vec<Datum>, env_ref: EnvRef) -> TCOResult {
        let arg = args.get(0).unwrap();
        let desugared = desugar::desugar(arg);
        let preprocessed = preprocess::preprocess(desugared, &mut self.symbol_table, &self.builtins)?;
        Ok(TCOWrapper::Return(self.eval(preprocessed, env_ref)?))
    }

    pub fn eval(&mut self, expr: Expression, mut env_ref: EnvRef) -> LispResult {
        self.level += 1;
        // println!("Evaling on level {} and env {}", self.level, ienv_ref);
        let mut maybe_expr = Some(expr);

        while let Some(e) = maybe_expr {
            let res = match e {
                Expression::If(cond, cons, alt) => self.eval_sf_if(*cond, *cons, *alt, env_ref),
                Expression::Do(es, last) => self.eval_sf_do(es, *last, env_ref),
                Expression::Case(e, cases, else_case) => self.eval_sf_case(*e, cases, *else_case, env_ref),
                Expression::Quote(datum) => Ok(TCOWrapper::Return(*datum)),
                Expression::Definition(name, value) => self.eval_sf_definition(name, *value, env_ref),
                Expression::MacroDefinition(name, value) => {
                    self.macros.insert(name, *value);
                    Ok(TCOWrapper::Return(Datum::Undefined))
                },
                Expression::Assignment(name, value) => self.eval_sf_assignment(name, *value, env_ref),
                Expression::VectorPush(name, value) => self.eval_sf_vector_push(name, *value, env_ref),
                Expression::VectorRef(name, value) => self.eval_sf_vector_ref(name, *value, env_ref),
                Expression::VectorSet(name, index, value) => self.eval_sf_vector_set(name, *index, *value, env_ref),
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
                    let evaled_args = self.eval_list(args, env_ref);
                    self.apply(f, evaled_args)
                },
                Expression::BuiltinFunctionCall(fun, args) => {
                    let mut evaled_args = self.eval_list(args, env_ref);
                    Ok(TCOWrapper::Return(fun(evaled_args.as_mut_slice())?))
                },
                Expression::SymbolFunctionCall(fun, args) => self.eval_sf_symbol_function_call(fun, args, env_ref),
                Expression::SpecialFunctionCall(fun, args) => {
                    let evaled_args = self.eval_list(args, env_ref.clone());
                    match fun.as_ref() {
                        "apply" => self.eval_special_apply(evaled_args),
                        "eval" => self.eval_special_eval(evaled_args, env_ref),
                        "read" => self.eval_special_read(evaled_args),
                        _ => panic!("Unknown builtin function: {}", fun)
                    }
                },
                Expression::LambdaDef(args, body, lambda_type) => {
                    Ok(TCOWrapper::Return(Datum::Lambda(Lambda{ env: env_ref, params: args, body: body, kind: lambda_type })))
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
