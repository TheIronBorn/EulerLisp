use std::collections::HashMap;

use ::LispFn;
use ::Datum;
use ::LispErr::*;
use ::LispResult;
use ::LispErr;
use ::Arity;

use ::bignum;
use ::builtin::register;
use ::bignum::Bignum;
use ::eval::Evaluator;
use ::EnvRef;

fn number_to_bignum(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    Ok(Datum::Bignum(Bignum::new(vs[0].as_integer()?)))
}

fn bignum_from_digits(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let digits = vs[0].as_pair()?.collect_list()?;
    let mut chunks = Vec::new();
    let mut pow = 1;
    let mut result = 0;

    for digit in digits {
        result += digit.as_integer()? * pow;
        pow *= 10;

        if pow == (bignum::CHUNK as isize) {
            pow = 1;
            chunks.push(result as usize);
            result = 0;
        }
    }
    if result != 0 {
        chunks.push(result as usize);
    }

    Ok(Datum::Bignum(Bignum::from_chunks(chunks)))
}

fn bignum_chunks(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    if let Datum::Bignum(a) = vs[0].clone() {
        let digits = a.chunks();
        return Ok(Datum::make_list_from_vec(
            digits.into_iter().map(|d| Datum::Integer(d)).collect()
        ));
    }
    return Err(TypeError("bignum-chunks", "bignum", vs[0].clone()))
}

fn bignum_from_chunks(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let chunks = vs[0].as_pair()?.collect_list()?;
    let result : Result<Vec<usize>, LispErr> = chunks.into_iter().map(|c| c.as_uinteger()).collect();
    Ok(Datum::Bignum(Bignum::from_chunks(result?)))
}

pub fn load(hm: &mut HashMap<String, LispFn>) {
    register(hm, "bignum", number_to_bignum, Arity::Exact(1));
    register(hm, "digits->bignum", bignum_from_digits, Arity::Exact(1));
    register(hm, "bignum-chunks", bignum_chunks, Arity::Exact(1));
    register(hm, "chunks->bignum", bignum_from_chunks, Arity::Exact(1));
}
