use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::Ordering;

use ::Datum;
use ::Symbol;

pub type EnvRef = Rc<RefCell<Env>>;
pub type Binding = Rc<RefCell<Datum>>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Env(HashMap<usize, Binding>);

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

impl Env {
    pub fn new(parent: Option<EnvRef>) -> Self {
        match parent {
            Some(env_ref) => {
                let bindings = env_ref.borrow().0.clone();
                Env(bindings)
            },
            None => {
                Env(HashMap::new())
            }
        }
    }

    pub fn find_def(&self, key: &Symbol) -> Option<Binding> {
        match self.0.get(&key) {
            Some(binding) => Some(binding.clone()),
            None => None
        }
    }

    pub fn contains_key(&self, key: &Symbol) -> bool {
        self.0.contains_key(key)
    }
    
    pub fn extend(&mut self, keys: Vec<Symbol>, values: Vec<Datum>) {
        for (k, v) in keys.iter().zip(values.iter()) {
            self.0.insert(*k, Rc::new(RefCell::new(v.clone())));
        }
    }

    pub fn insert(&mut self, key: Symbol, value: Datum) {
        self.0.insert(key, Rc::new(RefCell::new(value)));
    }

    pub fn define(&mut self, key: Symbol, value: Datum) -> bool {
        self.0.insert(key, Rc::new(RefCell::new(value)));
        true
    }

    pub fn set(&mut self, key: Symbol, value: Datum) -> bool {
        match self.0.get_mut(&key) {
            Some(v) => {
                *v.borrow_mut() = value;
                true
            },
            None => {
                false
            }
        }
    }
}
