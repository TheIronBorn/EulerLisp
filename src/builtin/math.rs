use std::collections::HashMap;
use std::rc::Rc;

use ::Value;
use ::LispErr::*;

use ::builtin::register;

pub fn load(hm: &mut HashMap<String, Value>) {
    register(hm, "+", Rc::new(|vs| {
        check_arity!(vs, 2);
        if let Value::Number(a) = vs[0] {
            if let Value::Number(b) = vs[1] {
                return Ok(Value::Number(a + b));
            }
        }
        Err(InvalidTypeOfArguments)
    }));
    register(hm, "*", Rc::new(|vs| {
        check_arity!(vs, 2);
        if let Value::Number(a) = vs[0] {
            if let Value::Number(b) = vs[1] {
                return Ok(Value::Number(a * b));
            }
        }
        Err(InvalidTypeOfArguments)
    }));
    register(hm, "/", Rc::new(|vs| {
        check_arity!(vs, 2);
        if let Value::Number(a) = vs[0] {
            if let Value::Number(b) = vs[1] {
                return Ok(Value::Number(a / b));
            }
        }
        Err(InvalidTypeOfArguments)
    }));
    register(hm, "%", Rc::new(|vs| {
        check_arity!(vs, 2);
        if let Value::Number(a) = vs[0] {
            if let Value::Number(b) = vs[1] {
                return Ok(Value::Number(a % b));
            }
        }
        Err(InvalidTypeOfArguments)
    }));
    register(hm, "-", Rc::new(|vs| {
        // TODO: Check arity
        if let Value::Number(a) = vs[0] {
            if vs.len() == 2 {
                if let Value::Number(b) = vs[1] {
                    return Ok(Value::Number(a - b));
                } else {
                    return Ok(Value::Number(-a));
                }
            }
        }
        Err(InvalidTypeOfArguments)
    }));
}
