use std::collections::HashMap;

use ::Datum;
use ::LispErr::*;
use ::LispResult;

use ::builtin::register;

fn eq(vs: Vec<Datum>) -> LispResult {
    check_arity!(vs, 2);
    Ok(Datum::Bool(vs[0] == vs[1]))
}

fn neq(vs: Vec<Datum>) -> LispResult {
    check_arity!(vs, 2);
    Ok(Datum::Bool(vs[0] != vs[1]))
}

fn lt(vs: Vec<Datum>) -> LispResult {
    check_arity!(vs, 2);
    Ok(Datum::Bool(vs[0] < vs[1]))
}

fn gt(vs: Vec<Datum>) -> LispResult {
    check_arity!(vs, 2);
    Ok(Datum::Bool(vs[0] > vs[1]))
}

fn lte(vs: Vec<Datum>) -> LispResult {
    check_arity!(vs, 2);
    Ok(Datum::Bool(vs[0] <= vs[1]))
}

fn gte(vs: Vec<Datum>) -> LispResult {
    check_arity!(vs, 2);
    Ok(Datum::Bool(vs[0] >= vs[1]))
}

pub fn load(hm: &mut HashMap<String, Datum>) {
    register(hm, "=", eq);
    register(hm, "!=", neq);
    register(hm, "<", lt);
    register(hm, ">", gt);
    register(hm, "<=", lte);
    register(hm, ">=", gte);
}
