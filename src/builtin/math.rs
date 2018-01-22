use std::collections::HashMap;
use rand::{thread_rng, Rng};

use LispFn;
use Datum;
use LispErr::*;
use LispResult;
use Arity;

use builtin::primes::PRIMES;
use builtin::register;
use ::eval::Evaluator;
use ::EnvRef;
use ::Fsize;

const WITNESSES: [(isize, &[isize]); 7] = [
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
        return true;
    }

    let (s, d) = factor2(n - 1);
    let &(_, witnesses) = WITNESSES.iter().find(|&&(max, _)| max > n).unwrap();

    'witness: for &a in witnesses.iter() {
        let mut x = modexp(a, d, n);
        if x == 1 || x == n - 1 {
            continue 'witness;
        }
        for _ in 0..s {
            x = (x * x) % n;
            if x == 1 {
                return false;
            }
            if x == n - 1 {
                continue 'witness;
            }
        }
        return false;
    }
    true
}

fn prime_questionmark(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    if let Datum::Integer(n) = vs[0] {
        return Ok(Datum::Bool(det_miller_rabin(n)));
    }
    Err(InvalidTypeOfArguments)
}

fn add(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let mut res = vs[0].take();

    for v in &mut vs[1..] {
        res = res + v.take();
    }
    Ok(res)
}

fn subtract(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    if vs.len() == 1 {
        Ok(-vs[0].take())
    } else {
        Ok(vs[0].take() - vs[1].take())
    }
}

fn mult(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let mut res = vs[0].take();

    for v in &mut vs[1..] {
        res = res * v.take();
    }
    Ok(res)
}

fn fx_add(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    if let Datum::Integer(a) = vs[0] {
        if let Datum::Integer(b) = vs[1] {
            return Ok(Datum::Integer(a + b));
        }
    }
    Err(InvalidTypeOfArguments)
}

fn fx_subtract(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    if let Datum::Integer(a) = vs[0] {
        if let Datum::Integer(b) = vs[1] {
            return Ok(Datum::Integer(a - b));
        }
    }
    Err(InvalidTypeOfArguments)
}

fn fx_mult(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    if let Datum::Integer(a) = vs[0] {
        if let Datum::Integer(b) = vs[1] {
            return Ok(Datum::Integer(a * b));
        }
    }
    Err(InvalidTypeOfArguments)
}

fn fx_div(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    if let Datum::Integer(a) = vs[0] {
        if let Datum::Integer(b) = vs[1] {
            return Ok(Datum::Integer(a / b));
        }
    }
    Err(InvalidTypeOfArguments)
}

fn isqrt(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    if let Datum::Integer(a) = vs[0] {
        let res = (a as f64).sqrt() as isize;
        return Ok(Datum::from(&res));
    }
    Err(InvalidTypeOfArguments)
}

fn inc(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    if let Datum::Integer(a) = vs[0] {
        return Ok(Datum::from(&(a + 1)));
    }
    Err(InvalidTypeOfArguments)
}

fn dec(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    if let Datum::Integer(a) = vs[0] {
        return Ok(Datum::from(&(a - 1)));
    }
    Err(InvalidTypeOfArguments)
}

fn zero_questionmark(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    if let Datum::Integer(a) = vs[0] {
        return Ok(Datum::Bool(a == 0));
    }
    Err(InvalidTypeOfArguments)
}

fn divides_questionmark(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    if let Datum::Integer(a) = vs[0] {
        if let Datum::Integer(b) = vs[1] {
            return Ok(Datum::Bool((b % a) == 0));
        }
    }
    Err(InvalidTypeOfArguments)
}

fn even_questionmark(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    if let Datum::Integer(a) = vs[0] {
        return Ok(Datum::Bool((a % 2) == 0));
    }
    Err(InvalidTypeOfArguments)
}

fn odd_questionmark(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    if let Datum::Integer(a) = vs[0] {
        return Ok(Datum::Bool((a % 2) == 1));
    }
    Err(InvalidTypeOfArguments)
}

fn div(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    Ok(vs[0].take() / vs[1].take())
}

fn modulo(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    Ok(vs[0].take() % vs[1].take())
}

fn divmod(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    if let Datum::Integer(a) = vs[0] {
        if let Datum::Integer(b) = vs[1] {
            return Ok(Datum::DottedList(
                vec![Datum::Integer(a / b)],
                Box::new(Datum::Integer(a % b)),
            ));
        }
    }
    Err(InvalidTypeOfArguments)
}

fn builtin_modexp(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    if let Datum::Integer(b) = vs[0] {
        if let Datum::Integer(e) = vs[1] {
            if let Datum::Integer(m) = vs[2] {
                return Ok(Datum::Integer(modexp(b, e, m)));
            }
        }
    }
    Err(InvalidTypeOfArguments)
}

fn rand(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    if let Datum::Integer(a) = vs[0] {
        if let Datum::Integer(b) = vs[1] {
            return Ok(Datum::from(&thread_rng().gen_range(a, b + 1)));
        }
    }
    Err(InvalidTypeOfArguments)
}

fn prime_factors(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    if let Datum::Integer(mut a) = vs[0] {
        let mut result = Vec::new();
        if a < 2 {
            return Ok(Datum::List(result));
        }

        for i in PRIMES.iter() {
            if a % i == 0 {
                let mut count = 0;
                while a % i == 0 {
                    a /= i;
                    count += 1;
                }
                result.push(Datum::DottedList(
                    vec![Datum::Integer(*i)],
                    Box::new(Datum::Integer(count))
                ));
            }
            if *i > a {
                break;
            }
        }

        let mut i = PRIMES[PRIMES.len() - 1] + 2;
        if a > i {
            loop {
                if a % i == 0 {
                    let mut count = 0;
                    while a % i == 0 {
                        a /= i;
                        count += 1;
                    }
                    result.push(Datum::DottedList(
                        vec![Datum::Integer(i)],
                        Box::new(Datum::Integer(count))
                    ));
                }
                i += 1;
                if i > a {
                    break;
                }
            }
        }
        return Ok(Datum::List(result));
    }
    Err(InvalidTypeOfArguments)
}

fn factors(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    if let Datum::Integer(a) = vs[0] {
        let mut result = Vec::new();
        let root = (a as f64).sqrt() as isize;

        result.push(Datum::Integer(1));
        if a > 1 {
            result.push(Datum::Integer(a));
        }
        if a > 2 {
            for i in 2..(root + 1) {
                if a % i == 0 {
                    result.push(Datum::Integer(i));
                    if (a / i) != i {
                        result.push(Datum::Integer(a / i));
                    }
                }
            }
        }
        return Ok(Datum::List(result));
    }
    Err(InvalidTypeOfArguments)
}

fn primes(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    if let Datum::Integer(a) = vs[0] {
        if a < 1 {
            panic!("Can't take a negative number of primes");
        }

        let to = a as usize;
        if to > PRIMES.len() {
            panic!("There are only {} precalculated primes", PRIMES.len());
        }

        let primes = PRIMES[0..to].to_vec().iter().map(|p|
            Datum::from(p)
        ).collect();
        return Ok(Datum::List(primes));
    }
    Err(InvalidTypeOfArguments)
}

fn digits(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    if let Datum::Integer(mut a) = vs[0] {
        let mut result = Vec::new();

        while a != 0 {
            result.push(Datum::from(&(a % 10)));
            a /= 10;
        }

        return Ok(Datum::List(result))
    }
    Err(InvalidTypeOfArguments)
}

fn digits_to_number(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    if let Datum::List(ref digits) = vs[0] {
        let mut pow = 1;
        let mut result = 0;

        for digit in digits {
            if let Datum::Integer(n) = *digit {
                result += n * pow;
                pow *= 10;
            } else {
                panic!("digits->number only works for lists of numbers")
            }
        }

        return Ok(Datum::from(&result))
    }
    Err(InvalidTypeOfArguments)
}

fn numerator(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    match vs[0].take() {
        Datum::Integer(n) => Ok(Datum::Integer(n)),
        Datum::Rational(r) => Ok(Datum::Integer(r.num)),
       _ => Err(InvalidTypeOfArguments)
    }
}

fn denominator(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    match vs[0].take() {
        Datum::Integer(_) => Ok(Datum::Integer(1)),
        Datum::Rational(r) => Ok(Datum::Integer(r.denom)),
       _ => Err(InvalidTypeOfArguments)
    }
}

fn to_float(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    Ok(Datum::Float(vs[0].as_float()?))
}

fn log10(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let a = vs[0].as_float()?;
    Ok(Datum::Float(a.log10()))
}

fn log2(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let a = vs[0].as_float()?;
    Ok(Datum::Float(a.log2()))
}

fn ln(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let a = vs[0].as_float()?;
    Ok(Datum::Float(a.ln()))
}

fn log(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let a = vs[0].as_float()?;
    let b = vs[1].as_float()?;
    Ok(Datum::Float(a.log(b)))
}

fn sqrt(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let a = vs[0].as_float()?;
    Ok(Datum::Float(a.sqrt()))
}

fn ceil(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let a = vs[0].as_float()?;
    Ok(Datum::Integer(a.ceil() as isize))
}

fn floor(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let a = vs[0].as_float()?;
    Ok(Datum::Integer(a.floor() as isize))
}

fn round(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let a = vs[0].as_float()?;
    Ok(Datum::Integer(a.round() as isize))
}

pub fn load(hm: &mut HashMap<String, LispFn>) {
    register(hm, "prime?", prime_questionmark, Arity::Exact(1));
    register(hm, "+", add, Arity::Min(2));
    register(hm, "-", subtract, Arity::Min(1));
    register(hm, "*", mult, Arity::Min(2));
    register(hm, "fx+", fx_add, Arity::Exact(2));
    register(hm, "fx-", fx_subtract, Arity::Exact(2));
    register(hm, "fx*", fx_mult, Arity::Exact(2));
    register(hm, "isqrt", isqrt, Arity::Exact(1));
    register(hm, "inc", inc, Arity::Exact(1));
    register(hm, "dec", dec, Arity::Exact(1));
    register(hm, "divides?", divides_questionmark, Arity::Exact(2));
    register(hm, "zero?", zero_questionmark, Arity::Exact(1));
    register(hm, "even?", even_questionmark, Arity::Exact(1));
    register(hm, "odd?", odd_questionmark, Arity::Exact(1));
    register(hm, "/", div, Arity::Exact(2));
    register(hm, "%", modulo, Arity::Exact(2));
    register(hm, "div", fx_div, Arity::Exact(2));
    register(hm, "divmod", divmod, Arity::Exact(2));
    register(hm, "modexp", builtin_modexp, Arity::Exact(3));
    register(hm, "rand", rand, Arity::Exact(2));
    register(hm, "factors", factors, Arity::Exact(1));
    register(hm, "prime-factors", prime_factors, Arity::Exact(1));
    register(hm, "primes", primes, Arity::Exact(1));
    register(hm, "number->digits", digits, Arity::Exact(1));
    register(hm, "digits->number", digits_to_number, Arity::Exact(1));
    register(hm, "numerator", numerator, Arity::Exact(1));
    register(hm, "denominator", denominator, Arity::Exact(1));
    register(hm, "->float", to_float, Arity::Exact(1));
    register(hm, "log10", log10, Arity::Exact(1));
    register(hm, "log2", log2, Arity::Exact(1));
    register(hm, "ln", ln, Arity::Exact(1));
    register(hm, "log", log, Arity::Exact(2));
    register(hm, "sqrt", sqrt, Arity::Exact(1));
    register(hm, "ceil", ceil, Arity::Exact(1));
    register(hm, "floor", floor, Arity::Exact(1));
    register(hm, "round", round, Arity::Exact(1));
}
