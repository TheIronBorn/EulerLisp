use std::collections::HashMap;
use std::rc::Rc;

use ::Datum;
use ::LispErr::*;

use ::builtin::register;

pub fn load(hm: &mut HashMap<String, Datum>) {
    register(hm, "+", Rc::new(|vs| {
        check_arity!(vs, 2);
        if let Datum::Number(a) = vs[0] {
            if let Datum::Number(b) = vs[1] {
                return Ok(Datum::Number(a + b));
            }
        }
        Err(InvalidTypeOfArguments)
    }));
    register(hm, "*", Rc::new(|vs| {
        check_arity!(vs, 2);
        if let Datum::Number(a) = vs[0] {
            if let Datum::Number(b) = vs[1] {
                return Ok(Datum::Number(a * b));
            }
        }
        Err(InvalidTypeOfArguments)
    }));
    register(hm, "/", Rc::new(|vs| {
        check_arity!(vs, 2);
        if let Datum::Number(a) = vs[0] {
            if let Datum::Number(b) = vs[1] {
                return Ok(Datum::Number(a / b));
            }
        }
        Err(InvalidTypeOfArguments)
    }));
    register(hm, "%", Rc::new(|vs| {
        check_arity!(vs, 2);
        if let Datum::Number(a) = vs[0] {
            if let Datum::Number(b) = vs[1] {
                return Ok(Datum::Number(a % b));
            }
        }
        Err(InvalidTypeOfArguments)
    }));
    register(hm, "-", Rc::new(|vs| {
        // TODO: Check arity
        if let Datum::Number(a) = vs[0] {
            if vs.len() == 2 {
                if let Datum::Number(b) = vs[1] {
                    return Ok(Datum::Number(a - b));
                } else {
                    return Ok(Datum::Number(-a));
                }
            }
        }
        Err(InvalidTypeOfArguments)
    }));
}
