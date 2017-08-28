use ::Value;
use ::LispResult;
use ::LispErr::*;

use env::Environment;
use parser;
use desugar;

pub struct Evaluator { env: Environment }

impl Evaluator {
    pub fn new() -> Self {
        Evaluator { env: Environment::new() }
    }

    // If the key is already set in the current env,
    // throw an error,
    // otherwise define it
    pub fn builtin_def(&mut self, args: &[Value]) -> LispResult {
        if args.len() != 2 {
            Err(InvalidNumberOfArguments)
        } else {
            match args[0] {
                Value::Atom(ref a) => {
                    let value = self.eval(&args[1])?;
                    if self.env.define_into(a, value) {
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
    pub fn builtin_set(&mut self, args: &[Value]) -> LispResult {
        if args.len() != 2 {
            Err(InvalidNumberOfArguments)
        } else {
            match args[0] {
                Value::Atom(ref a) => {
                    let value = self.eval(&args[1])?;
                    self.env.set_into(a, value);
                    // TODO: Handle errors
                    Ok(Value::Atom("ok".to_string()))
                },
                _ => Err(InvalidTypeOfArguments),
            }
        }
    }

    pub fn builtin_cons(&mut self, args: &[Value]) -> LispResult {
        if args.len() != 2 {
            Err(InvalidNumberOfArguments)
        } else {
            let fst = self.eval(&args[0])?;
            let rst = self.eval(&args[1])?;

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

    pub fn builtin_quote(&mut self, args: &[Value]) -> LispResult {
        if args.len() != 1 {
            Err(InvalidNumberOfArguments)
        } else {
            match args[0] {
                // Value::List(ref l) => self.builtin_list(&l[..]),
                ref other => Ok(other.clone())
            }
        }
    }

    pub fn builtin_list(&mut self, args: &[Value]) -> LispResult {
        if args.len() == 0 {
            Ok(Value::Nil)
        } else {
            let mut vals: Vec<Value> = Vec::new();

            for a in args.iter().skip(1) {
                match self.eval(a) {
                    Ok(v) => vals.push(v),
                    Err(msg) => return Err(msg),
                }
            }

            Ok(Value::List(vals))
        }
    }

    pub fn builtin_lambda(&mut self, args: &[Value]) -> LispResult {
        if args.len() != 2 {
            Err(InvalidNumberOfArguments)
        } else {
            let mut params: Vec<String> = Vec::new();

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

            let body = args[1].clone();
            Ok(Value::Lambda(self.env.clone(), params, Box::new(body)))
        }
    }

    pub fn builtin_puts(&mut self, args: &[Value]) -> LispResult {
        if args.len() != 1 {
            Err(InvalidNumberOfArguments)
        } else {
            let value = self.eval(&args[0])?;
            match value {
                // Print string without " around them
                Value::Str(ref x) => print!("{}\n", x),
                _ => println!("{}", value),
            };
            Ok(Value::Nil)
        }
    }

    pub fn builtin_eq(&mut self, args: &[Value]) -> LispResult {
        if args.len() != 2 {
            Err(InvalidNumberOfArguments)
        } else {
            Ok(Value::Bool(self.eval(&args[0]) == self.eval(&args[1])))
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

    pub fn builtin_if(&mut self, args: &[Value]) -> LispResult {
        if !(args.len() == 2 || args.len() == 3) {
            return Err(InvalidNumberOfArguments);
        }

        let cond = args.get(0).unwrap();
        let cons = args.get(1).unwrap();

        let default_alt = Value::Nil;
        let alt = args.get(2).unwrap_or(&default_alt);

        match self.eval(&cond)? {
            Value::Bool(true) => self.eval(cons),
            Value::Bool(false) => self.eval(alt),
            _ => Err(InvalidTypeOfArguments)
        }
    }

    pub fn builtin_cond(&mut self, args: &[Value]) -> LispResult {
        let usage_err = Err(InvalidNumberOfArguments);
        if args.len() == 0 {
            return usage_err;
        }

        for arg in args.iter() {
            match *arg {
                Value::List(ref elems) => {
                    if elems.len() != 2 {
                        return usage_err;
                    }
                    let cond = elems.get(0).unwrap();
                    let cons = elems.get(1).unwrap();

                    // TODO this does not check if "else" comes last
                    if *cond == Value::Atom("else".to_string()) {
                        return self.eval(cons);
                    } else {
                        let res = self.eval(cond)?;
                        if res == Value::Bool(true) {
                            return self.eval(cons);
                        } else {
                            continue
                        }
                    }
                },
                _ => {
                    return usage_err;
                }
            }
        }

        Ok(Value::Nil)
    }

    pub fn builtin_is_pair(&mut self, args: &[Value]) -> LispResult {
        if args.len() != 1 {
            Err(InvalidNumberOfArguments)
        } else {
            let value = self.eval(&args[0])?;
            Ok(Value::Bool(value.is_pair()))
        }
    }

    pub fn builtin_is_list(&mut self, args: &[Value]) -> LispResult {
        if args.len() != 1 {
            Err(InvalidNumberOfArguments)
        } else {
            let value = self.eval(&args[0])?;
            Ok(Value::Bool(value.is_list()))
        }
    }

    pub fn builtin_is_nil(&mut self, args: &[Value]) -> LispResult {
        if args.len() != 1 {
            Err(InvalidNumberOfArguments)
        } else {
            let value = self.eval(&args[0])?;
            Ok(Value::Bool(value.is_nil()))
        }
    }

    pub fn builtin_first(&mut self, args: &[Value]) -> LispResult {
        if args.len() != 1 {
            Err(InvalidNumberOfArguments)
        } else {
            let value = self.eval(&args[0])?;
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

    pub fn builtin_rest(&mut self, args: &[Value]) -> LispResult {
        if args.len() != 1 {
            Err(InvalidNumberOfArguments)
        } else {
            let value = self.eval(&args[0])?;
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

    pub fn builtin_do(&mut self, args: &[Value]) -> LispResult {
        let mut result = Ok(Value::Nil);

        // TODO: Fail all if one threw an error?
        for a in args.iter() {
            result = self.eval(a);
        }

        result
    }

    pub fn builtin_plus(&mut self, args: &[Value]) -> LispResult {
        let mut result = 0;

        for a in args.iter() {
            if let Value::Number(n) = self.eval(a)? {
                result += n;
            } else {
                return Err(InvalidTypeOfArguments);
            }
        }

        Ok(Value::Number(result))
    }

    pub fn builtin_mult(&mut self, args: &[Value]) -> LispResult {
        let mut result = 1;

        for a in args.iter() {
            if let Value::Number(n) = self.eval(a)? {
                result *= n;
            } else {
                return Err(InvalidTypeOfArguments);
            }
        }

        Ok(Value::Number(result))
    }

    pub fn builtin_minus(&mut self, args: &[Value]) -> LispResult {
        let mut result = 0;

        if args.len() == 0 {
            return Err(InvalidNumberOfArguments);
        }

        // (- ...) is a bit weird,
        // (- 1) => -1
        // (- 1 2 3) = 1 - 2 - 3 = -4
        if let Value::Number(n) = self.eval(&args[0])? {
            if args.len() == 1 {
                result = -n;
            } else {
                result = n; 
            }
        } else {
            return Err(InvalidTypeOfArguments);
        }

        if args.len() == 1 {
        }

        for a in args[1..].iter() {
            if let Value::Number(n) = self.eval(a)? {
                result -= n;
            } else {
                return Err(InvalidTypeOfArguments);
            }
        }

        Ok(Value::Number(result))
    }

    fn builtin_read(&mut self, args: &[Value]) -> LispResult {
        if args.len() != 1 {
            return Err(InvalidNumberOfArguments);
        }

        let value = self.eval(&args[0])?;
        match value {
            Value::Str(ref input) => {
                let result = parser::parse(input);
                Ok(result)
            },
            _ => Err(InvalidTypeOfArguments)
        }
    }

    fn builtin_eval(&mut self, args: &[Value]) -> LispResult {
        if args.len() != 1 {
            return Err(InvalidNumberOfArguments);
        }

        let value = self.eval(&args[0])?;
        self.eval(&value)
    }

    pub fn apply(&mut self, f: Value, args: &[Value]) -> LispResult {
        // println!("Applying {:?} to {:?}", args, f);
        match f {
            Value::Lambda(env, params, body) => {
                let mut e = env.clone();
                if params.len() != args.len() {
                    return Err(InvalidNumberOfArguments);
                } else {
                    for (p, a) in params.iter().zip(args.iter()) {
                        let value = self.eval(&a);
                        e.define(p, a);
                    }
                }

                let mut ev = Evaluator { env: e };
                ev.eval(&body)
            },
            _ => Err(InvalidTypeOfArguments),
        }
    } 

    pub fn eval_str(&mut self, input: &str) -> LispResult {
        let mut result = parser::parse(input);
        let desugared = desugar::desugar(&result);
        self.eval(&desugared)
    }

    pub fn eval(&mut self, v: &Value) -> LispResult {
        // println!("Evaling {}", v);
        match *v {
            Value::List(ref elems) => {
                if elems.len() >= 1 {
                    match elems[0].clone() {
                        Value::Atom(s) => {
                            match s.as_ref() {
                                "def"  => self.builtin_def(&elems[1..]),
                                "set!"  => self.builtin_set(&elems[1..]),
                                "fn"  => self.builtin_lambda(&elems[1..]),
                                "if"   => self.builtin_if(&elems[1..]),
                                "cond"   => self.builtin_cond(&elems[1..]),
                                "do"   => self.builtin_do(&elems[1..]),
                                "cons" => self.builtin_cons(&elems[1..]),
                                "fst" => self.builtin_first(&elems[1..]),
                                "rst" => self.builtin_rest(&elems[1..]),
                                "list"   => self.builtin_list(&elems[1..]),
                                "quote"   => self.builtin_quote(&elems[1..]),
                                "puts"   => self.builtin_puts(&elems[1..]),
                                "read"   => self.builtin_read(&elems[1..]),
                                "eval"   => self.builtin_eval(&elems[1..]),
                                "="    => self.builtin_eq(&elems[1..]),
                                // "<"    => self.builtin_lt(&elems[1..]),
                                "+"    => self.builtin_plus(&elems[1..]),
                                "*"    => self.builtin_mult(&elems[1..]),
                                "-"    => self.builtin_minus(&elems[1..]),
                                // ">"    => self.builtin_gt(&elems[1..]),
                                // "<="   => self.builtin_le(&elems[1..]),
                                // ">="   => self.builtin_ge(&elems[1..]),
                                "pair?"   => self.builtin_is_pair(&elems[1..]),
                                "list?"   => self.builtin_is_list(&elems[1..]),
                                "null?"   => self.builtin_is_nil(&elems[1..]),
                                other    => {
                                    let v = self.env.get(&other.to_string()).clone();

                                    let mut vals: Vec<Value> = Vec::new();

                                    for a in elems.iter().skip(1) {
                                        match self.eval(a) {
                                            Ok(v) => vals.push(v),
                                            Err(msg) => return Err(msg),
                                        }
                                    }
                                    self.apply(v, &vals[..])
                                }
                            }
                        },
                        other => {
                            let v = self.eval(&other)?;

                            let mut vals: Vec<Value> = Vec::new();

                            for a in elems.iter().skip(1) {
                                match self.eval(a) {
                                    Ok(v) => vals.push(v),
                                    Err(msg) => return Err(msg),
                                }
                            }
                            self.apply(v, &vals[..])
                        },
                    }
                } else {
                    Err(InvalidNumberOfArguments)
                }
            },
            Value::Atom(ref v) => {
                Ok(self.env.get(v).clone())
            }
            ref other => {
                Ok(other.clone())
            }
        }
    }
}
