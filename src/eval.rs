use ::Value;
use ::LispResult;
use ::LispErr::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use std::fs::File;
use std::io::Read;

use env::*;
use parser;
use desugar;

pub struct Evaluator { envs: EnvArena }

impl Evaluator {
    pub fn new() -> Self {
        Evaluator { envs: EnvArena::new() }
    }

    pub fn make_env(&mut self, parent: Option<EnvRef>) -> EnvRef {
        self.envs.make_env(parent)
    }

    // If the key is already set in the current env,
    // throw an error,
    // otherwise define it
    pub fn builtin_def(&mut self, args: &[Value], env_ref: EnvRef) -> LispResult {
        if args.len() != 2 {
            Err(InvalidNumberOfArguments)
        } else {
            match args[0] {
                Value::Atom(ref a) => {
                    let value = self.eval(&args[1], env_ref)?;
                    if self.envs.define_into(env_ref, a, value) {
                        Ok(Value::Atom("ok".to_string()))
                    } else {
                        Err(DefinitionAlreadyDefined)
                    }
                },
                _ => Err(InvalidTypeOfArguments),
            }
        }
    }

    // Walk up the env tree until key is set,
    // then change its value
    pub fn builtin_set(&mut self, args: &[Value], env_ref: EnvRef) -> LispResult {
        if args.len() != 2 {
            Err(InvalidNumberOfArguments)
        } else {
            match args[0] {
                Value::Atom(ref a) => {
                    let value = self.eval(&args[1], env_ref)?;
                    if self.envs.set_into(env_ref, a, value) {
                        Ok(Value::Atom("ok".to_string()))
                    } else {
                        Err(DefinitionNotFound)
                    }
                },
                _ => Err(InvalidTypeOfArguments),
            }
        }
    }

    pub fn builtin_cons(&mut self, args: &[Value], env_ref: EnvRef) -> LispResult {
        if args.len() != 2 {
            Err(InvalidNumberOfArguments)
        } else {
            let fst = self.eval(&args[0], env_ref)?;
            let rst = self.eval(&args[1], env_ref)?;

            match rst {
                Value::Nil => Ok(Value::List(vec![fst])),
                Value::DottedList(ref elems) => {
                    let mut new = elems.clone();
                    new.insert(0, fst);
                    Ok(Value::DottedList(new))
                }
                Value::List(ref elems) => {
                    let mut new = elems.clone();
                    new.insert(0, fst);
                    Ok(Value::List(new))
                }
                other => Ok(Value::DottedList(vec![fst, other]))
            }
        }
    }

    pub fn builtin_quote(&mut self, args: &[Value], env_ref: EnvRef) -> LispResult {
        if args.len() != 1 {
            Err(InvalidNumberOfArguments)
        } else {
            match args[0] {
                // Value::List(ref l) => self.builtin_list(&l[..]),
                ref other => Ok(other.clone())
            }
        }
    }

    pub fn builtin_list(&mut self, args: &[Value], env_ref: EnvRef) -> LispResult {
        if args.len() == 0 {
            Ok(Value::Nil)
        } else {
            let mut vals: Vec<Value> = Vec::new();

            for a in args.iter() {
                match self.eval(a, env_ref) {
                    Ok(v) => vals.push(v),
                    Err(msg) => return Err(msg),
                }
            }

            Ok(Value::List(vals))
        }
    }

    pub fn builtin_lambda(&mut self, args: &[Value], env_ref: EnvRef) -> LispResult {
        if args.len() != 2 {
            Err(InvalidNumberOfArguments)
        } else {
            let mut params: Vec<String> = Vec::new();
            let mut env = Environment {
                bindings: HashMap::new(),
                parent: Some(env_ref)
            };

            match args[0] {
                Value::List(ref elems) => {
                    for a in elems {
                        match *a {
                            Value::Atom(ref v) => params.push(v.clone()),
                            _ => return Err(InvalidTypeOfArguments),
                        }
                    }
                },
                _ => return Err(InvalidTypeOfArguments),
            }

            let child_env_ref = self.envs.add_env(env);

            let body = args[1].clone();
            Ok(Value::Lambda(child_env_ref, params, Box::new(body)))
        }
    }

    pub fn builtin_puts(&mut self, args: &[Value], env_ref: EnvRef) -> LispResult {
        if args.len() != 1 {
            Err(InvalidNumberOfArguments)
        } else {
            let value = self.eval(&args[0], env_ref)?;
            match value {
                // Print string without " around them
                Value::Str(ref x) => print!("{}\n", x),
                _ => println!("{}", value),
            };
            Ok(Value::Nil)
        }
    }

    pub fn builtin_inspect(&mut self, args: &[Value], env_ref: EnvRef) -> LispResult {
        if args.len() != 1 {
            Err(InvalidNumberOfArguments)
        } else {
            let value = self.eval(&args[0], env_ref)?;
            println!("{:?}", value);
            Ok(Value::Nil)
        }
    }

    pub fn builtin_eq(&mut self, args: &[Value], env_ref: EnvRef) -> LispResult {
        if args.len() != 2 {
            Err(InvalidNumberOfArguments)
        } else {
            Ok(Value::Bool(self.eval(&args[0], env_ref) == self.eval(&args[1], env_ref)))
        }
    }

    // pub fn builtin_ge(&mut self, args: &[Value]) -> LispResult {
    //     if args.len() != 2 {
    //         panic!("Usage: (>= <a> <b>)");
    //     } else {
    //         Value::Bool(self.eval(&args[0]) >= self.eval(&args[1]))
    //     }
    // }

    // pub fn builtin_le(&mut self, args: &[Value]) -> LispResult {
    //     if args.len() != 2 {
    //         panic!("Usage: (<= <a> <b>)");
    //     } else {
    //         Value::Bool(self.eval(&args[0]) <= self.eval(&args[1]))
    //     }
    // }

    // pub fn builtin_gt(&mut self, args: &[Value]) -> LispResult {
    //     if args.len() != 2 {
    //         panic!("Usage: (> <a> <b>)");
    //     } else {
    //         Value::Bool(self.eval(&args[0]) > self.eval(&args[1]))
    //     }
    // }

    // pub fn builtin_lt(&mut self, args: &[Value]) -> LispResult {
    //     if args.len() != 2 {
    //         panic!("Usage: (< <a> <b>)");
    //     } else {
    //         Value::Bool(self.eval(&args[0]) < self.eval(&args[1]))
    //     }
    // }

    pub fn builtin_if(&mut self, args: &[Value], env_ref: EnvRef) -> LispResult {
        if !(args.len() == 2 || args.len() == 3) {
            return Err(InvalidNumberOfArguments);
        }

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

    pub fn builtin_cond(&mut self, args: &[Value], env_ref: EnvRef) -> LispResult {
        if args.len() == 0 {
            return Err(InvalidNumberOfArguments);
        }

        for arg in args.iter() {
            match *arg {
                Value::List(ref elems) => {
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
                },
                _ => {
                    return Err(InvalidTypeOfArguments);
                }
            }
        }

        Ok(Value::Nil)
    }

    pub fn builtin_is_pair(&mut self, args: &[Value], env_ref: EnvRef) -> LispResult {
        if args.len() != 1 {
            Err(InvalidNumberOfArguments)
        } else {
            let value = self.eval(&args[0], env_ref)?;
            Ok(Value::Bool(value.is_pair()))
        }
    }

    pub fn builtin_is_list(&mut self, args: &[Value], env_ref: EnvRef) -> LispResult {
        if args.len() != 1 {
            Err(InvalidNumberOfArguments)
        } else {
            let value = self.eval(&args[0], env_ref)?;
            Ok(Value::Bool(value.is_list()))
        }
    }

    pub fn builtin_is_nil(&mut self, args: &[Value], env_ref: EnvRef) -> LispResult {
        if args.len() != 1 {
            Err(InvalidNumberOfArguments)
        } else {
            let value = self.eval(&args[0], env_ref)?;
            Ok(Value::Bool(value.is_nil()))
        }
    }

    pub fn builtin_first(&mut self, args: &[Value], env_ref: EnvRef) -> LispResult {
        if args.len() != 1 {
            Err(InvalidNumberOfArguments)
        } else {
            let value = self.eval(&args[0], env_ref)?;
            match value {
                // TODO: find some way to ensure dotted list size >= 2
                Value::DottedList(ref elems) => {
                    Ok(elems.first().unwrap().clone())
                },
                Value::List(ref elems) => {
                    Ok(elems.first().unwrap().clone())
                },
                _ => Err(InvalidTypeOfArguments)
            }
        }
    }

    pub fn builtin_rest(&mut self, args: &[Value], env_ref: EnvRef) -> LispResult {
        if args.len() != 1 {
            Err(InvalidNumberOfArguments)
        } else {
            let value = self.eval(&args[0], env_ref)?;
            match value {
                // TODO: find some way to ensure dotted list size >= 2
                Value::DottedList(ref elems) => {
                    if elems.len() == 2 {
                        Ok(elems.get(1).unwrap().clone())
                    } else {
                        let rest: Vec<Value> = elems[1..].iter().map(|v| v.clone()).collect();
                        Ok(Value::DottedList(rest))
                    }
                },
                Value::List(ref elems) => {
                    if elems.len() == 1 {
                        Ok(Value::Nil)
                    } else {
                        let rest: Vec<Value> = elems[1..].iter().map(|v| v.clone()).collect();
                        Ok(Value::List(rest))
                    }
                },
                _ => Err(InvalidTypeOfArguments)
            }
        }
    }

    pub fn builtin_begin(&mut self, args: &[Value], env_ref: EnvRef) -> LispResult {
        let mut result = Ok(Value::Nil);

        // TODO: Fail all if one threw an error?
        for a in args.iter() {
            result = self.eval(a, env_ref);
        }

        result
    }

    pub fn builtin_plus(&mut self, args: &[Value], env_ref: EnvRef) -> LispResult {
        let mut result = 0;

        for a in args.iter() {
            if let Value::Number(n) = self.eval(a, env_ref)? {
                result += n;
            } else {
                return Err(InvalidTypeOfArguments);
            }
        }

        Ok(Value::Number(result))
    }

    pub fn builtin_mult(&mut self, args: &[Value], env_ref: EnvRef) -> LispResult {
        let mut result = 1;

        for a in args.iter() {
            if let Value::Number(n) = self.eval(a, env_ref)? {
                result *= n;
            } else {
                return Err(InvalidTypeOfArguments);
            }
        }

        Ok(Value::Number(result))
    }

    pub fn builtin_minus(&mut self, args: &[Value], env_ref: EnvRef) -> LispResult {
        if args.len() == 0 {
            return Err(InvalidNumberOfArguments);
        }

        let mut result;

        // (- ...) is a bit weird,
        // (- 1) => -1
        // (- 1 2 3) = 1 - 2 - 3 = -4
        if let Value::Number(n) = self.eval(&args[0], env_ref)? {
            if args.len() == 1 {
                result = -n;
            } else {
                result = n; 
            }
        } else {
            return Err(InvalidTypeOfArguments);
        }

        for a in args.iter().skip(1) {
            if let Value::Number(n) = self.eval(a, env_ref)? {
                result -= n;
            } else {
                return Err(InvalidTypeOfArguments);
            }
        }

        Ok(Value::Number(result))
    }

    fn builtin_read(&mut self, args: &[Value], env_ref: EnvRef) -> LispResult {
        if args.len() != 1 {
            return Err(InvalidNumberOfArguments);
        }

        let value = self.eval(&args[0], env_ref)?;
        match value {
            Value::Str(ref input) => {
                let result = parser::parse_value(input);
                Ok(result)
            },
            _ => Err(InvalidTypeOfArguments)
        }
    }

    fn builtin_eval(&mut self, args: &[Value], env_ref: EnvRef) -> LispResult {
        if args.len() != 1 {
            return Err(InvalidNumberOfArguments);
        }

        let value = self.eval(&args[0], env_ref)?;
        self.eval(&value, env_ref)
    }

    fn builtin_load(&mut self, args: &[Value], env_ref: EnvRef) -> LispResult {
        if args.len() != 1 {
            return Err(InvalidNumberOfArguments);
        }

        let value = self.eval(&args[0], env_ref)?;

        match value {
            Value::Str(ref path) => self.eval_file(path, env_ref),
            _ => Err(InvalidTypeOfArguments),
        }
    }

    pub fn apply(&mut self, f: Value, args: &[Value], env_ref: EnvRef) -> LispResult {
        // println!("Applying {:?} to {:?}", args, f);
        match f {
            Value::Lambda(env, params, body) => {
                let mut e = Environment::new(Some(env_ref));
                if params.len() != args.len() {
                    return Err(InvalidNumberOfArguments);
                } else {
                    for (p, a) in params.iter().zip(args.iter()) {
                        let value = self.eval(&a, env_ref);
                        match value {
                            Ok(v) => e.define_into(p, v),
                            Err(err) => {
                                return Err(err)
                            },
                        };
                    }
                }

                let child_env_ref = self.envs.add_env(e);
                self.eval(&body, child_env_ref)
            },
            _ => Err(InvalidTypeOfArguments),
        }
    } 

    pub fn eval_file(&mut self, path: &str, env_ref: EnvRef) -> LispResult {
        let mut f = File::open(path).expect("Could not open file");
        let mut input = String::new();

        f.read_to_string(&mut input);

        self.eval_str(&input[..], env_ref)
    }

    pub fn eval_str(&mut self, input: &str, env_ref: EnvRef) -> LispResult {
        let result = parser::parse_program(input);
        let mut ret = Value::Nil;

        for v in result.iter() {
            let desugared = desugar::desugar(v);
            match self.eval(&desugared, env_ref) {
                Err(err) => {
                    return Err(err)
                },
                Ok(value) => {
                    ret = value;
                },
            }
        }

        Ok(ret)
    }

    pub fn eval(&mut self, v: &Value, env_ref: EnvRef) -> LispResult {
        // println!("Evaling {}", v);
        match *v {
            Value::List(ref elems) => {
                if elems.len() >= 1 {
                    match elems[0].clone() {
                        Value::Atom(s) => {
                            match s.as_ref() {
                                "def"  => self.builtin_def(&elems[1..], env_ref),
                                "set!"  => self.builtin_set(&elems[1..], env_ref),
                                "load"  => self.builtin_load(&elems[1..], env_ref),
                                "fn"  => self.builtin_lambda(&elems[1..], env_ref),
                                "if"   => self.builtin_if(&elems[1..], env_ref),
                                "cond"   => self.builtin_cond(&elems[1..], env_ref),
                                "do"   => self.builtin_begin(&elems[1..], env_ref),
                                "cons" => self.builtin_cons(&elems[1..], env_ref),
                                "fst" => self.builtin_first(&elems[1..], env_ref),
                                "rst" => self.builtin_rest(&elems[1..], env_ref),
                                "list"   => self.builtin_list(&elems[1..], env_ref),
                                "quote"   => self.builtin_quote(&elems[1..], env_ref),
                                "puts"   => self.builtin_puts(&elems[1..], env_ref),
                                "inspect"   => self.builtin_inspect(&elems[1..], env_ref),
                                "read"   => self.builtin_read(&elems[1..], env_ref),
                                "eval"   => self.builtin_eval(&elems[1..], env_ref),
                                "="    => self.builtin_eq(&elems[1..], env_ref),
                                // "<"    => self.builtin_lt(&elems[1..]),
                                "+"    => self.builtin_plus(&elems[1..], env_ref),
                                "*"    => self.builtin_mult(&elems[1..], env_ref),
                                "-"    => self.builtin_minus(&elems[1..], env_ref),
                                // ">"    => self.builtin_gt(&elems[1..]),
                                // "<="   => self.builtin_le(&elems[1..]),
                                // ">="   => self.builtin_ge(&elems[1..]),
                                "pair?"   => self.builtin_is_pair(&elems[1..], env_ref),
                                "list?"   => self.builtin_is_list(&elems[1..], env_ref),
                                "null?"   => self.builtin_is_nil(&elems[1..], env_ref),
                                "debug-env" => {
                                    println!("{:?}", self.envs.get_env(env_ref));
                                    Ok(Value::Nil)
                                },
                                other    => {
                                    // TODO: Find a way to do this with less duplication
                                    let v = self.envs.get(env_ref, &other.to_string()).clone();
                                    self.apply(v, &elems[1..], env_ref)
                                }
                            }
                        },
                        other => {
                            let v = self.eval(&other, env_ref)?;
                            self.apply(v, &elems[1..], env_ref)
                        },
                    }
                } else {
                    Err(InvalidNumberOfArguments)
                }
            },
            Value::Atom(ref v) => {
                let res = self.envs.get(env_ref, &v.to_string()).clone();
                Ok(res)
            },
            ref other => {
                Ok(other.clone())
            }
        }
    }
}
