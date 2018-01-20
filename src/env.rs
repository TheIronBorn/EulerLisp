use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use ::Datum;
use ::Symbol;
use ::BindingRef;

pub type EnvRef = Rc<RefCell<Env>>;
pub type Binding = Rc<RefCell<Datum>>;

#[derive(Clone, Debug, PartialEq)]
pub struct Env {
    bindings: Vec<Binding>,
    parent: Option<EnvRef>
}

impl Env {
    pub fn new(parent: Option<EnvRef>) -> Self {
        Env {
            bindings: Vec::new(),
            parent: parent
        }
    }

    pub fn get_binding(&self, binding: BindingRef) -> Binding {
        let BindingRef(depth, index) = binding;
        self.get_binding_(depth, index)
    }

    pub fn get_binding_(&self, depth: usize, binding: usize) -> Binding {
        if depth == 0 {
            self.bindings.get(binding).expect("Trying to get undefined binding").clone()
        } else {
            if let Some(ref parent) = self.parent {
                parent.borrow().get_binding_(depth - 1, binding)
            } else {
                panic!("Trying to get binding with non-zero depth in root env");
            }
        }
    }

    pub fn extend(&mut self, values: Vec<Datum>) {
        let bindings = values.into_iter().map( |v| Rc::new(RefCell::new(v)) );
        self.bindings.extend(bindings);
    }
}

// This type of environment is only needed
// during the preprocessing phase.
// There we convert each reference to a variable
// to a `BindingRef(depth, binding)`
// where `depth` tells us how many environments
// we need to move up to find a binding
// and `binding` is the index of this binding
// in a list.
//
// Using these two numbers,
// we can repersent environments without HashMaps
// and access to variables should be faster, too

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

    pub fn lookup_with_depth(&self, key: &Symbol, depth: usize) -> Option<BindingRef> {
        if let Some(binding) = self.bindings.get(&key) {
            Some(BindingRef(depth, *binding))
        } else {
            if let Some(ref env_ref) = self.parent {
                env_ref.borrow().lookup_with_depth(key, depth + 1)
            } else {
                None
            }
        }
    }

    pub fn lookup(&self, key: &Symbol) -> Option<BindingRef> {
        self.lookup_with_depth(key, 0)
    }

    pub fn insert(&mut self, key: &Symbol) -> Option<BindingRef> {
        let exists = self.bindings.get(key).is_some();

        if exists {
            None
        } else {
            let a = BindingRef(0, self.counter);
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
