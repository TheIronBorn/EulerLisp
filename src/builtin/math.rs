use std::collections::HashMap;
use std::rc::Rc;
use rand::{thread_rng, Rng};

use ::Datum;
use ::LispErr::*;

use ::builtin::register;

const WITNESSES: [(i64, &[i64]);7] = [
    (2_047, &[2]),
    (1_373_653, &[2, 3]),
    (9_080_191, &[31, 73]),
    (25_326_001, &[2, 3, 5]),
    (3_215_031_751, &[2, 3, 5, 7]),
    (4_759_123_141, &[2, 7, 61]),
    (1_122_004_669_633, &[2, 13, 23, 1662803]),
];

fn modexp(mut base: i64, mut exponent: i64, modulo: i64) -> i64 {
    let mut c = 1;
    while exponent != 0 {
        if exponent % 2 == 1 {
            exponent -= 1;
            c = (base * c) % modulo;
        }
        exponent /= 2;
        base = (base * base) % modulo;
    }
    c
}

fn factor2(n: i64) -> (i64, i64) {
    let mut d = n;
    let mut r = 0;

    while (d % 2) == 0 {
        d = d >> 1;
        r += 1;
    }

    (r, d)
}

fn det_miller_rabin(n: i64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return  true;
    }

    let (s, d) = factor2(n - 1);
    let &(_, witnesses) = WITNESSES.iter().find(|&&(max, _)| max > n).unwrap();

    'witness: for &a in witnesses.iter() {
        let mut x = modexp(a, d, n);
        if x == 1 || x == n - 1 {
            continue 'witness
        }
        for _ in 0..s {
            x = (x * x) % n;
            if x == 1 {
                return false
            }
            if x == n - 1 {
                continue 'witness
            }
        }
        return false;
    }
    true
}

pub fn load(hm: &mut HashMap<String, Datum>) {
    register(hm, "prime?", Rc::new(|vs| {
        check_arity!(vs, 1);
        if let Datum::Number(n) = vs[0] {
            return Ok(Datum::Bool(det_miller_rabin(n)))
        }
        Err(InvalidTypeOfArguments)
    }));
    register(hm, "+", Rc::new(|vs| {
        let mut res = 0;
        for v in vs {
            if let Datum::Number(a) = v {
                res += a;
            } else {
                return Err(InvalidTypeOfArguments);
            }
        }
        Ok(Datum::Number(res))
    }));
    register(hm, "*", Rc::new(|vs| {
        let mut res = 1;
        for v in vs {
            if let Datum::Number(a) = v {
                res *= a;
            } else {
                return Err(InvalidTypeOfArguments);
            }
        }
        Ok(Datum::Number(res))
    }));
    register(hm, "max", Rc::new(|vs| {
        if vs.len() == 0 {
            return Err(InvalidNumberOfArguments);
        }

        let first = vs.get(0).unwrap();
        if let Datum::Number(mut res) = *first {
            for v in vs.iter().skip(1) {
                if let Datum::Number(a) = *v {
                    if a > res {
                        res = a;
                    }
                } else {
                    return Err(InvalidTypeOfArguments);
                }
            }
            Ok(Datum::Number(res))
        } else {
            return Err(InvalidTypeOfArguments);
        }
    }));
    register(hm, "min", Rc::new(|vs| {
        if vs.len() == 0 {
            return Err(InvalidNumberOfArguments);
        }

        let first = vs.get(0).unwrap();
        if let Datum::Number(mut res) = *first {
            for v in vs.iter().skip(1) {
                if let Datum::Number(a) = *v {
                    if a < res {
                        res = a;
                    }
                } else {
                    return Err(InvalidTypeOfArguments);
                }
            }
            Ok(Datum::Number(res))
        } else {
            return Err(InvalidTypeOfArguments);
        }
    }));
    register(hm, "isqrt", Rc::new(|vs| {
        check_arity!(vs, 1);
        if let Datum::Number(a) = vs[0] {
            let res = (a as f64).sqrt() as i64;
            return Ok(Datum::Number(res));
        }
        Err(InvalidTypeOfArguments)
    }));
    register(hm, "inc", Rc::new(|vs| {
        check_arity!(vs, 1);
        if let Datum::Number(a) = vs[0] {
            return Ok(Datum::Number(a + 1));
        }
        Err(InvalidTypeOfArguments)
    }));
    register(hm, "dec", Rc::new(|vs| {
        check_arity!(vs, 1);
        if let Datum::Number(a) = vs[0] {
            return Ok(Datum::Number(a - 1));
        }
        Err(InvalidTypeOfArguments)
    }));
    register(hm, "divides?", Rc::new(|vs| {
        check_arity!(vs, 2);
        if let Datum::Number(a) = vs[0] {
            if let Datum::Number(b) = vs[1] {
                return Ok(Datum::Bool((b % a) == 0));
            }
        }
        Err(InvalidTypeOfArguments)
    }));
    register(hm, "zero?", Rc::new(|vs| {
        check_arity!(vs, 1);
        if let Datum::Number(a) = vs[0] {
            return Ok(Datum::Bool(a == 0));
        }
        Err(InvalidTypeOfArguments)
    }));
    register(hm, "even?", Rc::new(|vs| {
        check_arity!(vs, 1);
        if let Datum::Number(a) = vs[0] {
            return Ok(Datum::Bool((a % 2) == 0));
        }
        Err(InvalidTypeOfArguments)
    }));
    register(hm, "odd?", Rc::new(|vs| {
        check_arity!(vs, 1);
        if let Datum::Number(a) = vs[0] {
            return Ok(Datum::Bool((a % 2) == 1));
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
    register(hm, "modexp", Rc::new(|vs| {
        check_arity!(vs, 3);
        if let Datum::Number(b) = vs[0] {
            if let Datum::Number(e) = vs[1] {
                if let Datum::Number(m) = vs[2] {
                    return Ok(Datum::Number(modexp(b, e, m)));
                }
            }
        }
        Err(InvalidTypeOfArguments)
    }));
}

