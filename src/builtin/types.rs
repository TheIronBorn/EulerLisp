use std::collections::HashMap;

use ::Datum;
use ::LispFn;
use ::LispResult;
use ::Arity;

use ::builtin::register;

fn pair_questionmark(vs: Vec<Datum>) -> LispResult {
    Ok(Datum::Bool(vs[0].is_pair()))
}

fn list_questionmark(vs: Vec<Datum>) -> LispResult {
    Ok(Datum::Bool(vs[0].is_list()))
}

fn nil_qustionmark(vs: Vec<Datum>) -> LispResult {
    Ok(Datum::Bool(vs[0].is_nil()))
}

pub fn load(hm: &mut HashMap<String, LispFn>) {
    register(hm, "pair?", pair_questionmark, Arity::Exact(1));
    register(hm, "list?", list_questionmark, Arity::Exact(1));
    register(hm, "nil?", nil_qustionmark, Arity::Exact(1));
}
