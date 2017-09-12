use std::collections::HashMap;

use ::Datum;

pub type EnvRef = usize;

// TODO: Use LispResults instead of panics

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Environment {
    pub bindings: HashMap<String, Datum>,
    pub parent: Option<EnvRef>
}

impl Environment {
    pub fn new(parent: Option<EnvRef>) -> Self {
        Environment { bindings: HashMap::new(), parent: parent }
    }
}

pub struct EnvArena {
    envs: Vec<Environment>
}

impl EnvArena {
    pub fn new() -> Self {
        Self { envs: Vec::new() }
    }

    pub fn make_env(&mut self, parent: Option<EnvRef>) -> EnvRef {
        let env_ref = self.envs.len();
        self.envs.push(Environment::new(parent));
        env_ref
    }

    pub fn add_env(&mut self, hm: HashMap<String, Datum>) -> EnvRef {
        let env_ref = self.envs.len();
        self.envs.push(Environment{ bindings: hm, parent: None });
        env_ref
    }

    pub fn get_env(&self, env_ref: EnvRef) -> &Environment {
        self.envs.get(env_ref).unwrap()
    }

    pub fn get(&self, env_ref: EnvRef, key: &String) -> &Datum {
        let e = self.envs.get(env_ref).unwrap();
        
        match e.bindings.get(key) {
            Some(v) => v,
            None => {
                match e.parent {
                    Some(r) => self.get(r, key),
                    None => panic!("Key not found {}", key)
                }
            }
        }
    }

    pub fn define_into(&mut self, env_ref: EnvRef, key: &String, value: Datum) -> bool {
        let mut e = self.envs.get_mut(env_ref).unwrap();

        if e.bindings.contains_key(key) {
            false
        } else {
            e.bindings.insert(key.clone(), value);
            true
        }
    }

    pub fn set_into(&mut self, env_ref: EnvRef, key: &String, value: Datum) -> bool {
        let mut cur = env_ref;

        loop {
            let mut e = self.envs.get_mut(cur).unwrap();
            match e.bindings.get_mut(key) {
                Some(v) => {
                    *v = value;
                    return true;
                },
                None => {
                    match e.parent {
                        Some(r) => { cur = r },
                        None => { return false; },
                    };
                }
            };
        }
    }
}
