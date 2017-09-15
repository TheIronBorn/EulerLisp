use std::collections::HashMap;
use std::rc::Rc;

use ::Datum;
use ::LispErr::*;

use ::builtin::register;

pub fn load(hm: &mut HashMap<String, Datum>) {
    register(hm, "=", Rc::new(|vs| {
        check_arity!(vs, 2);
        Ok(Datum::Bool(vs[0] == vs[1]))
    }));
    register(hm, "!=", Rc::new(|vs| {
        check_arity!(vs, 2);
        Ok(Datum::Bool(vs[0] != vs[1]))
    }));
    register(hm, ">", Rc::new(|vs| {
        check_arity!(vs, 2);
        Ok(Datum::Bool(vs[0] > vs[1]))
    }));
    register(hm, "<", Rc::new(|vs| {
        check_arity!(vs, 2);
        Ok(Datum::Bool(vs[0] < vs[1]))
    }));
    register(hm, ">=", Rc::new(|vs| {
        check_arity!(vs, 2);
        Ok(Datum::Bool(vs[0] >= vs[1]))
    }));
    register(hm, "<=", Rc::new(|vs| {
        check_arity!(vs, 2);
        Ok(Datum::Bool(vs[0] <= vs[1]))
    }));
}
