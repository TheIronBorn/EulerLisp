use std::collections::HashMap;
use std::rc::Rc;
use rand::{thread_rng, Rng};

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
    register(hm, ">>", Rc::new(|vs| {
        check_arity!(vs, 2);
        if let Datum::Number(a) = vs[0] {
            if let Datum::Number(b) = vs[1] {
                return Ok(Datum::Number(a >> b));
            }
        }
        Err(InvalidTypeOfArguments)
    }));
    register(hm, "<<", Rc::new(|vs| {
        check_arity!(vs, 2);
        if let Datum::Number(a) = vs[0] {
            if let Datum::Number(b) = vs[1] {
                return Ok(Datum::Number(a << b));
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
    register(hm, "rand", Rc::new(|vs| {
        check_arity!(vs, 2);
        if let Datum::Number(a) = vs[0] {
            if let Datum::Number(b) = vs[1] {
                return Ok(Datum::Number(thread_rng().gen_range(a, b + 1)));
            }
        }
        Err(InvalidTypeOfArguments)
    }));
    register(hm, "powmod", Rc::new(|vs| {
        check_arity!(vs, 3);
        if let Datum::Number(b) = vs[0] {
            if let Datum::Number(e) = vs[1] {
                if let Datum::Number(m) = vs[2] {
                    let mut c = 1;
                    let mut e_ = 0;
                    loop {
                        e_ += 1;
                        c = (b * c) % m;
                        if e_ == e {
                            break;
                        }
                    }

                    return Ok(Datum::Number(c));
                }
            }
        }
        Err(InvalidTypeOfArguments)
    }));
}

