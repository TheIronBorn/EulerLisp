use ::Value;

use env::Environment;

pub struct Evaluator { env: Environment }

impl Evaluator {
    pub fn new() -> Self {
        Evaluator { env: Environment::new() }
    }

    pub fn builtin_def(&mut self, args: &[Value]) -> Value {
        if args.len() != 2 {
            panic!("Usage: (def <key> <value>)");
        } else {
            match args[0] {
                Value::Atom(ref a) => {
                    let value = self.eval(&args[1]);
                    self.env.define_rep(a, value)
                },
                _ => panic!("def key must be atom"),
            };
            Value::Atom("ok".to_string())
        }
    }

    pub fn builtin_cons(&mut self, args: &[Value]) -> Value {
        if args.len() != 2 {
            panic!("Usage: (cons <fst> <rst>)");
        } else {
            let fst = self.eval(&args[0]);
            let rst = self.eval(&args[1]);

            match rst {
                Value::Nil => Value::List(vec![fst]),
                Value::DottedList(ref elems) => {
                    let mut new = elems.clone();
                    new.insert(0, fst);
                    Value::DottedList(new)
                }
                Value::List(ref elems) => {
                    let mut new = elems.clone();
                    new.insert(0, fst);
                    Value::List(new)
                }
                other => Value::DottedList(vec![fst, other])
            }
        }
    }

    pub fn builtin_quote(&mut self, args: &[Value]) -> Value {
        if args.len() != 1 {
            panic!("Usage: (quote <value>)");
        } else {
            match args[0] {
                // Value::List(ref l) => self.builtin_list(&l[..]),
                ref other => other.clone()
            }
        }
    }

    pub fn builtin_list(&mut self, args: &[Value]) -> Value {
        if args.len() == 0 {
            Value::Nil
        } else {
            let vals: Vec<Value> = args.iter().map(|v| self.eval(v)).collect();
            Value::List(vals)
        }
    }

    pub fn builtin_lambda(&mut self, args: &[Value]) -> Value {
        if args.len() != 2 {
            panic!("Usage: (fn (args, ...) <body>)");
        } else {
            let mut params: Vec<String> = Vec::new();

            match args[0] {
                Value::List(ref elems) => {
                    for a in elems {
                        match *a {
                            Value::Atom(ref v) => params.push(v.clone()),
                            _ => panic!("Lambda arguments must be strings"),
                        }
                    }
                },
                _ => panic!("Usage: (fn (args, ...) <body>)"),
            }

            let body = args[1].clone();
            Value::Lambda(self.env.clone(), params, Box::new(body))
        }
    }

    pub fn builtin_puts(&mut self, args: &[Value]) -> Value {
        if args.len() != 1 {
            panic!("Usage: (puts <value>)");
        } else {
            let value = self.eval(&args[0]);
            match value {
                // Print string without " around them
                Value::Str(ref x) => print!("{}\n", x),
                _ => println!("{}", value),
            };
            Value::Nil
        }
    }

    pub fn builtin_eq(&mut self, args: &[Value]) -> Value {
        if args.len() != 2 {
            panic!("Usage: (= <a> <b>)");
        } else {
            Value::Bool(self.eval(&args[0]) == self.eval(&args[1]))
        }
    }

    // pub fn builtin_ge(&mut self, args: &[Value]) -> Value {
    //     if args.len() != 2 {
    //         panic!("Usage: (>= <a> <b>)");
    //     } else {
    //         Value::Bool(self.eval(&args[0]) >= self.eval(&args[1]))
    //     }
    // }

    // pub fn builtin_le(&mut self, args: &[Value]) -> Value {
    //     if args.len() != 2 {
    //         panic!("Usage: (<= <a> <b>)");
    //     } else {
    //         Value::Bool(self.eval(&args[0]) <= self.eval(&args[1]))
    //     }
    // }

    // pub fn builtin_gt(&mut self, args: &[Value]) -> Value {
    //     if args.len() != 2 {
    //         panic!("Usage: (> <a> <b>)");
    //     } else {
    //         Value::Bool(self.eval(&args[0]) > self.eval(&args[1]))
    //     }
    // }

    // pub fn builtin_lt(&mut self, args: &[Value]) -> Value {
    //     if args.len() != 2 {
    //         panic!("Usage: (< <a> <b>)");
    //     } else {
    //         Value::Bool(self.eval(&args[0]) < self.eval(&args[1]))
    //     }
    // }

    pub fn builtin_if(&mut self, args: &[Value]) -> Value {
        let usage = "Usage: (if <condition> <consequent> [<alternative>])";
        let cond = args.get(0).expect(usage);
        let cons = args.get(1).expect(usage);
        let default_alt = Value::Nil;
        let alt = args.get(2).unwrap_or(&default_alt);

        match self.eval(&cond) {
            Value::Bool(true) => self.eval(cons),
            Value::Bool(false) => self.eval(alt),
            _ => panic!("if condition must eval to a boolean")
        }
    }

    pub fn builtin_cond(&mut self, args: &[Value]) -> Value {
        let usage = "Usage: (cond (<condition> <consequent>)... [(else <alternative>)])";
        if args.len() == 0 {
            panic!(usage);
        }

        for arg in args.iter() {
            match *arg {
                Value::List(ref elems) => {
                    let cond = elems.get(0).expect(usage);
                    let cons = elems.get(1).expect(usage);

                    // TODO this does not check if "else" comes last
                    if *cond == Value::Atom("else".to_string()) {
                        return self.eval(cons);
                    } else {
                        let res = self.eval(cond);
                        if res == Value::Bool(true) {
                            return self.eval(cons);
                        } else {
                            continue
                        }
                    }
                },
                _ => panic!(usage)
            }
        }

        Value::Nil
    }

    pub fn builtin_is_pair(&mut self, args: &[Value]) -> Value {
        if args.len() != 1 {
            panic!("Usage: (is_pair <value>)");
        } else {
            let value = self.eval(&args[0]);
            Value::Bool(value.is_pair())
        }
    }

    pub fn builtin_is_list(&mut self, args: &[Value]) -> Value {
        if args.len() != 1 {
            panic!("Usage: (is_pair <value>)");
        } else {
            let value = self.eval(&args[0]);
            Value::Bool(value.is_list())
        }
    }

    pub fn builtin_is_nil(&mut self, args: &[Value]) -> Value {
        if args.len() != 1 {
            panic!("Usage: (is_pair <value>)");
        } else {
            let value = self.eval(&args[0]);
            Value::Bool(value.is_nil())
        }
    }

    pub fn builtin_first(&mut self, args: &[Value]) -> Value {
        if args.len() != 1 {
            panic!("Usage: (fst <value>)");
        } else {
            let value = self.eval(&args[0]);
            match value {
                // TODO: find some way to ensure dotted list size >= 2
                Value::DottedList(ref elems) => {
                    elems.first().unwrap().clone()
                },
                Value::List(ref elems) => {
                    elems.first().unwrap().clone()
                },
                _ => panic!("Can't take first of non-list value")
            }
        }
    }

    pub fn builtin_rest(&mut self, args: &[Value]) -> Value {
        if args.len() != 1 {
            panic!("Usage: (rst <value>)");
        } else {
            let value = self.eval(&args[0]);
            match value {
                // TODO: find some way to ensure dotted list size >= 2
                Value::DottedList(ref elems) => {
                    if elems.len() == 2 {
                        elems.get(1).unwrap().clone()
                    } else {
                        let rest: Vec<Value> = elems[1..].iter().map(|v| v.clone()).collect();
                        Value::DottedList(rest)
                    }
                },
                Value::List(ref elems) => {
                    if elems.len() == 1 {
                        Value::Nil
                    } else {
                        let rest: Vec<Value> = elems[1..].iter().map(|v| v.clone()).collect();
                        Value::List(rest)
                    }
                },
                _ => panic!("Can't take first of non-list value")
            }
        }
    }

    pub fn builtin_do(&mut self, args: &[Value]) -> Value {
        let mut result = Value::Nil;

        for a in args.iter() {
            result = self.eval(a);
        }

        result
    }

    pub fn apply(&mut self, f: Value, args: &[Value]) -> Value {
        match f {
            Value::Lambda(env, params, body) => {
                let mut e = env.clone();
                if params.len() != args.len() {
                    panic!("Not enough / too many arguments");
                } else {
                    for (p, a) in params.iter().zip(args.iter()) {
                        let value = self.eval(&a);
                        e.define(p, a);
                    }
                }

                let mut ev = Evaluator { env: e };
                ev.eval(&body)
            },
            _ => panic!("Not a lambda"),
        }
    } 

    pub fn eval(&mut self, v: &Value) -> Value {
        println!("Evaling {}", v);
        match *v {
            Value::List(ref elems) => {
                if elems.len() >= 1 {
                    // let args: Vec<Value> = elems[1..].iter().map(|e| self.eval(e)).collect();
                    match elems[0].clone() {
                        Value::Atom(s) => {
                            match s.as_ref() {
                                "def"  => self.builtin_def(&elems[1..]),
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
                                "="    => self.builtin_eq(&elems[1..]),
                                // "<"    => self.builtin_lt(&elems[1..]),
                                // ">"    => self.builtin_gt(&elems[1..]),
                                // "<="   => self.builtin_le(&elems[1..]),
                                // ">="   => self.builtin_ge(&elems[1..]),
                                "pair?"   => self.builtin_is_pair(&elems[1..]),
                                "list?"   => self.builtin_is_list(&elems[1..]),
                                "null?"   => self.builtin_is_nil(&elems[1..]),
                                other    => {
                                    let v = self.env.get(&other.to_string()).clone();
                                    self.apply(v, &elems[1..])
                                },
                            }
                        },
                        other => {
                            let v = self.eval(&other);
                            self.apply(v, &elems[1..])
                        },
                    }
                } else {
                    panic!("Empty calls are not allowed")
                }
            },
            Value::Atom(ref v) => {
                self.env.get(v).clone()
            }
            ref other => {
                other.clone()
            }
        }
    }
}

fn apply_function(v: &Value, args: &[Value]) -> Value {
    match *v {
        Value::Atom(ref x) => match &x[..] {
            "add" => primitive_add(args),
            "mul" => primitive_mul(args),
            ref other => panic!("Unknown function {:?}", other)
        },
        ref other => panic!("Unknown function {:?}", *other)
    }
}

fn primitive_add(args: &[Value]) -> Value {
    let mut sum = 0;
    for i in args {
        match i {
            &Value::Number(n) => {
                sum += n;
            },
            other => panic!("Invalid argument for `add`: {:?}", other)
        }
    }
    Value::Number(sum)
}

fn primitive_mul(args: &[Value]) -> Value {
    let mut sum = 1;
    for i in args {
        match i {
            &Value::Number(n) => {
                sum *= n;
            },
            other => panic!("Invalid argument for `add`: {:?}", other)
        }
    }
    Value::Number(sum)
}
