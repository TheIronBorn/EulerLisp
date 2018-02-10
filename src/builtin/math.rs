use std::collections::HashMap;
use rand::{thread_rng, Rng};
use std::f64;

use LispFn;
use Datum;
use LispErr::*;
use LispResult;
use Arity;

use builtin::primes::PRIMES;
use builtin::register;
use ::eval::Evaluator;
use ::EnvRef;

fn isqrt(n: isize) -> isize {
    (n as f64).sqrt() as isize
}

fn totient(mut n: isize) -> isize {
    let mut res = n;
    let to = isqrt(n);

    for p in 2..(to+1) {
        if n % p == 0 {
            while n % p == 0 {
                n /= p
            }
            res -= res / p;
        }
    }

    if n > 1 {
        res -= res / n;
    }

    res
}

fn totient_sum(n: isize) -> isize {
    let l = isqrt(n);
    let mut v = vec![0; (l + 1) as usize];
    let floor_nl = n / l;
    let mut big_v = vec![0; (floor_nl + 1) as usize];

    for x in 1..(l + 1) {
        let mut res = (x * (x + 1)) / 2;
        let isqrtx = isqrt(x);

        for g in 2..(isqrtx + 1) {
            res -= v[(x / g) as usize];
        }

        for z in 1..(isqrtx + 1) {
            if z != x / z {
                res -= ((x / z) - (x / (z + 1))) * v[z as usize] 
            }
        }

        v[x as usize] = res;
    }

    for x_ in 1..(floor_nl + 1) {
        let x = floor_nl - x_ + 1;
        let k = n / x;
        let mut res = (k * (k + 1)) / 2;

        let isqrtk = isqrt(k);

        for g in 2..(isqrtk + 1) {
            if (k / g) <= l {
                res -= v[(k / g) as usize];
            } else {
                res -= big_v[(x * g) as usize];
            }
        }

        for z in 1..(isqrtk + 1) {
            if z != (k / z) {
                res -= ((k / z) - (k / (z + 1))) * v[z as usize];
            }
        }

        big_v[x as usize] = res;
    }

    big_v[1]
}

const WITNESSES: [(isize, &[isize]); 10] = [
    (2_047, &[2]),
    (1_373_653, &[2, 3]),
    (9_080_191, &[31, 73]),
    (25_326_001, &[2, 3, 5]),
    (3_215_031_751, &[2, 3, 5, 7]),
    (4_759_123_141, &[2, 7, 61]),
    (1_122_004_669_633, &[2, 13, 23, 1662803]),
    (2_152_302_898_747, &[2, 3, 5, 7, 11]),
    (3_474_749_660_383, &[2, 3, 5, 7, 11, 13]),
    (341_550_071_728_321, &[2, 3, 5, 7, 11, 13, 17])
];

fn modexp(base: isize, exponent: isize, modulo: isize) -> isize {
    let mut c = 1;

    let mut base = base as i128;
    let mut exponent = exponent as i128;
    let modulo = modulo as i128;

    while exponent != 0 {
        if exponent % 2 == 1 {
            exponent -= 1;
            c = (base * c) % modulo;
        }
        exponent /= 2;
        base = (base * base) % modulo;
    }
    (c as isize)
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

    // Check against some obvious candidates first
    if (n % 2) == 0 { return n == 2; } 
    if (n % 3) == 0 { return n == 3; } 
    if (n % 5) == 0 { return n == 5; } 
    if (n % 7) == 0 { return n == 7; } 
    if (n % 11) == 0 { return n == 11; } 
    if (n % 13) == 0 { return n == 13; } 
    if (n % 17) == 0 { return n == 17; } 
    if (n % 19) == 0 { return n == 19; } 
    if (n % 23) == 0 { return n == 23; } 
    if (n % 29) == 0 { return n == 29; } 

    let (s, d) = factor2(n - 1);
    let &(_, witnesses) = WITNESSES.iter().find(|&&(max, _)| max > n).unwrap();

    let n_ = n as i128;
    'witness: for &a in witnesses.iter() {
        let mut x = modexp(a, d, n) as i128;
        if x == 1 || x == n_ - 1 {
            continue 'witness;
        }
        for _ in 0..s {
            x = (x * x) % n_;
            if x == 1 {
                return false;
            }
            if x == n_ - 1 {
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
        let mut res = vs[0].take();
        for v in &mut vs[1..] {
            res = res - v.take();
        }
        Ok(res)
    }
}

fn mult(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let mut res = vs[0].take();

    for v in &mut vs[1..] {
        res = res * v.take();
    }
    Ok(res)
}

fn fx_div(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    if let Datum::Integer(a) = vs[0] {
        if let Datum::Integer(b) = vs[1] {
            return Ok(Datum::Integer(a / b));
        }
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
            return Ok(Datum::make_pair(
                Datum::Integer(a / b),
                Datum::Integer(a % b),
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
            return Ok(Datum::make_list_from_vec(result));
        }

        for i in PRIMES.iter() {
            if a % i == 0 {
                let mut count = 0;
                while a % i == 0 {
                    a /= i;
                    count += 1;
                }
                result.push(Datum::make_pair(
                    Datum::Integer(*i),
                    Datum::Integer(count)
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
                    result.push(Datum::make_pair(
                        Datum::Integer(i),
                        Datum::Integer(count)
                    ));
                }
                i += 2;
                if i > a {
                    break;
                }
            }
        }
        return Ok(Datum::make_list_from_vec(result));
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
        return Ok(Datum::make_list_from_vec(result));
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
        return Ok(Datum::make_list_from_vec(primes));
    }
    Err(InvalidTypeOfArguments)
}

fn digits(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    match vs[0] {
        Datum::Integer(mut a) => {
            let mut result = Vec::new();

            while a != 0 {
                result.push(Datum::from(&(a % 10)));
                a /= 10;
            }

            return Ok(Datum::make_list_from_vec(result))
        },
        Datum::Bignum(ref a) => {
            let digits = a.digits();
            return Ok(Datum::make_list_from_vec(
                    digits.into_iter().map(|d| Datum::Integer(d)).collect()
            ));
        },
        _ => Err(InvalidTypeOfArguments)
    }
}

fn num_digits(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    match vs[0] {
        Datum::Integer(a) => {
            let res = (a as f64).log10().floor() + 1.0;
            return Ok(Datum::Integer(res as isize))
        },
        Datum::Bignum(ref a) => {
            return Ok(Datum::Integer(a.num_digits()))
        },
        ref other => Err(TypeError("num-digits", "integer / bignum", other.clone()))
    }
}

fn digits_to_number(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    if let Datum::Pair(ref ptr) = vs[0] {
        let digits = ptr.borrow().collect_list()?;
        let mut pow = 1;
        let mut result = 0;

        for digit in digits {
            if let Datum::Integer(n) = digit {
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

fn cbrt(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let a = vs[0].as_float()?;
    Ok(Datum::Float(a.cbrt()))
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

fn gcd(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let mut x = vs[0].take().as_integer()?;
    let mut y = vs[1].take().as_integer()?;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }

    Ok(Datum::Integer(x))
}

fn sin(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let v = vs[0].take().as_float()?;
    Ok(Datum::Float(v.sin()))
}

fn cos(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let v = vs[0].take().as_float()?;
    Ok(Datum::Float(v.cos()))
}

fn tan(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let v = vs[0].take().as_float()?;
    Ok(Datum::Float(v.tan()))
}

fn asin(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let v = vs[0].take().as_float()?;
    Ok(Datum::Float(v.asin()))
}

fn acos(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let v = vs[0].take().as_float()?;
    Ok(Datum::Float(v.acos()))
}

fn atan(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let v = vs[0].take().as_float()?;
    Ok(Datum::Float(v.atan()))
}

fn get_pi(_vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    Ok(Datum::Float(f64::consts::PI))
}

fn radiants(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let v = vs[0].take().as_float()?;
    Ok(Datum::Float(v * (f64::consts::PI / 180.0)))
}

fn totient_(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let v = vs[0].take().as_integer()?;
    Ok(Datum::Integer(totient(v)))
}

fn totient_sum_(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let v = vs[0].take().as_integer()?;
    Ok(Datum::Integer(totient_sum(v)))
}

pub fn load(hm: &mut HashMap<String, LispFn>) {
    register(hm, "prime?", prime_questionmark, Arity::Exact(1));
    register(hm, "+", add, Arity::Min(2));
    register(hm, "-", subtract, Arity::Min(1));
    register(hm, "*", mult, Arity::Min(2));
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
    register(hm, "number-of-digits", num_digits, Arity::Exact(1));
    register(hm, "digits->number", digits_to_number, Arity::Exact(1));
    register(hm, "numerator", numerator, Arity::Exact(1));
    register(hm, "denominator", denominator, Arity::Exact(1));
    register(hm, "number->float", to_float, Arity::Exact(1));
    register(hm, "log10", log10, Arity::Exact(1));
    register(hm, "log2", log2, Arity::Exact(1));
    register(hm, "ln", ln, Arity::Exact(1));
    register(hm, "log", log, Arity::Exact(2));
    register(hm, "sqrt", sqrt, Arity::Exact(1));
    register(hm, "cbrt", cbrt, Arity::Exact(1));
    register(hm, "ceil", ceil, Arity::Exact(1));
    register(hm, "floor", floor, Arity::Exact(1));
    register(hm, "round", round, Arity::Exact(1));
    register(hm, "gcd", gcd, Arity::Exact(2));
    register(hm, "sin", sin, Arity::Exact(1));
    register(hm, "cos", cos, Arity::Exact(1));
    register(hm, "tan", tan, Arity::Exact(1));
    register(hm, "asin", asin, Arity::Exact(1));
    register(hm, "acos", acos, Arity::Exact(1));
    register(hm, "atan", atan, Arity::Exact(1));
    register(hm, "radiants", radiants, Arity::Exact(1));
    // TODO: Add builtin constants
    register(hm, "get-pi", get_pi, Arity::Exact(0));
    register(hm, "totient", totient_, Arity::Exact(1));
    register(hm, "totient-sum", totient_sum_, Arity::Exact(1));
}
