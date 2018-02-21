use std::collections::HashMap;

use ::Datum;
use ::LispFn;
use ::LispResult;
use ::Arity;
use ::eval::Evaluator;
use ::EnvRef;

mod list;
mod hashmap;
mod primes;
mod math;
mod bitwise;
mod misc;
mod types;
mod comparison;
mod string;
mod bignum;
mod stream;
mod data_structures;

// The difference between builtins and special forms is
// that special forms choose if they want to eval their arguments themselves,
// builtins are called with evaluated arguments

pub fn register(
    hm: &mut HashMap<String, LispFn>,
    name: &str,
    f: fn(&mut [Datum], &mut Evaluator, EnvRef) -> LispResult,
    arity: Arity
) {
    hm.insert(name.to_string(), LispFn(f, arity, String::from(name)));
}

pub fn load(hm: &mut HashMap<String, LispFn>) {
    list::load(hm);
    hashmap::load(hm);
    math::load(hm);
    bitwise::load(hm);
    misc::load(hm);
    string::load(hm);
    types::load(hm);
    comparison::load(hm);
    bignum::load(hm);
    stream::load(hm);
    data_structures::load(hm);
}
