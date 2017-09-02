use std::collections::HashMap;
use std::rc::Rc;

use ::Value;
use ::LispErr::*;

use ::builtin::register;

pub fn load(hm: &mut HashMap<String, Value>) {
    register(hm, "length", Rc::new(|vs| {
        check_arity!(vs, 1);
        match vs[0] {
            Value::Nil => Ok(Value::Number(0)),
            Value::List(ref elems) => {
                Ok(Value::Number(elems.len() as i64))
            },
            _ => Err(InvalidTypeOfArguments)
        }
    }));
}
