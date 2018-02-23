use std::collections::HashMap;

use ::LispFn;
use ::Datum;
use ::LispErr::*;
use ::LispResult;
use ::Arity;

use std::rc::Rc;
use std::cell::RefCell;

use stream::RangeStream;
use stream::StepStream;
use stream::MapStream;
use stream::FlatMapListStream;
use stream::FlatMapStreamStream;
use stream::AccumulateStream;
use stream::EnumerateStream;
use stream::SelectStream;
use stream::PermutationStream;
use stream::CombinationStream;

use ::eval::Evaluator;
use ::EnvRef;
use ::builtin::register;

fn step(vs: &mut [Datum], eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    if let Datum::Integer(from) = *(vs.get(0).unwrap_or(&Datum::Integer(0))) {
        if let Datum::Integer(step) = *(vs.get(1).unwrap_or(&Datum::Integer(1))) {
            return Ok(Datum::Stream(eval.get_unique_id(), Box::new(Rc::new(RefCell::new(StepStream::new(from, step))))))
        }
    }

    Err(InvalidTypeOfArguments)
}

fn permutations(vs: &mut [Datum], eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let array = vs[0].as_list()?;
    Ok(Datum::Stream(eval.get_unique_id(), Box::new(Rc::new(RefCell::new(PermutationStream::new(array))))))
}

fn combinations(vs: &mut [Datum], eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let size = vs[0].as_integer()?;
    let array = vs[1].as_list()?;
    Ok(Datum::Stream(eval.get_unique_id(), Box::new(Rc::new(RefCell::new(CombinationStream::new(array, size as usize))))))
}

fn range(vs: &mut [Datum], eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let from = vs[0].as_integer()?;
    let to = vs[1].as_integer()?;
    if vs.len() == 3 {
        let step = vs[2].as_integer()?;
        Ok(Datum::Stream(eval.get_unique_id(), Box::new(Rc::new(RefCell::new(RangeStream::new(from, to, step))))))
    } else {
        Ok(Datum::Stream(eval.get_unique_id(), Box::new(Rc::new(RefCell::new(RangeStream::new(from, to, 1))))))
    }
}

fn map(vs: &mut [Datum], eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let fun = vs[0].clone();
    let s = vs[1].as_stream()?;

    Ok(Datum::Stream(eval.get_unique_id(), Box::new(Rc::new(RefCell::new(MapStream::new(*s, fun))))))
}

fn flatmap_list(vs: &mut [Datum], eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let fun = vs[0].clone();
    let s = vs[1].as_stream()?;

    Ok(Datum::Stream(eval.get_unique_id(), Box::new(Rc::new(RefCell::new(FlatMapListStream::new(*s, fun))))))
}

fn flatmap_stream(vs: &mut [Datum], eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let fun = vs[0].clone();
    let s = vs[1].as_stream()?;

    Ok(Datum::Stream(eval.get_unique_id(), Box::new(Rc::new(RefCell::new(FlatMapStreamStream::new(*s, fun))))))
}

fn accumulate(vs: &mut [Datum], eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let fun = vs[0].clone();
    let acc = vs[1].clone();
    let s = vs[2].as_stream()?;

    Ok(Datum::Stream(eval.get_unique_id(), Box::new(Rc::new(RefCell::new(AccumulateStream::new(*s, fun, acc))))))
}

fn enumerate(vs: &mut [Datum], eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let s = vs[0].as_stream()?;
    Ok(Datum::Stream(eval.get_unique_id(), Box::new(Rc::new(RefCell::new(EnumerateStream::new(*s))))))
}

fn select(vs: &mut [Datum], eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let fun = vs[0].clone();
    let s = vs[1].as_stream()?;

    Ok(Datum::Stream(eval.get_unique_id(), Box::new(Rc::new(RefCell::new(SelectStream::new(*s, fun))))))
}

fn collect(vs: &mut [Datum], eval: &mut Evaluator, env_ref: EnvRef) -> LispResult {
    let mut arg = vs.get(0).unwrap().clone();
    if let Datum::Stream(_id, ref mut s) = arg {
        let mut res = Vec::new();
        let mut source = s.borrow_mut();

        loop {
            let next = source.next(eval, env_ref.clone());
            match next {
                Some(v) => res.push(v),
                None => break
            }
        }

        Ok(Datum::make_list_from_vec(res))
    } else {
        Err(InvalidTypeOfArguments)
    }
}

fn take(vs: &mut [Datum], eval: &mut Evaluator, env_ref: EnvRef) -> LispResult {
    let mut n = vs[0].as_integer()?;
    let mut arg = vs.get(1).unwrap().clone();

    if let Datum::Stream(_id, ref mut s) = arg {
        let mut res = Vec::new();
        let mut source = s.borrow_mut();

        loop {
            if n == 0 {
                break;
            }
            let next = source.next(eval, env_ref.clone());
            match next {
                Some(v) => {
                    n -= 1;
                    res.push(v)
                },
                None => break
            }
        }

        Ok(Datum::make_list_from_vec(res))
    } else {
        Err(InvalidTypeOfArguments)
    }
}

fn nth(vs: &mut [Datum], eval: &mut Evaluator, env_ref: EnvRef) -> LispResult {
    let mut n = vs[0].as_integer()?;

    if let Datum::Stream(_id, ref mut s) = vs[1].clone() {
        let mut source = s.borrow_mut();
        loop {
            let next = source.next(eval, env_ref.clone());
            match next {
                Some(v) => {
                    if n == 0 {
                        return Ok(v)
                    } else {
                        n -= 1;
                    }
                }
                None => return Ok(Datum::Nil)
            }
        }
    }

    Ok(Datum::Nil)
}

fn reduce(vs: &mut [Datum], eval: &mut Evaluator, env_ref: EnvRef) -> LispResult {
    let fun = vs[0].clone();
    let mut res = vs[1].clone();

    if let Datum::Stream(_id, ref mut s) = vs[2].clone() {
        let mut source = s.borrow_mut();
        loop {
            let next = source.next(eval, env_ref.clone());
            match next {
                Some(v) => {
                    res = eval.full_apply(fun.clone(), vec![v, res], env_ref.clone())
                },
                None => break
            }
        }

        Ok(res)
    } else {
        Err(InvalidTypeOfArguments)
    }
}

fn count(vs: &mut [Datum], eval: &mut Evaluator, env_ref: EnvRef) -> LispResult {
    let fun = vs[0].clone();
    let mut res = 0;

    if let Datum::Stream(_id, ref mut s) = vs[1].clone() {
        loop {
            let next = (*s).borrow_mut().next(eval, env_ref.clone());
            match next {
                Some(v) => {
                    let ret = eval.full_apply(fun.clone(), vec![v], env_ref.clone());
                    if ret.is_true() {
                        res += 1;
                    }
                },
                None => break
            }
        }

        Ok(Datum::Integer(res))
    } else {
        Err(InvalidTypeOfArguments)
    }
}

fn clone(vs: &mut [Datum], eval: &mut Evaluator, env_ref: EnvRef) -> LispResult {
    let len = vs[0].clone().as_integer()?;
    if let Datum::Stream(_id, ref mut s) = vs[1].clone() {
        let mut res = Vec::new();
        for _ in 0..len {
            let next = (*s).borrow_mut().next(eval, env_ref.clone());
            match next {
                Some(v) => res.push(v),
                None => break,
            };
        }
        Ok(Datum::make_list_from_vec(res))
    } else {
        Err(InvalidTypeOfArguments)
    }
}

pub fn load(hm: &mut HashMap<String, LispFn>) {
    register(hm, "range~", range, Arity::Range(2, 3));
    register(hm, "step~", step, Arity::Range(0, 2));
    register(hm, "permutations~", permutations, Arity::Exact(1));
    register(hm, "combinations~", combinations, Arity::Exact(2));
    register(hm, "map~", map, Arity::Exact(2));
    register(hm, "flatmap-list~", flatmap_list, Arity::Exact(2));
    register(hm, "flatmap-stream~", flatmap_stream, Arity::Exact(2));
    register(hm, "accumulate~", accumulate, Arity::Exact(3));
    register(hm, "enumerate~", enumerate, Arity::Exact(1));
    register(hm, "nth~", nth, Arity::Exact(2));
    register(hm, "select~", select, Arity::Exact(2));
    register(hm, "count~", count, Arity::Exact(2));
    register(hm, "reduce~", reduce, Arity::Exact(3));
    register(hm, "clone~", clone, Arity::Exact(2));
    register(hm, "collect", collect, Arity::Exact(1));
    register(hm, "take~", take, Arity::Exact(2));
}
