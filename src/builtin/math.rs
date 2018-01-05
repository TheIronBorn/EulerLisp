use std::collections::HashMap;
use rand::{thread_rng, Rng};

use ::LispFn;
use ::Datum;
use ::LispErr::*;
use ::LispResult;
use ::Arity;

use ::builtin::register;

const WITNESSES: [(isize, &[isize]);7] = [
    (2_047, &[2]),
    (1_373_653, &[2, 3]),
    (9_080_191, &[31, 73]),
    (25_326_001, &[2, 3, 5]),
    (3_215_031_751, &[2, 3, 5, 7]),
    (4_759_123_141, &[2, 7, 61]),
    (1_122_004_669_633, &[2, 13, 23, 1662803]),
];

fn modexp(mut base: isize, mut exponent: isize, modulo: isize) -> isize {
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

fn factor2(n: isize) -> (isize, isize) {
    let mut d = n;
    let mut r = 0;

    while (d % 2) == 0 {
        d = d >> 1;
        r += 1;
    }

    (r, d)
}

fn det_miller_rabin(n: isize) -> bool {
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

fn prime_questionmark(vs: &mut [Datum]) -> LispResult {
    if let Datum::Number(n) = vs[0] {
        return Ok(Datum::Bool(det_miller_rabin(n)))
    }
    Err(InvalidTypeOfArguments)
}

fn add(vs: &mut [Datum]) -> LispResult {
    let mut res = 0;
    for v in vs {
        if let Datum::Number(a) = *v {
            res += a;
        } else {
            return Err(InvalidTypeOfArguments);
        }
    }
    Ok(Datum::Number(res))
}

fn subtract(vs: &mut [Datum]) -> LispResult {
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
}

fn mult(vs: &mut [Datum]) -> LispResult {
    let mut res = 1;
    for v in vs {
        if let Datum::Number(a) = *v {
            res *= a;
        } else {
            return Err(InvalidTypeOfArguments);
        }
    }
    Ok(Datum::Number(res))
}

fn fx_add(vs: &mut [Datum]) -> LispResult {
    if let Datum::Number(a) = vs[0] {
        if let Datum::Number(b) = vs[1] {
            return Ok(Datum::Number(a + b));
        }
    }
    Err(InvalidTypeOfArguments)
}

fn fx_subtract(vs: &mut [Datum]) -> LispResult {
    if let Datum::Number(a) = vs[0] {
        if let Datum::Number(b) = vs[1] {
            return Ok(Datum::Number(a - b));
        }
    }
    Err(InvalidTypeOfArguments)
}

fn fx_mult(vs: &mut [Datum]) -> LispResult {
    if let Datum::Number(a) = vs[0] {
        if let Datum::Number(b) = vs[1] {
            return Ok(Datum::Number(a * b));
        }
    }
    Err(InvalidTypeOfArguments)
}

fn max(vs: &mut [Datum]) -> LispResult {
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
}

fn min(vs: &mut [Datum]) -> LispResult {
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
}

fn isqrt(vs: &mut [Datum]) -> LispResult {
    if let Datum::Number(a) = vs[0] {
        let res = (a as f64).sqrt() as isize;
        return Ok(Datum::Number(res));
    }
    Err(InvalidTypeOfArguments)
}

fn inc(vs: &mut [Datum]) -> LispResult {
    if let Datum::Number(a) = vs[0] {
        return Ok(Datum::Number(a + 1));
    }
    Err(InvalidTypeOfArguments)
}

fn dec(vs: &mut [Datum]) -> LispResult {
    if let Datum::Number(a) = vs[0] {
        return Ok(Datum::Number(a - 1));
    }
    Err(InvalidTypeOfArguments)
}

fn zero_questionmark(vs: &mut [Datum]) -> LispResult {
    if let Datum::Number(a) = vs[0] {
        return Ok(Datum::Bool(a == 0));
    }
    Err(InvalidTypeOfArguments)
}

fn divides_questionmark(vs: &mut [Datum]) -> LispResult {
    if let Datum::Number(a) = vs[0] {
        if let Datum::Number(b) = vs[1] {
            return Ok(Datum::Bool((b % a) == 0));
        }
    }
    Err(InvalidTypeOfArguments)
}

fn even_questionmark(vs: &mut [Datum]) -> LispResult {
    if let Datum::Number(a) = vs[0] {
        return Ok(Datum::Bool((a % 2) == 0));
    }
    Err(InvalidTypeOfArguments)
}

fn odd_questionmark(vs: &mut [Datum]) -> LispResult {
    if let Datum::Number(a) = vs[0] {
        return Ok(Datum::Bool((a % 2) == 1));
    }
    Err(InvalidTypeOfArguments)
}

fn div(vs: &mut [Datum]) -> LispResult {
    if let Datum::Number(a) = vs[0] {
        if let Datum::Number(b) = vs[1] {
            return Ok(Datum::Number(a / b));
        }
    }
    Err(InvalidTypeOfArguments)
}

fn shift_left(vs: &mut [Datum]) -> LispResult {
    if let Datum::Number(a) = vs[0] {
        if let Datum::Number(b) = vs[1] {
            return Ok(Datum::Number(a >> b));
        }
    }
    Err(InvalidTypeOfArguments)
}

fn shift_right(vs: &mut [Datum]) -> LispResult {
    if let Datum::Number(a) = vs[0] {
        if let Datum::Number(b) = vs[1] {
            return Ok(Datum::Number(a << b));
        }
    }
    Err(InvalidTypeOfArguments)
}

fn modulo(vs: &mut [Datum]) -> LispResult {
    if let Datum::Number(a) = vs[0] {
        if let Datum::Number(b) = vs[1] {
            return Ok(Datum::Number(a % b));
        }
    }
    Err(InvalidTypeOfArguments)
}

fn divmod(vs: &mut [Datum]) -> LispResult {
    if let Datum::Number(a) = vs[0] {
        if let Datum::Number(b) = vs[1] {
            return Ok(
                Datum::DottedList(
                    vec!(Datum::Number(a / b)),
                    Box::new(Datum::Number(a % b))
                    )
                );
        }
    }
    Err(InvalidTypeOfArguments)
}

fn builtin_modexp(vs: &mut [Datum]) -> LispResult {
    if let Datum::Number(b) = vs[0] {
        if let Datum::Number(e) = vs[1] {
            if let Datum::Number(m) = vs[2] {
                return Ok(Datum::Number(modexp(b, e, m)));
            }
        }
    }
    Err(InvalidTypeOfArguments)
}

fn rand(vs: &mut [Datum]) -> LispResult {
    if let Datum::Number(a) = vs[0] {
        if let Datum::Number(b) = vs[1] {
            return Ok(Datum::Number(thread_rng().gen_range(a, b + 1)));
        }
    }
    Err(InvalidTypeOfArguments)
}

fn factors(vs: &mut [Datum]) -> LispResult {
    if let Datum::Number(a) = vs[0] {
        let mut result = Vec::new();
        let root = (a as f64).sqrt() as isize;

        result.push(Datum::Number(1));
        if a > 1 {
            result.push(Datum::Number(a));
        }
        if a > 2 {
            for i in 2..(root+1) {
                if a % i == 0 {
                    result.push(Datum::Number(i));
                    if (a / i) != i {
                        result.push(Datum::Number(a / i));
                    }
                }
            }
        }
        return Ok(Datum::List(result));
    }
    Err(InvalidTypeOfArguments)
}

pub fn load(hm: &mut HashMap<String, LispFn>) {
    register(hm, "prime?", prime_questionmark, Arity::Exact(1));
    register(hm, "+", add, Arity::Min(2));
    register(hm, "-", subtract, Arity::Min(1));
    register(hm, "*", mult, Arity::Min(2));
    register(hm, "fx+", fx_add, Arity::Exact(2));
    register(hm, "fx-", fx_subtract, Arity::Exact(2));
    register(hm, "fx*", fx_mult, Arity::Exact(2));
    register(hm, "max", max, Arity::Min(2));
    register(hm, "min", min, Arity::Min(2));
    register(hm, "isqrt", isqrt, Arity::Exact(1));
    register(hm, "inc", inc, Arity::Exact(1));
    register(hm, "dec", dec, Arity::Exact(1));
    register(hm, "divides?", divides_questionmark, Arity::Exact(2));
    register(hm, "zero?", zero_questionmark, Arity::Exact(1));
    register(hm, "even?", even_questionmark, Arity::Exact(1));
    register(hm, "odd?", odd_questionmark, Arity::Exact(1));
    register(hm, "/", div, Arity::Exact(2));
    register(hm, ">>", shift_left, Arity::Exact(2));
    register(hm, "<<", shift_right, Arity::Exact(2));
    register(hm, "%", modulo, Arity::Exact(2));
    register(hm, "divmod", divmod, Arity::Exact(2));
    register(hm, "modexp", builtin_modexp, Arity::Exact(3));
    register(hm, "rand", rand, Arity::Exact(2));
    register(hm, "factors", factors, Arity::Exact(1));
}
