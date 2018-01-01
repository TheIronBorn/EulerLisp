use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::Ordering;

use ::Datum;
use ::Symbol;

pub type EnvRef = Rc<RefCell<Env>>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Env {
    pub bindings: HashMap<usize, Datum>,
    pub parent: Option<EnvRef>
}

impl PartialOrd for Env {
    fn partial_cmp(&self, _: &Env) -> Option<Ordering> {
        None
    }
}

impl Ord for Env {
    fn cmp(&self, _: &Env) -> Ordering {
        Ordering::Equal
    }
}

pub fn find_def_env(env: EnvRef, key: Symbol) -> Option<EnvRef> {
    let cloned = env.clone();
    let e = cloned.borrow();
    if e.bindings.contains_key(&key) {
        return Some(env);
    } else {
        match e.parent {
            Some(ref parent_ref) => { return find_def_env(parent_ref.clone(), key); },
            None => { return None; },
        };
    }
}

impl Env {
    pub fn new(parent: Option<EnvRef>) -> Self {
        Env { bindings: HashMap::new(), parent: parent }
    }

    
    pub fn extend(&mut self, keys: Vec<Symbol>, values: Vec<Datum>) {
        for (k, v) in keys.iter().zip(values.iter()) {
            self.bindings.insert(*k, v.clone());
        }
    }

    pub fn define(&mut self, key: Symbol, value: Datum) -> bool {
        if self.bindings.contains_key(&key) {
            false
        } else {
            self.bindings.insert(key, value);
            true
        }
    }

    pub fn set(&mut self, key: Symbol, value: Datum) -> bool {
        match self.bindings.get_mut(&key) {
            Some(v) => {
                *v = value;
                return true;
            },
            None => {
                match self.parent.clone() {
                    Some(r) => { return r.borrow_mut().set(key, value); },
                    None => { return false; },
                };
            }
        };
    }
}
