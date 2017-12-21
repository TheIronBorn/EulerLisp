use ::Datum;
use ::LispFn;
use ::LispResult;
use ::LispErr;
use ::Promise;
use ::LambdaType;
use ::Expression;
use ::Condition;
use ::LispErr::*;
use symbol_table::SymbolTable;

use std::fs;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

use time;

use env::*;
use parser;
use desugar;
use builtin;
use preprocess;

use macros;

pub struct Evaluator {
    envs: EnvArena,
    level: i64,
    symbol_table: SymbolTable
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

    pub fn sf_cond(&mut self, conditions: Vec<Condition>, else_case: Expression, env_ref: EnvRef) -> TCOResult {
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


    pub fn sf_and(&mut self, es: Vec<Expression>, last: Expression, env_ref: EnvRef) -> TCOResult {
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

    pub fn sf_or(&mut self, es: Vec<Expression>, last: Expression, env_ref: EnvRef) -> TCOResult {
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

    // fn sf_read(&mut self, args: &[Datum], env_ref: EnvRef) -> TCOResult {
    //     check_arity!(args, 1);

    //     if let Datum::Str(ref input) = self.eval(&args[0], env_ref)? {
    //         Ok(TCOWrapper::Return(parser::parse_datum(input)))
    //     } else {
    //         Err(InvalidTypeOfArguments)
    //     }
    // }

    // fn sf_eval(&mut self, args: &[Datum], env_ref: EnvRef) -> TCOResult {
    //     check_arity!(args, 1);

    //     let value = self.eval(&args[0], env_ref)?;
    //     Ok(TCOWrapper::Return(self.eval(&value, env_ref)?))
    // }

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
    //                     self.envs.set_into(env_ref, name, new);
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

    // TODO: Refactor out usage of unwrap()
    pub fn apply(&mut self, f: Datum, evaled_args: Vec<Datum>) -> TCOResult {
        match f {
            Datum::Lambda(env, params, body, lambda_type) => {
                let child_env = self.make_env(Some(env));

                match lambda_type {
                    LambdaType::Var => {
                        self.envs.define_into_sym(child_env, params[0].clone(), Datum::List(evaled_args));
                    },
                    LambdaType::List => {
                        if evaled_args.len() != params.len() {
                            return Err(InvalidNumberOfArguments);
                        } else {
                            for (p, a) in params.iter().zip(evaled_args.iter()) {
                                self.envs.define_into_sym(child_env, p.clone(), a.clone());
                            }
                        }
                    },
                    LambdaType::DottedList => {
                        // The last param can be a list of 0... args
                        if evaled_args.len() < (params.len() - 1) {
                            return Err(InvalidNumberOfArguments);
                        } else {
                            for (p, value) in params[0..(params.len() - 1)].iter().zip(evaled_args.clone()) {
                                self.envs.define_into_sym(child_env, p.clone(), value);
                            }
                            let rest: Vec<Datum> = evaled_args.iter().skip(params.len() - 1).cloned().collect();
                            self.envs.define_into_sym(child_env, params[params.len() - 1].clone(), Datum::List(evaled_args));
                        }
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

    pub fn eval_str(&mut self, input: &str, env_ref: EnvRef) -> LispResult {
        let result = parser::parse_program(input);
        let mut ret = Datum::Nil;

        for v in result.iter() {
            let desugared = desugar::desugar(v);
            let preprocessed = preprocess::preprocess(desugared, &mut self.symbol_table)?;
            println!("Preprocessed: {:?}", &preprocessed);
            match self.eval(preprocessed, env_ref) {
                Ok(res) => ret = res,
                Err(msg) => println!("!! {}", msg)
            }
        }

        Ok(ret)
    }

    pub fn eval(&mut self, expr: Expression, ienv_ref: EnvRef) -> LispResult {
        self.level += 1;
        // println!("Evaling {:?} on level {}", expr, self.level);
        let mut maybe_expr = Some(expr);
        let mut env_ref = ienv_ref;

        while let Some(e) = maybe_expr {
            let res = match e {
                Expression::If(cond, cons, alt) => {
                    match self.eval(*cond, env_ref)? {
                        Datum::Bool(false) => Ok(TCOWrapper::TailCall(*alt, env_ref)),
                        _ => Ok(TCOWrapper::TailCall(*cons, env_ref)),
                    }
                },
                Expression::Do(es, last) => {
                    for e in es.into_iter() {
                        self.eval(e, env_ref)?;
                    }
                    Ok(TCOWrapper::TailCall(*last, env_ref))
                },
                Expression::And(es, last) => self.sf_and(es, *last, env_ref),
                Expression::Or(es, last) => self.sf_or(es, *last, env_ref),
                Expression::Conditional(conditions, else_case) => self.sf_cond(conditions, *else_case, env_ref),
                Expression::Quote(datum) => Ok(TCOWrapper::Return(*datum)),
                Expression::Definition(name, value) => {
                    let value = self.eval(*value, env_ref)?;
                    if self.envs.define_into_sym(env_ref, name, value) {
                        Ok(TCOWrapper::Return(Datum::Undefined))
                    } else {
                        Err(DefinitionAlreadyDefined)
                    }
                },
                Expression::Assignment(name, value) => {
                    let value = self.eval(*value, env_ref)?;
                    if self.envs.set_into_sym(env_ref, name, value) {
                        Ok(TCOWrapper::Return(Datum::Undefined))
                    } else {
                        Err(DefinitionNotFound)
                    }
                },
                Expression::Symbol(name) => {
                    let value = self.envs.get_sym(env_ref, name);
                    Ok(TCOWrapper::Return(value.clone()))
                },
                Expression::Bool(v) => Ok(TCOWrapper::Return(Datum::Bool(v))),
                Expression::Number(v) => Ok(TCOWrapper::Return(Datum::Number(v))),
                Expression::Character(v) => Ok(TCOWrapper::Return(Datum::Character(v))),
                Expression::Str(v) => Ok(TCOWrapper::Return(Datum::Str(v))),
                Expression::FunctionCall(fun, args) => {
                    let f = self.eval(*fun, env_ref)?;
                    let evaled_args = args.into_iter().map(|a| self.eval(a, env_ref).unwrap()).collect();
                    self.apply(f, evaled_args)
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

        self.envs.free(env_ref);
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
