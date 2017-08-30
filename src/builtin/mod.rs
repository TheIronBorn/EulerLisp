use std::collections::HashMap;

use ::Value;
use ::LispResult;
use ::LispErr::*;

// The difference between builtins and special forms is,
// that special forms choose if they want to eval their arguments themselves,
// builtins are called with evaluated arguments

pub struct Builtin {
    methods: HashMap<(&'static str, usize), Box<Fn(Vec<Value>)->LispResult>>
}

impl Builtin {
    pub fn new() -> Self {
        let mut res = Self { methods: HashMap::new() };
        res.methods.insert(
            ("pair?", 1), Box::new(|vs| Ok(Value::Bool(vs[0].is_pair())))
        );
        res.methods.insert(
            ("list?", 1), Box::new(|vs| Ok(Value::Bool(vs[0].is_list())))
        );
        res.methods.insert(
            ("nil?", 1), Box::new(|vs| Ok(Value::Bool(vs[0].is_nil())))
        );
        res.methods.insert(
            ("=", 2), Box::new(|vs| Ok(Value::Bool(vs[0] == vs[1])))
        );
        // TODO: What should happen when compairing two different types?
        res.methods.insert(
            (">", 2), Box::new(|vs| Ok(Value::Bool(vs[0] > vs[1])))
        );
        res.methods.insert(
            ("<", 2), Box::new(|vs| Ok(Value::Bool(vs[0] < vs[1])))
        );
        res.methods.insert(
            (">=", 2), Box::new(|vs| Ok(Value::Bool(vs[0] >= vs[1])))
        );
        res.methods.insert(
            ("<=", 2), Box::new(|vs| Ok(Value::Bool(vs[0] <= vs[1])))
        );
        res.methods.insert(
            ("puts", 1), Box::new(|vs| {
                match vs[0] {
                    // Print string without " around them
                    Value::Str(ref x) => print!("{}\n", x),
                    ref other => println!("{}", other),
                };
                Ok(Value::Nil)
            })
        );
        res.methods.insert(
            ("inspect", 1), Box::new(|vs| {
                println!("{:?}", vs[0]);
                Ok(Value::Nil)
            })
        );
        res.methods.insert(
            ("+", 2), Box::new(|vs| {
                if let Value::Number(a) = vs[0] {
                    if let Value::Number(b) = vs[1] {
                        return Ok(Value::Number(a + b));
                    }
                }
                Err(InvalidTypeOfArguments)
            })
        );
        res.methods.insert(
            ("*", 2), Box::new(|vs| {
                if let Value::Number(a) = vs[0] {
                    if let Value::Number(b) = vs[1] {
                        return Ok(Value::Number(a * b));
                    }
                }
                Err(InvalidTypeOfArguments)
            })
        );
        res.methods.insert(
            ("/", 2), Box::new(|vs| {
                if let Value::Number(a) = vs[0] {
                    if let Value::Number(b) = vs[1] {
                        return Ok(Value::Number(a / b));
                    }
                }
                Err(InvalidTypeOfArguments)
            })
        );
        res.methods.insert(
            ("%", 2), Box::new(|vs| {
                if let Value::Number(a) = vs[0] {
                    if let Value::Number(b) = vs[1] {
                        return Ok(Value::Number(a % b));
                    }
                }
                Err(InvalidTypeOfArguments)
            })
        );
        res.methods.insert(
            ("-", 2), Box::new(|vs| {
                if let Value::Number(a) = vs[0] {
                    if let Value::Number(b) = vs[1] {
                        return Ok(Value::Number(a * b));
                    }
                }
                Err(InvalidTypeOfArguments)
            })
        );
        res.methods.insert(
            ("-", 1), Box::new(|vs| {
                if let Value::Number(a) = vs[0] {
                    return Ok(Value::Number(-a));
                }
                Err(InvalidTypeOfArguments)
            })
        );
        res.methods.insert(
            ("not", 1), Box::new(|vs| {
                if let Value::Bool(b) = vs[0] {
                    return Ok(Value::Bool(!b));
                }
                Err(InvalidTypeOfArguments)
            })
        );

        res.methods.insert(
            ("cons", 2), Box::new(|vs| {
                // TODO: Can this be done without clone?
                let fst = vs[0].clone();
                let rst = vs[1].clone();

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
            })
        );
        res.methods.insert(
            ("fst", 1), Box::new(|vs| {
                match vs[0] {
                    // TODO: find some way to ensure dotted list size >= 2
                    Value::DottedList(ref elems) => {
                        Ok(elems.first().unwrap().clone())
                    },
                    Value::List(ref elems) => {
                        Ok(elems.first().unwrap().clone())
                    },
                    _ => Err(InvalidTypeOfArguments)
                }
            })
        );
        res.methods.insert(
            ("rst", 1), Box::new(|vs| {
                match vs[0] {
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
            })
        );



        res
    }

    pub fn is_builtin(&self, name: &str, arity: usize) -> bool {
        self.methods.contains_key(&(name, arity))
    }

    pub fn apply(&self, name: &str, args: Vec<Value>) -> LispResult {
        let arity = args.len();
        match self.methods.get(&(name, arity)) {
            Some(f) => f(args),
            None => Err(DefinitionNotFound),
        }
    }
}
