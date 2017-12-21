use std::collections::HashMap;
use std::rc::Rc;

use ::Datum;
use ::LispErr::*;

use ::builtin::register;

pub fn load(hm: &mut HashMap<String, Datum>) {
    register(hm, "println", Rc::new(|vs| {
        check_arity!(vs, 1);
        match vs[0] {
            // Print string without " around them
            Datum::Str(ref x) => print!("{}\n", x),
            ref other => println!("{}", other),
        };
        Ok(Datum::Undefined)
    }));
    register(hm, "print", Rc::new(|vs| {
        check_arity!(vs, 1);
        match vs[0] {
            // Print string without " around them
            Datum::Str(ref x) => print!("{}", x),
            ref other => print!("{}", other),
        };
        Ok(Datum::Undefined)
    }));
    register(hm, "inspect", Rc::new(|vs| {
        check_arity!(vs, 1);
        println!("{:?}", vs[0]);
        Ok(Datum::Undefined)
    }));
    register(hm, "not", Rc::new(|vs| {
        check_arity!(vs, 1);
        if let Datum::Bool(b) = vs[0] {
            return Ok(Datum::Bool(!b));
        }
        Err(InvalidTypeOfArguments)
    }));
}
