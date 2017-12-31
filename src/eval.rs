use ::Datum;
use ::LispFn;
use ::LispResult;
use ::LispErr;
use ::Promise;
use ::LambdaType;
use ::Expression;
use ::Condition;
use ::Symbol;
use ::LispErr::*;
use symbol_table::SymbolTable;

use std::fs;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

use env::*;
use parser;
use desugar;
use builtin;
use preprocess;

pub struct Evaluator {
    envs: EnvArena,
    level: i64,
    symbol_table: SymbolTable,
    macros: HashMap<Symbol, Expression>
}

pub type TCOResult = Result<TCOWrapper, LispErr>;
pub enum TCOWrapper {
    Return(Datum),
    TailCall(Expression, EnvRef),
}

impl Evaluator {
    pub fn new() -> Self {
        let mut ev = Evaluator {
            symbol_table: SymbolTable::new(),
            macros: HashMap::new(),
            envs: EnvArena::new(),
            level: 0,
        };
        ev.make_root_env();

        let paths = fs::read_dir("./stdlib").unwrap();
        for path in paths {
            let path_str = path.unwrap().path().display().to_string();
            println!("Loading {}", path_str);
            ev.eval_file(&path_str[..], 0);
        }

        ev
    }

    pub fn make_root_env(&mut self) -> EnvRef {
        let mut hm: HashMap<String, Datum> = HashMap::new(); 
        builtin::load(&mut hm);
        self.envs.add_env(hm, &mut self.symbol_table)
    }

    pub fn make_env(&mut self, parent: Option<EnvRef>) -> EnvRef {
        self.envs.make_env(parent)
    }

    fn eval_sf_cond(&mut self, conditions: Vec<Condition>, else_case: Expression, env_ref: EnvRef) -> TCOResult {
        for condition in conditions.into_iter() {
            let Condition(cond, cons) = condition;
            let res = self.eval(*cond, env_ref)?;
            if res == Datum::Bool(true) {
                return Ok(TCOWrapper::TailCall(*cons, env_ref));
            } else {
                continue
            }
        }

        Ok(TCOWrapper::Return(self.eval(else_case, env_ref)?))
    }

    // pub fn sf_benchmark(&mut self, args: &[Datum], env_ref: EnvRef) -> TCOResult {
    //     check_arity!(args, 2);

    //     if let Datum::Number(iterations) = self.eval(&args[0], env_ref)? {
    //         let mut res = Datum::Nil;
    //         let start = time::now();

    //         for _ in 0..iterations {
    //             res = self.eval(&args[1], env_ref)?;
    //         }

    //         println!("Benchmark Result: {}", time::now() - start);
    //         Ok(TCOWrapper::Return(res))
    //     } else {
    //         Err(InvalidNumberOfArguments)
    //     }
    // }

    // pub fn sf_info(&mut self, args: &[Datum], env_ref: EnvRef) -> TCOResult {
    //     check_arity!(args, 0);


    //     println!("Environments: {}", self.envs.size());
    //     println!("Symbols: {}", self.envs.symbol_table.index);
    //     println!("Level: {}", self.level);
    //     Ok(TCOWrapper::Return(Datum::Nil))
    // }


    fn eval_sf_and(&mut self, es: Vec<Expression>, last: Expression, env_ref: EnvRef) -> TCOResult {
        for a in es.into_iter() {
            match self.eval(a, env_ref)? {
                Datum::Bool(v) => {
                    if v == false {
                        return Ok(TCOWrapper::Return(Datum::Bool(false)))
                    }
                },
                _ => (),
            }
        }
        return Ok(TCOWrapper::TailCall(last, env_ref))
    }

    fn eval_sf_or(&mut self, es: Vec<Expression>, last: Expression, env_ref: EnvRef) -> TCOResult {
        for a in es.into_iter() {
            match self.eval(a, env_ref)? {
                Datum::Bool(v) => {
                    if v == true {
                        return Ok(TCOWrapper::Return(Datum::Bool(true)));
                    }
                },
                ref v => {
                    return Ok(TCOWrapper::Return(v.clone()));
                },
            }
        }

        return Ok(TCOWrapper::TailCall(last, env_ref))
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

    pub fn apply(&mut self, f: Datum, evaled_args: Vec<Datum>) -> TCOResult {
        match f {
            Datum::Lambda(env, params, body, lambda_type) => {
                let child_env = self.make_env(Some(env));

                match lambda_type {
                    LambdaType::Var => {
                        self.envs.define(child_env, params[0].clone(), Datum::List(evaled_args));
                    },
                    LambdaType::List => {
                        if evaled_args.len() != params.len() {
                            return Err(InvalidNumberOfArguments);
                        } else {
                            self.envs.extend(child_env, params, evaled_args);
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

                Ok(TCOWrapper::TailCall((*body).clone(), child_env))
            },
            Datum::Builtin(LispFn(fun)) => {
                Ok(TCOWrapper::Return(fun(evaled_args)?))
            },
            _ => Err(InvalidTypeOfArguments),
        } 
    }

    pub fn eval_file(&mut self, path: &str, env_ref: EnvRef) -> LispResult {
        // TODO: Add IOError type
        let mut f = File::open(path).expect("Could not open file");
        let mut input = String::new();

        f.read_to_string(&mut input).expect("Could not read file");
        self.eval_str(&input[..], env_ref)
    }

    pub fn lookup_macro(&self, name: Symbol) -> Option<&Expression> {
        self.macros.get(&name)
    }

    pub fn eval_str(&mut self, input: &str, env_ref: EnvRef) -> LispResult {
        let result = parser::parse_program(input);
        let mut ret = Datum::Nil;

        for v in result.iter() {
            let desugared = desugar::desugar(v);
            let preprocessed = preprocess::preprocess(desugared, &mut self.symbol_table)?;
            // println!("Preprocessed: {:?}", &preprocessed);
            match self.eval(preprocessed, env_ref) {
                Ok(res) => ret = res,
                Err(msg) => println!("!! {}", msg)
            }
        }

        Ok(ret)
    }

    fn eval_list(&mut self, exprs: Vec<Expression>, env_ref: EnvRef) -> Vec<Datum> {
        exprs.into_iter().map(|a| self.eval(a, env_ref).unwrap()).collect()
    }

    fn eval_sf_if(&mut self, cond: Expression, cons: Expression, alt: Expression, env_ref: EnvRef) -> TCOResult {
        match self.eval(cond, env_ref)? {
            Datum::Bool(true) => Ok(TCOWrapper::TailCall(cons, env_ref)),
            Datum::Bool(false) => Ok(TCOWrapper::TailCall(alt, env_ref)),
            _ => panic!("Condition of if must return a boolean"),
        }
    }

    fn eval_sf_do(&mut self, expressions: Vec<Expression>, last: Expression, env_ref: EnvRef) -> TCOResult {
        for e in expressions.into_iter() {
            self.eval(e, env_ref)?;
        }
        Ok(TCOWrapper::TailCall(last, env_ref))
    }

    fn eval_sf_definition(&mut self, name: Symbol, value: Expression, env_ref: EnvRef) -> TCOResult {
        let value = self.eval(value, env_ref)?;
        if self.envs.define(env_ref, name, value) {
            Ok(TCOWrapper::Return(Datum::Undefined))
        } else {
            Err(DefinitionAlreadyDefined)
        }
    }

    fn eval_sf_assignment(&mut self, name: Symbol, value: Expression, env_ref: EnvRef) -> TCOResult {
        let value = self.eval(value, env_ref)?;
        if let Some(old) = self.envs.get_mut(env_ref, name) {
            *old = value;
            Ok(TCOWrapper::Return(Datum::Undefined))
        } else {
            Err(DefinitionNotFound)
        }
    }

    fn eval_sf_vector_push(&mut self, name: Symbol, value: Expression, env_ref: EnvRef) -> TCOResult {
        let value = self.eval(value, env_ref)?;
        if let Some(old) = self.envs.get_mut(env_ref, name) {
            match old {
                &mut Datum::Vector(ref mut elements) => {
                    elements.push(value);
                },
                _ => {
                    return Err(InvalidTypeOfArguments);
                }
            }
            Ok(TCOWrapper::Return(Datum::Undefined))
        } else {
            Err(DefinitionNotFound)
        }
    }

    fn eval_sf_vector_set(&mut self, name: Symbol, index: Expression, value: Expression, env_ref: EnvRef) -> TCOResult {
        let vindex = self.eval(index, env_ref)?;
        let value = self.eval(value, env_ref)?;

        if let Datum::Number(index) = vindex {
            if let Some(old) = self.envs.get_mut(env_ref, name) {
                match old {
                    &mut Datum::Vector(ref mut elements) => {
                        if let Some(elem) = elements.get_mut(index as usize) {
                            *elem = value;
                        } else {
                            // TODO: Index out of bounds
                            return Err(InvalidTypeOfArguments);
                        }
                    },
                    _ => {
                        return Err(InvalidTypeOfArguments);
                    }
                }
                Ok(TCOWrapper::Return(Datum::Undefined))
            } else {
                Err(DefinitionNotFound)
            }
        } else {
            Err(InvalidTypeOfArguments)
        }
    }

    fn eval_sf_symbol_function_call(&mut self, fun: Symbol, args: Vec<Expression>, env_ref: EnvRef) -> TCOResult {
        // Hack to get around issues with non-lexical borrowing
        // that will be fixed in a future release of rust
        // let mac = self.lookup_macro(fun);

        let f = self.eval(Expression::Symbol(fun), env_ref)?;
        let evaled_args = self.eval_list(args, env_ref);
        self.apply(f, evaled_args)
    }

    fn eval_special_apply(&mut self, args: Vec<Datum>, env_ref: EnvRef) -> TCOResult {
        let f = args.get(0).unwrap();
        let argslist = args.get(1).unwrap();
        if let Datum::List(ref args_) = *argslist {
            self.apply(f.clone(), args_.clone())
        } else {
            Err(InvalidTypeOfArguments)
        }
    }

    fn eval_special_read(&mut self, args: Vec<Datum>, env_ref: EnvRef) -> TCOResult {
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
        let preprocessed = preprocess::preprocess(desugared, &mut self.symbol_table)?;
        Ok(TCOWrapper::Return(self.eval(preprocessed, env_ref)?))
    }

    pub fn eval(&mut self, expr: Expression, ienv_ref: EnvRef) -> LispResult {
        self.level += 1;
        // println!("Evaling on level {} and env {}", self.level, ienv_ref);
        let mut maybe_expr = Some(expr);
        let mut env_ref = ienv_ref;
        let mut last_was_tail = false;

        while let Some(e) = maybe_expr {
            let res = match e {
                Expression::If(cond, cons, alt) => self.eval_sf_if(*cond, *cons, *alt, env_ref),
                Expression::Do(es, last) => self.eval_sf_do(es, *last, env_ref),
                Expression::And(es, last) => self.eval_sf_and(es, *last, env_ref),
                Expression::Or(es, last) => self.eval_sf_or(es, *last, env_ref),
                Expression::Conditional(conditions, else_case) => self.eval_sf_cond(conditions, *else_case, env_ref),
                Expression::Quote(datum) => Ok(TCOWrapper::Return(*datum)),
                Expression::Definition(name, value) => self.eval_sf_definition(name, *value, env_ref),
                Expression::MacroDefinition(name, value) => {
                    self.macros.insert(name, *value);
                    Ok(TCOWrapper::Return(Datum::Undefined))
                },
                Expression::Assignment(name, value) => self.eval_sf_assignment(name, *value, env_ref),
                Expression::VectorPush(name, value) => self.eval_sf_vector_push(name, *value, env_ref),
                Expression::VectorSet(name, index, value) => self.eval_sf_vector_set(name, *index, *value, env_ref),
                Expression::Symbol(ref name) => {
                    match self.envs.get(env_ref, name.clone()) {
                        Some(value) => Ok(TCOWrapper::Return(value.clone())),
                        None => {
                            let Symbol(key) = name.clone();
                            panic!("Key not found: {}", self.symbol_table.name(key));
                        }
                    } 
                },
                Expression::Bool(v) => Ok(TCOWrapper::Return(Datum::Bool(v))),
                Expression::Vector(v) => Ok(TCOWrapper::Return(Datum::Vector(v))),
                Expression::Number(v) => Ok(TCOWrapper::Return(Datum::Number(v))),
                Expression::Character(v) => Ok(TCOWrapper::Return(Datum::Character(v))),
                Expression::Str(v) => Ok(TCOWrapper::Return(Datum::Str(v))),
                Expression::FunctionCall(fun, args) => {
                    let f = self.eval(*fun, env_ref)?;
                    let evaled_args = self.eval_list(args, env_ref);
                    self.apply(f, evaled_args)
                },
                Expression::SymbolFunctionCall(fun, args) => self.eval_sf_symbol_function_call(fun, args, env_ref),
                // TODO: Refactor this to something cleaner & remove `.clone()`s
                Expression::SpecialFunctionCall(fun, args) => {
                    let evaled_args = self.eval_list(args, env_ref);
                    match fun.as_ref() {
                        "apply" => self.eval_special_apply(evaled_args, env_ref),
                        "eval" => self.eval_special_eval(evaled_args, env_ref),
                        "read" => self.eval_special_read(evaled_args, env_ref),
                        _ => panic!("Unknown builtin function: {}", fun)
                    }
                },
                Expression::LambdaDef(args, body, lambda_type) => {
                    Ok(TCOWrapper::Return(Datum::Lambda(env_ref.clone(), args, body, lambda_type)))
                }
                Expression::Lambda(env, args, body, lambda_type) =>
                    Ok(TCOWrapper::Return(Datum::Lambda(env, args, body, lambda_type))),
                Expression::Builtin(v) => Ok(TCOWrapper::Return(Datum::Builtin(v))),
                Expression::Promise(v) => Ok(TCOWrapper::Return(Datum::Promise(v))),
                Expression::Undefined => Ok(TCOWrapper::Return(Datum::Undefined)),
                Expression::Nil => Ok(TCOWrapper::Return(Datum::Nil)),
                // List(Vec<Datum>),
                // DottedList(Vec<Datum>, Box<Datum>),
                _ => panic!("Expression not valid in this context")
            };

            match res? {
                TCOWrapper::Return(v) => {
                    last_was_tail = false;
                    self.level -= 1;
                    return Ok(v)
                },
                TCOWrapper::TailCall(a, e) => {
                    maybe_expr = Some(a);
                    // TODO: Better handling of old envs
                    if (last_was_tail && e != env_ref) {
                        // self.envs.free(env_ref);
                    }
                    last_was_tail = true;
                    env_ref = e;
                    continue;
                }
            }
        }

        self.level -= 1;
        Ok(Datum::Undefined)
    }
}
//                         "debug-env" => {
//                             println!("{:?}", self.envs.get_env(env_ref));
//                             Ok(TCOWrapper::Return(Datum::Undefined))
//                         },
//                         "debug-envref" => {
//                             println!("{:?}", env_ref);
//                             Ok(TCOWrapper::Return(Datum::Undefined))
//                         },
