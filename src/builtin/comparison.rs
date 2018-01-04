use std::collections::HashMap;

use ::Datum;
use ::LispFn;
use ::LispResult;
use ::Arity;

use ::builtin::register;

fn eq(vs: &mut [Datum]) -> LispResult {
    Ok(Datum::Bool(vs[0] == vs[1]))
}

fn neq(vs: &mut [Datum]) -> LispResult {
    Ok(Datum::Bool(vs[0] != vs[1]))
}

fn lt(vs: &mut [Datum]) -> LispResult {
    Ok(Datum::Bool(vs[0] < vs[1]))
}

fn gt(vs: &mut [Datum]) -> LispResult {
    Ok(Datum::Bool(vs[0] > vs[1]))
}

fn lte(vs: &mut [Datum]) -> LispResult {
    Ok(Datum::Bool(vs[0] <= vs[1]))
}

fn gte(vs: &mut [Datum]) -> LispResult {
    Ok(Datum::Bool(vs[0] >= vs[1]))
}

pub fn load(hm: &mut HashMap<String, LispFn>) {
    register(hm, "=", eq, Arity::Exact(2));
    register(hm, "!=", neq, Arity::Exact(2));
    register(hm, "<", lt, Arity::Exact(2));
    register(hm, ">", gt, Arity::Exact(2));
    register(hm, "<=", lte, Arity::Exact(2));
    register(hm, ">=", gte, Arity::Exact(2));
}
