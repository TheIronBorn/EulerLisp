use std::collections::HashMap;

use ::Datum;
use ::Symbol;
use symbol_table::SymbolTable;

pub type EnvRef = usize;

// TODO: Use LispResults instead of panics

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Environment {
    pub bindings: HashMap<usize, Datum>,
    pub parent: Option<EnvRef>
}

impl Environment {
    pub fn new(parent: Option<EnvRef>) -> Self {
        Environment { bindings: HashMap::new(), parent: parent }
    }

    pub fn free(&mut self) {
        self.bindings = HashMap::new();
    }
}

pub struct EnvArena {
    envs: Vec<Environment>,
}

impl EnvArena {
    pub fn new() -> Self {
        Self { envs: Vec::new() }
    }

    pub fn size(&self) -> usize {
        self.envs.len()
    }

    pub fn free(&mut self, env: EnvRef) {
        if env != 0 && !self.envs.iter().any(|e| e.parent == Some(env)) {
            if let Some(e) = self.envs.get_mut(env) {
                e.free();
            }
        }
    }

    pub fn make_env(&mut self, parent: Option<EnvRef>) -> EnvRef {
        let env_ref = self.envs.len();
        self.envs.push(Environment::new(parent));
        env_ref
    }

    pub fn add_env(&mut self, hm: HashMap<String, Datum>, symbol_table: &mut SymbolTable) -> EnvRef {
        let env_ref = self.envs.len();

        let bindings: HashMap<usize, Datum> = hm.into_iter().map( |(k, v)|
          (symbol_table.insert(&k), v)
        ).collect();
        self.envs.push(Environment{ bindings: bindings, parent: None });
        env_ref
    }

    pub fn get_env(&self, env_ref: EnvRef) -> &Environment {
        self.envs.get(env_ref).unwrap()
    }

    pub fn get_sym(&self, env_ref: EnvRef, key: Symbol) -> &Datum {
        let Symbol(index) = key;
        let e = self.envs.get(env_ref).unwrap();
        
        match e.bindings.get(&index) {
            Some(v) => v,
            None => {
                match e.parent {
                    Some(r) => self.get_sym(r, key),
                    // TODO: Better error messages w/ real name
                    None => panic!("Key not found {}", &index)
                }
            }
        }
    }

    pub fn define_into_sym(&mut self, env_ref: EnvRef, key: Symbol, value: Datum) -> bool {
        let Symbol(index) = key;
        let mut e = self.envs.get_mut(env_ref).unwrap();
        if e.bindings.contains_key(&index) {
            false
        } else {
            e.bindings.insert(index, value);
            true
        }
    }

    pub fn set_into_sym(&mut self, env_ref: EnvRef, key: Symbol, value: Datum) -> bool {
        let mut cur = env_ref;
        let Symbol(index) = key;

        loop {
            let mut e = self.envs.get_mut(cur).unwrap();
            match e.bindings.get_mut(&index) {
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
