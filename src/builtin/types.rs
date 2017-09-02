use std::collections::HashMap;
use std::rc::Rc;

use ::Value;
use ::LispErr::*;

use ::builtin::register;

pub fn load(hm: &mut HashMap<String, Value>) {
    register(hm, "pair?", Rc::new(|vs| {
        check_arity!(vs, 1);
        Ok(Value::Bool(vs[0].is_pair()))
    }));

    register(hm, "list?", Rc::new(|vs| {
        check_arity!(vs, 1);
        Ok(Value::Bool(vs[0].is_list()))
    }));

    register(hm, "nil?", Rc::new(|vs| {
        check_arity!(vs, 1);
        Ok(Value::Bool(vs[0].is_nil()))
    }));
}
