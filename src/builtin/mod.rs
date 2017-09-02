use std::collections::HashMap;
use std::rc::Rc;

use ::Value;
use ::LispFn;
use ::LispResult;

use macros;

mod list;
mod math;
mod misc;
mod pair;
mod types;

// The difference between builtins and special forms is
// that special forms choose if they want to eval their arguments themselves,
// builtins are called with evaluated arguments

pub fn register(hm: &mut HashMap<String, Value>, name: &str,
            f: Rc<Fn(Vec<Value>)->LispResult>) {
    hm.insert(name.to_string(), Value::Builtin(LispFn(f)));
}


pub fn load(hm: &mut HashMap<String, Value>) {
    list::load(hm);
    math::load(hm);
    misc::load(hm);
    pair::load(hm);
    types::load(hm);
}
