use std::collections::HashMap;

use ::Datum;
use ::LispFn;
use ::LispErr::*;
use ::LispResult;

use ::builtin::register;

fn pair_questionmark(vs: Vec<Datum>) -> LispResult {
    check_arity!(vs, 1);
    Ok(Datum::Bool(vs[0].is_pair()))
}

fn list_questionmark(vs: Vec<Datum>) -> LispResult {
    check_arity!(vs, 1);
    Ok(Datum::Bool(vs[0].is_list()))
}

fn nil_qustionmark(vs: Vec<Datum>) -> LispResult {
    check_arity!(vs, 1);
    Ok(Datum::Bool(vs[0].is_nil()))
}

pub fn load(hm: &mut HashMap<String, LispFn>) {
    register(hm, "pair?", pair_questionmark);
    register(hm, "list?", list_questionmark);
    register(hm, "nil?", nil_qustionmark);
}
