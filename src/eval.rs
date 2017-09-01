use ::Value;
use ::LispFn;
use ::LispResult;
use ::Promise;
use ::LispErr::*;

use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

use time;

use env::*;
use parser;
use desugar;
use builtin;

pub struct Evaluator { envs: EnvArena, level: i64 }

pub enum EvalResult {
    Return(LispResult),
    TailCall(Value, EnvRef),
}

use self::EvalResult::*;

macro_rules! check_arity {
    ($args: ident, $number: expr) => {
        if $args.len() != $number {
            return Err(InvalidNumberOfArguments);
        }
    }
}

macro_rules! check_arity2 {
    ($args: ident, $number: expr) => {
        if $args.len() != $number {
            return Return(Err(InvalidNumberOfArguments));
        }
    }
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator { envs: EnvArena::new(), level: 0 }
    }

    pub fn make_root_env(&mut self) -> EnvRef {
        let mut hm: HashMap<String, Value> = HashMap::new(); 
        builtin::load(&mut hm);
        self.envs.add_env(hm)
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

    pub fn sf_if(&mut self, args: &[Value], env_ref: EnvRef) -> EvalResult {
        check_arity2!(args, 3);

        let cond = args.get(0).unwrap();
        let cons = args.get(1).unwrap();

        let default_alt = Value::Nil;
        let alt = args.get(2).unwrap_or(&default_alt);

        match self.eval(&cond, env_ref) {
            Err(e) => Return(Err(e)),
            Ok(v) => match v {
                Value::Bool(true) => TailCall(cons.clone(), env_ref),
                Value::Bool(false) => TailCall(alt.clone(), env_ref),
                _ => Return(Err(InvalidTypeOfArguments))
            }
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

    pub fn sf_begin(&mut self, args: &[Value], env_ref: EnvRef) -> EvalResult {

        // TODO: Fail if one of them threw an error
        for i in (0..(args.len() - 1)) {
            self.eval(&args[i], env_ref);
        }

        TailCall(args[args.len() - 1].clone(), env_ref)
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

    fn sf_delay(&mut self, args: &[Value], env_ref: EnvRef) -> LispResult {
        check_arity!(args, 1);
        Ok(Value::Promise(Promise::Delayed(Box::new(args[0].clone()))))
    }

    fn sf_force(&mut self, args: &[Value], env_ref: EnvRef) -> LispResult {
        check_arity!(args, 1);

        match args[0] {
            Value::Atom(ref name) => {
                let val = self.envs.get(env_ref, name).clone();
                match val {
                    Value::Promise(ref p) => {
                        let res = self.force_promise(p, env_ref)?;
                        let new = Value::Promise(Promise::Result(Box::new(res.clone())));
                        self.envs.set_into(env_ref, name, new);
                        Ok(res)
                    },
                    ref other => self.eval(other, env_ref),
                }
            },
            Value::Promise(ref p) => self.force_promise(p, env_ref),
            ref other => self.eval(other, env_ref),
        }
    }

    fn force_promise(&mut self, p: &Promise, env_ref: EnvRef) -> LispResult {
        match *p {
            Promise::Result(ref r) => Ok(*r.clone()),
            Promise::Delayed(ref r) => self.eval(&(*r.clone()), env_ref),
        }
    }

    pub fn apply(&mut self, f: Value, args: &[Value], env_ref: EnvRef) -> EvalResult {
        // println!("Applying {:?} to {:?}", args, f);
        match f {
            Value::Lambda(env, params, body) => {
                let child_env = self.make_env(Some(env));
                if params.len() != args.len() {
                    return Return(Err(InvalidNumberOfArguments));
                } else {
                    for (p, a) in params.iter().zip(args.iter()) {
                        // TODO: Refactor temp unwrap
                        // let value = self.eval(&a, env_ref)?;
                        let value = self.eval(&a, env_ref).unwrap();
                        self.envs.define_into(child_env, p, value);
                    }
                }
                // Return(self.eval(&body, child_env))
                TailCall((*body).clone(), child_env)
            },
            Value::Builtin(LispFn(fun)) => {
                let vals: Result<Vec<Value>, _> =
                    args.iter().map(|v| self.eval(v, env_ref)).collect();
                // TODO: Refactor temp unwrap
                // Return(fun(vals?))
                Return(fun(vals.unwrap()))
            },
            _ => Return(Err(InvalidTypeOfArguments)),
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

    pub fn eval(&mut self, iv: &Value, ienv_ref: EnvRef) -> LispResult {
        self.level += 1;
        // println!("Evaling {} on level {}", iv, self.level);
        let mut ast = Some(iv.clone());
        let mut env_ref = ienv_ref;

        while let Some(v) = ast {
            let res = match v {
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
                                "if"        => {
                                    match self.sf_if(args, env_ref) {
                                        Return(v) => v,
                                        TailCall(a, e) => {
                                            ast = Some(a.clone());
                                            env_ref = e;
                                            continue;
                                        }
                                    }
                                },
                                "cond"      => self.sf_cond(args, env_ref),
                                "do"        => {
                                    match self.sf_begin(args, env_ref) {
                                    // match self.sf_if(args, env_ref) {
                                        Return(v) => v,
                                        TailCall(a, e) => {
                                            ast = Some(a.clone());
                                            env_ref = e;
                                            continue;
                                        }
                                    }
                                },
                                "list"      => self.sf_list(args, env_ref),
                                "quote"     => self.sf_quote(args, env_ref),
                                "read"      => self.sf_read(args, env_ref),
                                "eval"      => self.sf_eval(args, env_ref),
                                "and"       => self.sf_and(args, env_ref),
                                "or"        => self.sf_or(args, env_ref),
                                "benchmark" => self.sf_benchmark(args, env_ref),
                                "delay"     => self.sf_delay(args, env_ref),
                                "force"     => self.sf_force(args, env_ref),
                                "debug-env" => {
                                    println!("{:?}", self.envs.get_env(env_ref));
                                    Ok(Value::Undefined)
                                },
                                "debug-envref" => {
                                    println!("{:?}", env_ref);
                                    Ok(Value::Undefined)
                                },
                                other    => {
                                    // TODO: Find a way to do this with less duplication
                                    let v = self.envs.get(env_ref, &other.to_string()).clone();
                                    match self.apply(v, args, env_ref) {
                                        Return(v) => v,
                                        TailCall(a, e) => {
                                            ast = Some(a.clone());
                                            env_ref = e;
                                            continue;
                                        }
                                    }
                                }
                            }
                        },
                        other => {
                            let v = self.eval(&other, env_ref)?;
                            match self.apply(v, args, env_ref) {
                                Return(v) => v,
                                TailCall(a, e) => {
                                    ast = Some(a.clone());
                                    env_ref = e;
                                    continue;
                                }
                            }
                        },
                    }
                },
                Value::Atom(ref v) => Ok(self.envs.get(env_ref, &v.to_string()).clone()),
                ref other => Ok(other.clone())
            };

            self.level -= 1;
            return res;
            // match res {
            //     Return(v) => {
            //         return v
            //     }
            //     TailCall(a) => {
            //         ast = a
            //     }
            // }
        }

        Ok(Value::Undefined)
    }
}
