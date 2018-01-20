use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::Ordering;

use ::Datum;
use ::Symbol;

pub type EnvRef = Rc<RefCell<Env>>;
pub type Binding = Rc<RefCell<Datum>>;

#[derive(Clone, Debug, PartialEq)]
pub struct Env(HashMap<usize, Binding>, Option<EnvRef>);
impl Env {
    pub fn new(parent: Option<EnvRef>) -> Self {
        Env(HashMap::new(), parent)
    }

    pub fn find_def(&self, key: &Symbol) -> Option<Binding> {
        if let Some(binding) = self.0.get(&key) {
            Some(binding.clone())
        } else {
            if let Some(ref env_ref) = self.1 {
                env_ref.borrow().find_def(key)
            } else {
                None
            }
        }
    }

    pub fn extend(&mut self, keys: Vec<Symbol>, values: Vec<Datum>) {
        for (k, v) in keys.iter().zip(values.into_iter()) {
            self.0.insert(*k, Rc::new(RefCell::new(v)));
        }
    }

    pub fn define(&mut self, key: Symbol, value: Datum) {
        self.0.insert(key, Rc::new(RefCell::new(value)));
    }

    pub fn set(&mut self, key: Symbol, value: Datum) -> bool {
        match self.find_def(&key) {
            Some(v) => {
                *v.borrow_mut() = value;
                true
            },
            None => false
        }
    }
}

// This type of environment is only needed
// during the preprocessing phase.
// There we convert each reference to a variable
// to a `ARef(depth, binding)`
// where `depth` tells us how many environments
// we need to move up to find a binding
// and `binding` is the index of this binding
// in a list.
//
// Using these two numbers,
// we can repersent environments without HashMaps
// and access to variables should be faster, too

#[derive(Clone, Debug, PartialEq)]
pub struct ARef(usize, usize);

#[derive(Clone, Debug, PartialEq)]
pub struct AEnv {
    bindings: HashMap<usize, usize>,
    parent: Option<AEnvRef>,
    counter: usize
}

pub type AEnvRef = Rc<RefCell<AEnv>>;
impl AEnv {
    pub fn new(parent: Option<AEnvRef>) -> Self {
        AEnv {
            bindings: HashMap::new(),
            parent: parent,
            counter: 0
        }
    }

    pub fn lookup_with_depth(&self, key: &Symbol, depth: usize) -> Option<ARef> {
        if let Some(binding) = self.bindings.get(&key) {
            Some(ARef(depth, *binding))
        } else {
            if let Some(ref env_ref) = self.parent {
                env_ref.borrow().lookup_with_depth(key, depth + 1)
            } else {
                None
            }
        }
    }

    pub fn lookup(&self, key: &Symbol) -> Option<ARef> {
        self.lookup_with_depth(key, 0)
    }

    pub fn insert(&mut self, key: &Symbol) -> Option<ARef> {
        let exists = self.bindings.get(key).is_some();

        if exists {
            None
        } else {
            let a = ARef(0, self.counter);
            self.bindings.insert(key.clone(), self.counter);
            self.counter += 1;
            Some(a)
        }
    }

    pub fn extend(&mut self, keys: Vec<Symbol>) {
        for k in keys.iter() {
            self.bindings.insert(*k, self.counter);
            self.counter += 1;
        }
    }
}
