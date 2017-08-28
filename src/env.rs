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

    pub fn define(&mut self, key: &String, value: &Value) -> bool {
        self.define_into(key, value.clone())
    }

    pub fn define_into(&mut self, key: &String, value: Value) -> bool {
        if self.bindings.contains_key(key) {
            false
        } else {
            self.bindings.insert(key.clone(), value);
            true
        }
    }

    pub fn set(&mut self, key: &String, value: &Value) {
        self.set_into(key, value.clone());
    }

    pub fn set_into(&mut self, key: &String, value: Value) {
        self.bindings.insert(key.clone(), value);
    }

    pub fn get(&self, key: &String) -> &Value {
        // println!("Looking up {:?} in {:?}", key, &self);
        match self.bindings.get(key) {
            Some(value) => value,
            None => panic!(format!("Undefined variable '{}'", key))
        }
    }
}
