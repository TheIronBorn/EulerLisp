use std::collections::HashMap;

use ::Value;

#[derive(Clone)]
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

    pub fn get(&self, key: &String) -> &Value {
        match self.bindings.get(key) {
            Some(value) => value,
            None => panic!("Could not find key!")
        }
    }
}
