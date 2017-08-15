use ::Value;

use env::Environment;

pub struct Evaluator {
    env: Environment
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator { env: Environment::new() }
    }

    pub fn eval(&mut self, v: &Value) -> Value {
        match *v {
            Value::List(ref elems) => {
                if elems.len() >= 1 {
                    match elems[0].clone() {
                        Value::Atom(s) => {
                            match s.as_ref() {
                                "def" => {
                                    if elems.len() != 3 {
                                        panic!("Usage: (def key value)");
                                    } else {
                                        match elems[1].clone() {
                                            Value::Atom(a) => {
                                                self.env.define(&a, &elems[2].clone());
                                            },
                                            _ => panic!("def key must be atom"),
                                        };

                                        Value::Atom("ok".to_string())
                                    }
                                },
                                _ => panic!("Unknown command"),
                            }
                        },
                        _ => panic!("Command must be atom"),
                        // let mut values = Vec::new();
                        // let mut env_ = env.clone();
                        // for e in elems.iter() {
                        //   let res = eval(e, env_);
                        //   values.push(res.0);
                        //   env_ = res.1;
                        // }

                        // let res = apply_function(&values[0], &values[1..]);
                        // (res, env_)
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
