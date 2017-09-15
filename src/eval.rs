use ::Datum;
use ::LispFn;
use ::LispResult;
use ::LispErr;
use ::Promise;
use ::LambdaType;
use ::LispErr::*;

use std::fs;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;


use time;

use env::*;
use parser;
use desugar;
use builtin;

use macros;

pub struct Evaluator {
    // symbol_table: SymbolTable,
    envs: EnvArena,
    level: i64
}

pub type TCOResult = Result<TCOWrapper, LispErr>;
pub enum TCOWrapper {
    Return(Datum),
    TailCall(Datum, EnvRef),
}

impl Evaluator {
    pub fn new() -> Self {
        let mut ev = Evaluator {
            // symbol_table: SymbolTable::new(),
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
        self.envs.add_env(hm)
    }

    pub fn make_env(&mut self, parent: Option<EnvRef>) -> EnvRef {
        self.envs.make_env(parent)
    }

    // If the key is already set in the current env,
    // throw an error,
    // otherwise define it
    pub fn sf_def(&mut self, args: &[Datum], env_ref: EnvRef) -> TCOResult {
        check_arity!(args, 2);

        if let Datum::Symbol(ref a) = args[0] {
            let value = self.eval(&args[1], env_ref)?;
            if self.envs.define_into(env_ref, a, value) {
                Ok(TCOWrapper::Return(Datum::Undefined))
            } else {
                Err(DefinitionAlreadyDefined)
            }
        } else {
            Err(InvalidTypeOfArguments)
        }
    }

    // Walk up the env tree until key is set,
    // then change its value
    pub fn sf_set(&mut self, args: &[Datum], env_ref: EnvRef) -> TCOResult {
        check_arity!(args, 2);

        if let Datum::Symbol(ref a) = args[0] {
            let value = self.eval(&args[1], env_ref)?;
            if self.envs.set_into(env_ref, a, value) {
                Ok(TCOWrapper::Return(Datum::Undefined))
            } else {
                Err(DefinitionNotFound)
            }
        } else {
            Err(InvalidTypeOfArguments)
        }
    }

    pub fn sf_quote(&mut self, args: &[Datum], _: EnvRef) -> TCOResult {
        check_arity!(args, 1);

        match args[0] {
            // Datum::List(ref l) => self.sf_list(&l[..]),
            ref other => Ok(TCOWrapper::Return(other.clone()))
        }
    }

    pub fn sf_list(&mut self, args: &[Datum], env_ref: EnvRef) -> TCOResult {
        if args.len() == 0 {
            Ok(TCOWrapper::Return(Datum::Nil))
        } else {
            let vals: Result<Vec<Datum>, _> =
                args.iter().map(|v| self.eval(v, env_ref)).collect();
            Ok(TCOWrapper::Return(Datum::List(vals?)))
        }
    }

    pub fn sf_lambda(&mut self, args: &[Datum], env_ref: EnvRef) -> TCOResult {
        check_arity!(args, 2);

        let mut params: Vec<String> = Vec::new();
        let mut lambda_type: LambdaType;

        match args[0] {
            Datum::Symbol(ref name) => {
                params.push(name.clone());
                lambda_type = LambdaType::Var;
            },
            Datum::List(ref elems) => {
                for a in elems {
                    if let Datum::Symbol(ref v) = *a {
                        params.push(v.clone());
                    } else {
                        return Err(InvalidTypeOfArguments);
                    }
                };
                lambda_type = LambdaType::List;
            },
            Datum::DottedList(ref elems, ref tail) => {
                for a in elems {
                    if let Datum::Symbol(ref v) = *a {
                        params.push(v.clone());
                    } else {
                        return Err(InvalidTypeOfArguments);
                    }
                }
                if let Datum::Symbol(ref v) = **tail {
                    params.push(v.clone());
                }
                lambda_type = LambdaType::DottedList;
            },
            _ => return Err(InvalidTypeOfArguments),
        }

        let body = args[1].clone();
        Ok(TCOWrapper::Return(Datum::Lambda(env_ref, params, Box::new(body), lambda_type)))
    }

    pub fn sf_if(&mut self, args: &[Datum], env_ref: EnvRef) -> TCOResult {
        if !(args.len() == 2 || args.len() == 3) {
            return Err(InvalidNumberOfArguments);
        }

        let cond = args.get(0).unwrap();
        let cons = args.get(1).unwrap();

        let default_alt = Datum::Nil;
        let alt = args.get(2).unwrap_or(&default_alt);

        match self.eval(&cond, env_ref)? {
            Datum::Bool(true) => Ok(TCOWrapper::TailCall(cons.clone(), env_ref)),
            Datum::Bool(false) => Ok(TCOWrapper::TailCall(alt.clone(), env_ref)),
            _ => Err(InvalidTypeOfArguments)
        }
    }

    // TODO: Make else expr a tail call
    pub fn sf_cond(&mut self, args: &[Datum], env_ref: EnvRef) -> TCOResult {
        for arg in args.iter() {
            if let Datum::List(ref elems) = *arg {
                if elems.len() != 2 {
                    return Err(InvalidTypeOfArguments);
                }

                let cond = elems.get(0).unwrap();
                let cons = elems.get(1).unwrap();

                // TODO this does not check if "else" comes last
                if *cond == Datum::Symbol("else".to_string()) {
                    return Ok(TCOWrapper::Return(self.eval(cons, env_ref)?));
                } else {
                    let res = self.eval(cond, env_ref)?;
                    if res == Datum::Bool(true) {
                        return Ok(TCOWrapper::Return(self.eval(cons, env_ref)?));
                    } else {
                        continue
                    }
                }
            } else {
                return Err(InvalidTypeOfArguments);
            }
        }

        Ok(TCOWrapper::Return(Datum::Nil))
    }

    pub fn sf_benchmark(&mut self, args: &[Datum], env_ref: EnvRef) -> TCOResult {
        check_arity!(args, 2);

        if let Datum::Number(iterations) = self.eval(&args[0], env_ref)? {
            let mut res = Datum::Nil;
            let start = time::now();

            for _ in 0..iterations {
                res = self.eval(&args[1], env_ref)?;
            }

            println!("Benchmark Result: {}", time::now() - start);
            Ok(TCOWrapper::Return(res))
        } else {
            Err(InvalidNumberOfArguments)
        }
    }

    pub fn sf_begin(&mut self, args: &[Datum], env_ref: EnvRef) -> TCOResult {
        for i in 0..(args.len() - 1) {
            self.eval(&args[i], env_ref)?;
        }

        Ok(TCOWrapper::TailCall(args[args.len() - 1].clone(), env_ref))
    }

    pub fn sf_and(&mut self, args: &[Datum], env_ref: EnvRef) -> TCOResult {
        for a in args[0..(args.len() - 1)].iter() {
            match self.eval(a, env_ref)? {
                Datum::Bool(b) => {
                    if b == false {
                        return Ok(TCOWrapper::Return(Datum::Bool(false)))
                    }
                },
                _ => (),
            }
        }

        return Ok(TCOWrapper::TailCall(args[args.len()-1].clone(), env_ref))
    }

    pub fn sf_or(&mut self, args: &[Datum], env_ref: EnvRef) -> TCOResult {
        for a in args[0..(args.len() - 1)].iter() {
            match self.eval(a, env_ref)? {
                Datum::Bool(b) => {
                    if b == true {
                        return Ok(TCOWrapper::Return(Datum::Bool(true)));
                    }
                },
                ref v => {
                    return Ok(TCOWrapper::Return(v.clone()));
                },
            }
        }

        return Ok(TCOWrapper::TailCall(args[args.len()-1].clone(), env_ref))
    }

    fn sf_read(&mut self, args: &[Datum], env_ref: EnvRef) -> TCOResult {
        check_arity!(args, 1);

        if let Datum::Str(ref input) = self.eval(&args[0], env_ref)? {
            Ok(TCOWrapper::Return(parser::parse_datum(input)))
        } else {
            Err(InvalidTypeOfArguments)
        }
    }

    fn sf_eval(&mut self, args: &[Datum], env_ref: EnvRef) -> TCOResult {
        check_arity!(args, 1);

        let value = self.eval(&args[0], env_ref)?;
        Ok(TCOWrapper::Return(self.eval(&value, env_ref)?))
    }

    fn sf_load(&mut self, args: &[Datum], env_ref: EnvRef) -> TCOResult {
        check_arity!(args, 1);

        if let Datum::Str(ref path) = self.eval(&args[0], env_ref)? {
            Ok(TCOWrapper::Return(self.eval_file(path, env_ref)?))
        } else {
            Err(InvalidTypeOfArguments)
        }
    }

    fn sf_delay(&mut self, args: &[Datum], env_ref: EnvRef) -> TCOResult {
        check_arity!(args, 1);
        Ok(TCOWrapper::Return(Datum::Promise(Promise::Delayed(env_ref, Box::new(args[0].clone())))))
    }

    fn sf_force(&mut self, args: &[Datum], env_ref: EnvRef) -> TCOResult {
        check_arity!(args, 1);

        match args[0] {
            Datum::Symbol(ref name) => {
                let val = self.envs.get(env_ref, name).clone();
                match val {
                    Datum::Promise(ref p) => {
                        let res = self.force_promise(p, env_ref)?;
                        let new = Datum::Promise(Promise::Result(Box::new(res.clone())));
                        self.envs.set_into(env_ref, name, new);
                        Ok(TCOWrapper::Return(res))
                    },
                    ref other => Ok(TCOWrapper::Return(self.eval(other, env_ref)?)),
                }
            },
            ref other => {
                match self.eval(other, env_ref)? {
                    Datum::Promise(ref p) => Ok(TCOWrapper::Return(self.force_promise(p, env_ref)?)),
                    ref other_ => Ok(TCOWrapper::Return(other_.clone())),
                }
            }
        }
    }

    fn force_promise(&mut self, p: &Promise, env_ref: EnvRef) -> LispResult {
        match *p {
            Promise::Result(ref r) => Ok(*r.clone()),
            Promise::Delayed(env_ref_, ref r) => self.eval(&(*r.clone()), env_ref_),
        }
    }

    // TODO: Refactor out usage of unwrap()
    pub fn apply(&mut self, f: Datum, args: &[Datum], env_ref: EnvRef) -> TCOResult {
        match f {
            Datum::Lambda(env, params, body, lambda_type) => {
                let child_env = self.make_env(Some(env));

                match lambda_type {
                    LambdaType::Var => {
                        let evaled_args = args.iter().map(|a| self.eval(&a, env_ref).unwrap()).collect();
                        self.envs.define_into(child_env, &params[0], Datum::List(evaled_args));
                    },
                    LambdaType::List => {
                        if args.len() != params.len() {
                            return Err(InvalidNumberOfArguments);
                        } else {
                            for (p, a) in params.iter().zip(args.iter()) {
                                let value = self.eval(&a, env_ref).unwrap();
                                self.envs.define_into(child_env, p, value);
                            }
                        }
                    },
                    LambdaType::DottedList => {
                        // The last param can be a list of 0... args
                        if args.len() < (params.len() - 1) {
                            return Err(InvalidNumberOfArguments);
                        } else {
                            for (p, a) in params[0..(params.len() - 1)].iter().zip(args.iter()) {
                                let value = self.eval(&a, env_ref).unwrap();
                                self.envs.define_into(child_env, p, value);
                            }
                            let evaled_args = args[(params.len() - 1)..].iter().map(|a| self.eval(&a, env_ref).unwrap()).collect();
                            self.envs.define_into(child_env, &params[params.len() - 1], Datum::List(evaled_args));
                        }
                    }
                }

                Ok(TCOWrapper::TailCall((*body).clone(), child_env))
            },
            Datum::Builtin(LispFn(fun)) => {
                let vals: Result<Vec<Datum>, _> =
                    args.iter().map(|v| self.eval(v, env_ref)).collect();
                Ok(TCOWrapper::Return(fun(vals?)?))
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
            ret = self.eval(&desugared, env_ref)?;
        }

        Ok(ret)
    }

    pub fn eval(&mut self, iv: &Datum, ienv_ref: EnvRef) -> LispResult {
        self.level += 1;
        // println!("Evaling {} on level {}", iv, self.level);
        let mut ast = Some(iv.clone());
        let mut env_ref = ienv_ref;

        while let Some(v) = ast {
            let res = match v {
                Datum::List(ref elems) => {
                    if elems.len() == 0 {
                        return Err(InvalidNumberOfArguments)
                    }

                    let args = &elems[1..];
                    match elems[0].clone() {
                        Datum::Symbol(s) => {
                            match s.as_ref() {
                                "def"       => self.sf_def(args, env_ref),
                                "set!"      => self.sf_set(args, env_ref),
                                "load"      => self.sf_load(args, env_ref),
                                "fn"        => self.sf_lambda(args, env_ref),
                                "if"        => self.sf_if(args, env_ref),
                                "cond"      => self.sf_cond(args, env_ref),
                                "do"        => self.sf_begin(args, env_ref),
                                "or"        => self.sf_or(args, env_ref),
                                "and"       => self.sf_and(args, env_ref),
                                "list"      => self.sf_list(args, env_ref),
                                "quote"     => self.sf_quote(args, env_ref),
                                "read"      => self.sf_read(args, env_ref),
                                "eval"      => self.sf_eval(args, env_ref),
                                "benchmark" => self.sf_benchmark(args, env_ref),
                                "delay"     => self.sf_delay(args, env_ref),
                                "force"     => self.sf_force(args, env_ref),
                                "debug-env" => {
                                    println!("{:?}", self.envs.get_env(env_ref));
                                    Ok(TCOWrapper::Return(Datum::Undefined))
                                },
                                "debug-envref" => {
                                    println!("{:?}", env_ref);
                                    Ok(TCOWrapper::Return(Datum::Undefined))
                                },
                                other    => {
                                    // TODO: Find a way to do this with less duplication
                                    let v = self.envs.get(env_ref, &other.to_string()).clone();
                                    self.apply(v, args, env_ref)
                                }
                            }
                        },
                        other => {
                            let v = self.eval(&other, env_ref)?;
                            self.apply(v, args, env_ref)
                        },
                    }
                },
                Datum::Symbol(ref v) => Ok(TCOWrapper::Return(self.envs.get(env_ref, &v.to_string()).clone())),
                ref other => Ok(TCOWrapper::Return(other.clone()))
            };

            match res? {
                TCOWrapper::Return(v) => {
                    self.level -= 1;
                    return Ok(v)
                },
                TCOWrapper::TailCall(a, e) => {
                    ast = Some(a.clone());
                    env_ref = e;
                    continue;
                }
            }
        }
        self.level -= 1;
        Ok(Datum::Undefined)
    }
}
