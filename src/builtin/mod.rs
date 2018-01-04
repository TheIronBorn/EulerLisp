use std::collections::HashMap;

use ::Datum;
use ::LispFn;

mod list;
mod math;
mod misc;
mod types;
mod comparison;
mod string;

// The difference between builtins and special forms is
// that special forms choose if they want to eval their arguments themselves,
// builtins are called with evaluated arguments

pub fn register(hm: &mut HashMap<String, LispFn>, name: &str, f: LispFn) {
    hm.insert(name.to_string(), f);
}

pub fn load(hm: &mut HashMap<String, LispFn>) {
    list::load(hm);
    math::load(hm);
    misc::load(hm);
    string::load(hm);
    types::load(hm);
    comparison::load(hm);
}
