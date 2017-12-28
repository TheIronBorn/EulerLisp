use std::collections::HashMap;
use std::rc::Rc;

use ::Datum;
use ::LispErr::*;

use ::builtin::register;

pub fn load(hm: &mut HashMap<String, Datum>) {
    register(hm, "list", Rc::new(|vs| {
        if vs.len() == 0 {
            Ok(Datum::Nil)
        } else {
            Ok(Datum::List(vs))
        }
    }));

    register(hm, "nth", Rc::new(|vs| {
        check_arity!(vs, 2);
        // TODO: Handle index out of bounds error
        if let Datum::Number(n) = vs[0] {
            if let Datum::List(ref elems) = vs[1] {
                return Ok(elems.get(n as usize).unwrap().clone());
            }
        }
        Err(InvalidTypeOfArguments)
    }));

    register(hm, "length", Rc::new(|vs| {
        check_arity!(vs, 1);
        match vs[0] {
            Datum::Nil => Ok(Datum::Number(0)),
            Datum::List(ref elems) => {
                Ok(Datum::Number(elems.len() as i64))
            },
            _ => Err(InvalidTypeOfArguments)
        }
    }));

    register(hm, "append", Rc::new(|vs| {
        check_arity!(vs, 2);
        match vs[0] {
            Datum::Nil => {
                Ok(vs[1].clone())
            },
            Datum::List(ref elems) => {
                match vs[1] {
                    Datum::List(ref elems2) => {
                        let mut new_elems = elems.clone();
                        new_elems.extend(elems2.iter().cloned());
                        Ok(Datum::List(new_elems))
                    },
                    Datum::DottedList(ref elems2, ref tail) => {
                        let mut new_elems = elems.clone();
                        new_elems.extend(elems2.iter().cloned());
                        Ok(Datum::DottedList(new_elems, tail.clone()))
                    },
                    Datum::Nil => {
                        Ok(Datum::List(elems.clone()))
                    },
                    ref other => {
                        Ok(Datum::DottedList(elems.clone(), Box::new(other.clone())))
                    }
                }
            },
            _ => Err(InvalidTypeOfArguments)
        }
    }));

    register(hm, "push", Rc::new(|vs| {
        check_arity!(vs, 2);
        match vs[0] {
            Datum::Nil => {
                Ok(Datum::List(vec!(vs[1].clone())))
            },
            Datum::List(ref elems) => {
                let mut new_elems = elems.clone();
                new_elems.push(vs[1].clone());
                Ok(Datum::List(new_elems))
            },
            _ => Err(InvalidTypeOfArguments)
        }
    }));

    register(hm, "reverse", Rc::new(|vs| {
        check_arity!(vs, 1);

        match vs[0] {
            Datum::List(ref elems) => {
                let mut new_elems = elems.iter().rev().cloned().collect();
                Ok(Datum::List(new_elems))
            },
            _ => Err(InvalidTypeOfArguments),
        }
    }));
}
