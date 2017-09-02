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

    register(hm, "append", Rc::new(|vs| {
        check_arity!(vs, 2);
        match vs[0] {
            Value::Nil => {
                Ok(vs[1].clone())
            },
            Value::List(ref elems) => {
                match vs[1] {
                    Value::List(ref elems2) => {
                        let mut new_elems = elems.clone();
                        new_elems.extend(elems2.iter().cloned());
                        Ok(Value::List(new_elems))
                    },
                    Value::DottedList(ref elems2) => {
                        let mut new_elems = elems.clone();
                        new_elems.extend(elems2.iter().cloned());
                        Ok(Value::DottedList(new_elems))
                    },
                    ref other => {
                        let mut new_elems = elems.clone();
                        new_elems.push(other.clone());
                        Ok(Value::DottedList(new_elems))
                    }
                }
            },
            _ => Err(InvalidTypeOfArguments)
        }
    }));

    register(hm, "reverse", Rc::new(|vs| {
        check_arity!(vs, 1);

        match vs[0] {
            Value::List(ref elems) => {
                let mut new_elems = elems.iter().rev().cloned().collect();
                Ok(Value::List(new_elems))
            },
            _ => Err(InvalidTypeOfArguments),
        }
    }));
}
