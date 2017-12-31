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
}

pub struct EnvArena {
    envs: HashMap<EnvRef, Environment>,
    refs: HashMap<EnvRef, usize>,
    index: usize,
}

impl EnvArena {
    pub fn new() -> Self {
        Self { envs: HashMap::new(), refs: HashMap::new(), index: 0 }
    }

    pub fn size(&self) -> usize {
        self.index
    }

    pub fn free(&mut self, env: EnvRef) {
        let refs = self.refs.get(&env).unwrap();
        if env != 0 && *refs == 0 {
            self.envs.remove(&env);
            // TODO: Decrease refs of parent & delete if = 0
        }
    }

    pub fn make_env(&mut self, parent: Option<EnvRef>) -> EnvRef {
        let i = self.index;
        self.index += 1;
        if let Some(parent_ref) = parent {
            *self.refs.entry(parent_ref).or_insert(0) += 1;
        }
        self.refs.insert(i, 0);
        self.envs.insert(i, Environment::new(parent));
        i
    }

    pub fn add_env(&mut self, hm: HashMap<String, Datum>, symbol_table: &mut SymbolTable) -> EnvRef {
        let bindings: HashMap<usize, Datum> = hm.into_iter().map( |(k, v)|
          (symbol_table.insert(&k), v)
        ).collect();

        let i = self.index;
        self.index += 1;
        self.refs.insert(i, 0);
        self.envs.insert(i, Environment{ bindings: bindings, parent: None });
        i
    }

    pub fn get(&self, env_ref: EnvRef, key: Symbol) -> Option<&Datum> {
        let Symbol(index) = key;
        let e = self.envs.get(&env_ref).unwrap_or_else(
            || panic!("Trying to get symbol in invalid env {}", env_ref)
        );
        
        match e.bindings.get(&index) {
            Some(v) => Some(v),
            None => {
                match e.parent {
                    Some(r) => self.get(r, key),
                    None => None
                }
            }
        }
    }

    fn find_env(&self, ienv_ref: EnvRef, key: Symbol) -> Option<EnvRef> {
        let Symbol(index) = key;
        let mut env_ref = ienv_ref;

        loop {
            let maybe_e = self.envs.get(&env_ref);
            let e = maybe_e.unwrap_or_else(
                || panic!("Trying to get symbol in invalid env {}", env_ref)
            );
            match e.bindings.get(&index) {
                Some(_) => {
                    return Some(env_ref)
                },
                None => {
                    match e.parent {
                        Some(r) => {
                            env_ref = r;
                        }
                        None => {
                            return None
                        }
                    }
                }
            }
        }
    }

    pub fn get_mut(&mut self, env_ref: EnvRef, key: Symbol) -> Option<&mut Datum> {
        let source = self.find_env(env_ref, key.clone())?;
        let env = self.envs.get_mut(&source)?;

        let Symbol(index) = key;
        env.bindings.get_mut(&index)
    }
    
    pub fn extend(&mut self, env_ref: EnvRef, keys: Vec<Symbol>, values: Vec<Datum>) {
        let e = self.envs.get_mut(&env_ref).unwrap();
        for (k, v) in keys.iter().zip(values.iter()) {
            let Symbol(index) = *k;
            e.bindings.insert(index, v.clone());
        }
    }

    pub fn define(&mut self, env_ref: EnvRef, key: Symbol, value: Datum) -> bool {
        let Symbol(index) = key;
        // println!("{:?}", self.envs);
        // println!("Def into env {}", env_ref);
        let e = self.envs.get_mut(&env_ref).unwrap();
        if e.bindings.contains_key(&index) {
            false
        } else {
            e.bindings.insert(index, value);
            true
        }
    }

    pub fn set(&mut self, env_ref: EnvRef, key: Symbol, value: Datum) -> bool {
        let mut cur = env_ref;
        let Symbol(index) = key;

        loop {
            let e = self.envs.get_mut(&cur).unwrap();
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
