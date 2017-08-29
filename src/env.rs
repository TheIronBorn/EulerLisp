use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use ::Value;

pub struct EnvArena {
    envs: Vec<Environment>
}

impl EnvArena {
    pub fn new() -> Self {
        Self {
            envs: Vec::new()
        }
    }

    pub fn make_env(&mut self, parent: Option<EnvRef>) -> EnvRef {
        let env_ref = self.envs.len();
        self.envs.push(Environment::new(parent));
        env_ref
    }

    pub fn add_env(&mut self, env: Environment) -> EnvRef {
        let env_ref = self.envs.len();
        self.envs.push(env);
        env_ref
    }

    pub fn get(&self, env_ref: EnvRef, key: &String) -> &Value {
        let e = self.envs.get(env_ref).unwrap();
        
        match e.get(key) {
            Some(v) => v,
            None => {
                match e.parent {
                    Some(r) => self.get(r, key),
                    None => panic!("Key not found {}", key)
                }
            }
        }
    }

    pub fn define_into(&mut self, env_ref: EnvRef, key: &String, value: Value) -> bool {
        let mut e = self.envs.get_mut(env_ref).unwrap();
        (*e).define_into(key, value)
    }

    pub fn set_into(&mut self, env_ref: EnvRef, key: &String, value: Value) -> bool {
        let mut cur = env_ref;

        loop {
            let mut e = self.envs.get_mut(env_ref).unwrap();
            match e.bindings.get_mut(key) {
                Some(v) => {
                    *v = value;
                    return true;
                },
                None => {
                    match e.parent {
                        Some(r) => { cur = r },
                        None => { return false; },
                    }
                }
            }
        }
    }
}

pub type EnvRef = usize;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Environment {
    pub bindings: HashMap<String, Value>,
    pub parent: Option<EnvRef>
}

impl Environment {
    pub fn new(parent: Option<EnvRef>) -> Self {
        Environment {
          bindings: HashMap::new(),
          parent: parent
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

    pub fn get(&self, key: &String) -> Option<&Value> {
        self.bindings.get(key)
    }
}
