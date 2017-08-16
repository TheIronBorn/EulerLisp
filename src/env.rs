use std::collections::HashMap;

use ::Value;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Environment {
    bindings: HashMap<String, Value>
}

impl Environment {
    pub fn new() -> Self {
        Environment {
          bindings: HashMap::new()
        }
    }

    pub fn define(&mut self, key: &String, value: &Value) {
        self.bindings.insert(key.clone(), value.clone());
    }

    pub fn define_rep(&mut self, key: &String, value: Value) {
        self.bindings.insert(key.clone(), value);
    }

    pub fn get(&self, key: &String) -> &Value {
        match self.bindings.get(key) {
            Some(value) => value,
            None => panic!(format!("Undefined variable '{}'", key))
        }
    }
}
