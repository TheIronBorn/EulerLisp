use std::collections::HashMap;
use std::rc::Rc;

use ::Datum;
use ::LispFn;
use ::LispResult;

use macros;

mod list;
mod math;
mod misc;
mod pair;
mod types;
mod comparison;

// The difference between builtins and special forms is
// that special forms choose if they want to eval their arguments themselves,
// builtins are called with evaluated arguments

pub fn register(hm: &mut HashMap<String, Datum>, name: &str,
            f: Rc<Fn(Vec<Datum>)->LispResult>) {
    hm.insert(name.to_string(), Datum::Builtin(LispFn(f)));
}


pub fn load(hm: &mut HashMap<String, Datum>) {
    list::load(hm);
    math::load(hm);
    misc::load(hm);
    pair::load(hm);
    types::load(hm);
    comparison::load(hm);
}
