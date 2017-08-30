use ::Value;
use ::LispResult;
use ::LispErr::*;

use std::fs::File;
use std::io::Read;

use time;

use env::*;
use parser;
use desugar;
use builtin::Builtin;

pub struct Evaluator { envs: EnvArena, builtin: Builtin }

macro_rules! check_arity {
    ($args: ident, $number: expr) => {
        if $args.len() != $number {
            return Err(InvalidNumberOfArguments);
        }
    }
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator { envs: EnvArena::new(), builtin: Builtin::new() }
    }

    pub fn make_env(&mut self, parent: Option<EnvRef>) -> EnvRef {
        self.envs.make_env(parent)
    }

    // If the key is already set in the current env,
    // throw an error,
    // otherwise define it
    pub fn sf_def(&mut self, args: &[Value], env_ref: EnvRef) -> LispResult {
        check_arity!(args, 2);

        if let Value::Atom(ref a) = args[0] {
            let value = self.eval(&args[1], env_ref)?;
            if self.envs.define_into(env_ref, a, value) {
                Ok(Value::Undefined)
            } else {
                Err(DefinitionAlreadyDefined)
            }
        } else {
            Err(InvalidTypeOfArguments)
        }
    }

    // Walk up the env tree until key is set,
    // then change its value
    pub fn sf_set(&mut self, args: &[Value], env_ref: EnvRef) -> LispResult {
        check_arity!(args, 2);

        if let Value::Atom(ref a) = args[0] {
            let value = self.eval(&args[1], env_ref)?;
            if self.envs.set_into(env_ref, a, value) {
                Ok(Value::Undefined)
            } else {
                Err(DefinitionNotFound)
            }
        } else {
            Err(InvalidTypeOfArguments)
        }
    }

    pub fn sf_quote(&mut self, args: &[Value], _: EnvRef) -> LispResult {
        check_arity!(args, 1);

        match args[0] {
            // Value::List(ref l) => self.sf_list(&l[..]),
            ref other => Ok(other.clone())
        }
    }

    pub fn sf_list(&mut self, args: &[Value], env_ref: EnvRef) -> LispResult {
        if args.len() == 0 {
            Ok(Value::Nil)
        } else {
            let vals: Result<Vec<Value>, _> =
                args.iter().map(|v| self.eval(v, env_ref)).collect();
            Ok(Value::List(vals?))
        }
    }

    pub fn sf_lambda(&mut self, args: &[Value], env_ref: EnvRef) -> LispResult {
        check_arity!(args, 2);

        let mut params: Vec<String> = Vec::new();

        if let Value::List(ref elems) = args[0] {
            for a in elems {
                if let Value::Atom(ref v) = *a {
                    params.push(v.clone())
                } else {
                    return Err(InvalidTypeOfArguments);
                }
            }
        } else {
            return Err(InvalidTypeOfArguments)
        }

        let body = args[1].clone();
        Ok(Value::Lambda(env_ref, params, Box::new(body)))
    }

    pub fn sf_if(&mut self, args: &[Value], env_ref: EnvRef) -> LispResult {
        check_arity!(args, 3);

        let cond = args.get(0).unwrap();
        let cons = args.get(1).unwrap();

        let default_alt = Value::Nil;
        let alt = args.get(2).unwrap_or(&default_alt);

        match self.eval(&cond, env_ref)? {
            Value::Bool(true) => self.eval(cons, env_ref),
            Value::Bool(false) => self.eval(alt, env_ref),
            _ => Err(InvalidTypeOfArguments)
        }
    }

    pub fn sf_cond(&mut self, args: &[Value], env_ref: EnvRef) -> LispResult {
        for arg in args.iter() {
            if let Value::List(ref elems) = *arg {
                if elems.len() != 2 {
                    return Err(InvalidTypeOfArguments);
                }

                let cond = elems.get(0).unwrap();
                let cons = elems.get(1).unwrap();

                // TODO this does not check if "else" comes last
                if *cond == Value::Atom("else".to_string()) {
                    return self.eval(cons, env_ref);
                } else {
                    let res = self.eval(cond, env_ref)?;
                    if res == Value::Bool(true) {
                        return self.eval(cons, env_ref);
                    } else {
                        continue
                    }
                }
            } else {
                return Err(InvalidTypeOfArguments);
            }
        }

        Ok(Value::Nil)
    }

    pub fn sf_benchmark(&mut self, args: &[Value], env_ref: EnvRef) -> LispResult {
        check_arity!(args, 2);

        if let Value::Number(iterations) = self.eval(&args[0], env_ref)? {
            let mut res = Value::Nil;
            let start = time::now();

            for _ in 0..iterations {
                res = self.eval(&args[1], env_ref)?;
            }

            println!("Benchmark Result: {}", time::now() - start);
            Ok(res)
        } else {
            Err(InvalidNumberOfArguments)
        }
    }

    pub fn sf_begin(&mut self, args: &[Value], env_ref: EnvRef) -> LispResult {
        let mut result = Ok(Value::Nil);

        // TODO: Fail all if one threw an error?
        for a in args.iter() {
            result = self.eval(a, env_ref);
        }

        result
    }

    // TODO: Allow `and` and `or` to operate on all types of values
    pub fn sf_and(&mut self, args: &[Value], env_ref: EnvRef) -> LispResult {
        for a in args.iter() {
            if let Value::Bool(b) = self.eval(a, env_ref)? {
                if !b {
                    return Ok(Value::Bool(false))
                }
            } else {
                return Err(InvalidTypeOfArguments);
            }
        }

        return Ok(Value::Bool(true))
    }

    pub fn sf_or(&mut self, args: &[Value], env_ref: EnvRef) -> LispResult {
        for a in args.iter() {
            if let Value::Bool(b) = self.eval(a, env_ref)? {
                if b {
                    return Ok(Value::Bool(true))
                }
            } else {
                return Err(InvalidTypeOfArguments);
            }
        }


        return Ok(Value::Bool(false))
    }

    fn sf_read(&mut self, args: &[Value], env_ref: EnvRef) -> LispResult {
        check_arity!(args, 1);

        if let Value::Str(ref input) = self.eval(&args[0], env_ref)? {
            Ok(parser::parse_value(input))
        } else {
            Err(InvalidTypeOfArguments)
        }
    }

    fn sf_eval(&mut self, args: &[Value], env_ref: EnvRef) -> LispResult {
        check_arity!(args, 1);

        let value = self.eval(&args[0], env_ref)?;
        self.eval(&value, env_ref)
    }

    fn sf_load(&mut self, args: &[Value], env_ref: EnvRef) -> LispResult {
        check_arity!(args, 1);

        if let Value::Str(ref path) = self.eval(&args[0], env_ref)? {
            self.eval_file(path, env_ref)
        } else {
            Err(InvalidTypeOfArguments)
        }
    }

    pub fn apply(&mut self, f: Value, args: &[Value], env_ref: EnvRef) -> LispResult {
        // println!("Applying {:?} to {:?}", args, f);
        if let Value::Lambda(env, params, body) = f {
            let mut e = Environment::new(Some(env));
            if params.len() != args.len() {
                return Err(InvalidNumberOfArguments);
            } else {
                for (p, a) in params.iter().zip(args.iter()) {
                    let value = self.eval(&a, env_ref)?;
                    e.define_into(p, value);
                }
            }

            let child_env_ref = self.envs.add_env(e);
            self.eval(&body, child_env_ref)
        } else {
            Err(InvalidTypeOfArguments)
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
        let mut ret = Value::Nil;

        for v in result.iter() {
            let desugared = desugar::desugar(v);
            ret = self.eval(&desugared, env_ref)?;
        }

        Ok(ret)
    }

    pub fn eval(&mut self, v: &Value, env_ref: EnvRef) -> LispResult {
        // println!("Evaling {}", v);
        match *v {
            Value::List(ref elems) => {
                if elems.len() == 0 {
                    return Err(InvalidNumberOfArguments)
                }

                let args = &elems[1..];
                match elems[0].clone() {
                    Value::Atom(s) => {
                        match s.as_ref() {
                            "def"       => self.sf_def(args, env_ref),
                            "set!"      => self.sf_set(args, env_ref),
                            "load"      => self.sf_load(args, env_ref),
                            "fn"        => self.sf_lambda(args, env_ref),
                            "if"        => self.sf_if(args, env_ref),
                            "cond"      => self.sf_cond(args, env_ref),
                            "do"        => self.sf_begin(args, env_ref),
                            "list"      => self.sf_list(args, env_ref),
                            "quote"     => self.sf_quote(args, env_ref),
                            "read"      => self.sf_read(args, env_ref),
                            "eval"      => self.sf_eval(args, env_ref),
                            "and"       => self.sf_and(args, env_ref),
                            "or"        => self.sf_or(args, env_ref),
                            "benchmark" => self.sf_benchmark(args, env_ref),
                            "debug-env" => {
                                println!("{:?}", self.envs.get_env(env_ref));
                                Ok(Value::Undefined)
                            },
                            "debug-envref" => {
                                println!("{:?}", env_ref);
                                Ok(Value::Undefined)
                            },
                            other    => {
                                // TODO: prevent redefiniton of builtins
                                if self.builtin.is_builtin(other, elems.len() - 1) {
                                    let vals: Result<Vec<Value>, _> =
                                        args.iter().map(|v| self.eval(v, env_ref)).collect();

                                    self.builtin.apply(other, vals?)
                                } else {
                                    // TODO: Find a way to do this with less duplication
                                    let v = self.envs.get(env_ref, &other.to_string()).clone();
                                    self.apply(v, args, env_ref)
                                }
                            }
                        }
                    },
                    other => {
                        let v = self.eval(&other, env_ref)?;
                        self.apply(v, args, env_ref)
                    },
                }
            },
            Value::Atom(ref v) => Ok(self.envs.get(env_ref, &v.to_string()).clone()),
            ref other => Ok(other.clone())
        }
    }
}
