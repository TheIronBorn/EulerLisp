use std::collections::HashMap;

use ::Value;
use ::LispResult;
use ::LispErr::*;

pub struct Builtin {
    methods: HashMap<(&'static str, usize), Box<Fn(Vec<Value>)->LispResult>>
}

impl Builtin {
    pub fn new() -> Self {
        let mut res = Self { methods: HashMap::new() };
        res.methods.insert(
            ("pair_?", 1), Box::new(|vs| {
                Ok(Value::Bool(vs[0].is_pair()))
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
