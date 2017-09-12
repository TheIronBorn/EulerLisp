use std::collections::HashMap;
use std::rc::Rc;

use ::Datum;
use ::LispErr::*;

use ::builtin::register;

pub fn load(hm: &mut HashMap<String, Datum>) {
    register(hm, "pair?", Rc::new(|vs| {
        check_arity!(vs, 1);
        Ok(Datum::Bool(vs[0].is_pair()))
    }));

    register(hm, "list?", Rc::new(|vs| {
        check_arity!(vs, 1);
        Ok(Datum::Bool(vs[0].is_list()))
    }));

    register(hm, "nil?", Rc::new(|vs| {
        check_arity!(vs, 1);
        Ok(Datum::Bool(vs[0].is_nil()))
    }));
}
