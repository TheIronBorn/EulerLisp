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
            Value::Pair(Box::new(fst), Box::new(rst))
        }
    }

    pub fn builtin_eq(&mut self, args: &[Value]) -> Value {
        if args.len() != 2 {
            panic!("Usage: (= <a> <b>)");
        } else {
            Value::Bool(self.eval(&args[0]) == self.eval(&args[1]))
        }
    }

    pub fn builtin_ge(&mut self, args: &[Value]) -> Value {
        if args.len() != 2 {
            panic!("Usage: (>= <a> <b>)");
        } else {
            Value::Bool(self.eval(&args[0]) >= self.eval(&args[1]))
        }
    }

    pub fn builtin_le(&mut self, args: &[Value]) -> Value {
        if args.len() != 2 {
            panic!("Usage: (<= <a> <b>)");
        } else {
            Value::Bool(self.eval(&args[0]) <= self.eval(&args[1]))
        }
    }

    pub fn builtin_gt(&mut self, args: &[Value]) -> Value {
        if args.len() != 2 {
            panic!("Usage: (> <a> <b>)");
        } else {
            Value::Bool(self.eval(&args[0]) > self.eval(&args[1]))
        }
    }

    pub fn builtin_lt(&mut self, args: &[Value]) -> Value {
        if args.len() != 2 {
            panic!("Usage: (< <a> <b>)");
        } else {
            Value::Bool(self.eval(&args[0]) < self.eval(&args[1]))
        }
    }

    pub fn builtin_if(&mut self, args: &[Value]) -> Value {
        if args.len() != 3 {
            panic!("Usage: (if <cond> <then> <else>)");
        } else {
            let cond = self.eval(&args[0]);

            match cond {
              Value::Bool(true) => self.eval(&args[1]),
              Value::Bool(false) => self.eval(&args[2]),
              _ => panic!("if condition must eval to a boolean")
            }
        }
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

    pub fn eval(&mut self, v: &Value) -> Value {
        match *v {
            Value::List(ref elems) => {
                if elems.len() >= 1 {
                    // let args: Vec<Value> = elems[1..].iter().map(|e| self.eval(e)).collect();
                    match elems[0].clone() {
                        Value::Atom(s) => {
                            match s.as_ref() {
                                "def"  => self.builtin_def(&elems[1..]),
                                "if"   => self.builtin_if(&elems[1..]),
                                "cons" => self.builtin_cons(&elems[1..]),
                                "="    => self.builtin_eq(&elems[1..]),
                                "<"    => self.builtin_lt(&elems[1..]),
                                ">"    => self.builtin_gt(&elems[1..]),
                                "<="   => self.builtin_le(&elems[1..]),
                                ">="   => self.builtin_ge(&elems[1..]),
                                "pair?"   => self.builtin_is_pair(&elems[1..]),
                                "list?"   => self.builtin_is_list(&elems[1..]),
                                "nil?"   => self.builtin_is_nil(&elems[1..]),
                                _      => panic!("Unknown command"),
                            }
                        },
                        _ => panic!("Command must be atom"),
                    }
                } else {
                    panic!("Empty calls are not allowed")
                }
            },
            // Value::Atom(ref v) => {
            //     self.env.get(v).clone()
            // }
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
