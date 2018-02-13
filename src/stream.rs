use std::fmt;

use ::Datum;
use ::Stream;
use eval;
use env::EnvRef;

pub trait LispIterator {
    fn next(&mut self, eval: &mut eval::Evaluator, env_ref: EnvRef) -> Option<Datum>;
}

impl fmt::Debug for LispIterator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stream")
    }
}

impl PartialEq for LispIterator {
    fn eq(&self, _other: &LispIterator) -> bool {
        false
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct RangeStream {
    from: isize,
    to: isize,
    step: isize,
    current: isize
}

#[derive(PartialEq, Clone, Debug)]
pub struct StepStream {
    from: isize,
    step: isize,
    current: isize
}

impl StepStream {
    pub fn new(from: isize, step: isize) -> StepStream {
        StepStream { from: from, step: step, current: from }
    }
}

impl LispIterator for StepStream {
    fn next(&mut self, _eval: &mut eval::Evaluator, _env_ref: EnvRef) -> Option<Datum> {
        let ret = self.current;
        self.current += self.step;
        Some(Datum::Integer(ret))
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct MapStream {
    source: Box<Stream>,
    fun: Box<Datum>,
}

impl MapStream {
    pub fn new(source: Stream, fun: Datum) -> Self {
        Self { source: Box::new(source), fun: Box::new(fun) }
    }
}

impl LispIterator for MapStream {
    fn next(&mut self, eval: &mut eval::Evaluator, env_ref: EnvRef) -> Option<Datum> {
        let next = self.source.borrow_mut().next(eval, env_ref.clone());

        match next {
            Some(v) => Some(eval.full_apply((*self.fun).clone(), vec![v], env_ref)),
            None => None
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct EnumerateStream {
    source: Box<Stream>,
    index: isize,
}

impl EnumerateStream {
    pub fn new(source: Stream) -> Self {
        Self { source: Box::new(source), index: -1 }
    }
}

impl LispIterator for EnumerateStream {
    fn next(&mut self, eval: &mut eval::Evaluator, env_ref: EnvRef) -> Option<Datum> {
        let next = self.source.borrow_mut().next(eval, env_ref.clone());
        self.index += 1;
        match next {
            Some(v) => Some(Datum::make_pair(v, Datum::Integer(self.index))),
            None => None
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct AccumulateStream {
    source: Box<Stream>,
    fun: Box<Datum>,
    acc: Box<Datum>
}

impl AccumulateStream {
    pub fn new(source: Stream, fun: Datum, acc: Datum) -> Self {
        Self {
            source: Box::new(source),
            fun: Box::new(fun),
            acc: Box::new(acc)
        }
    }
}

impl LispIterator for AccumulateStream {
    fn next(&mut self, eval: &mut eval::Evaluator, env_ref: EnvRef) -> Option<Datum> {
        let next = self.source.borrow_mut().next(eval, env_ref.clone());

        match next {
            Some(v) => {
                let new_acc = eval.full_apply(
                    (*self.fun).clone(),
                    vec![v, *self.acc.clone()],
                    env_ref
                );
                self.acc = Box::new(new_acc.clone());
                Some(new_acc)
            }
            None => None
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct FlatMapStream {
    source: Box<Stream>,
    fun: Box<Datum>,
    stack: Vec<Datum>
}

impl FlatMapStream {
    pub fn new(source: Stream, fun: Datum) -> Self {
        Self { source: Box::new(source), fun: Box::new(fun), stack: Vec::new() }
    }
}

impl LispIterator for FlatMapStream {
    fn next(&mut self, eval: &mut eval::Evaluator, env_ref: EnvRef) -> Option<Datum> {
        if self.stack.len() == 0 {
            let next = self.source.borrow_mut().next(eval, env_ref.clone());
            match next {
                Some(v) => {
                    self.stack = eval.full_apply((*self.fun).clone(), vec![v], env_ref).as_list().unwrap();
                    Some(self.stack.remove(0))
                }
                None => None
            }
        } else {
            Some(self.stack.remove(0))
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct SelectStream {
    source: Box<Stream>,
    fun: Box<Datum>,
}

impl SelectStream {
    pub fn new(source: Stream, fun: Datum) -> Self {
        Self { source: Box::new(source), fun: Box::new(fun) }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct PermutationStream {
    array: Vec<Datum>,
    next: Vec<Datum>,
    i: usize,
    n: usize,
    c: Vec<usize>
}

#[derive(PartialEq, Clone, Debug)]
pub struct CombinationStream {
    array: Vec<Datum>,
    indices: Vec<usize>,
    n: usize,
    size: usize,
    done: bool,
}

impl PermutationStream {
    pub fn new(array: Vec<Datum>) -> Self {
        let n = array.len();
        Self {
            array: array.clone(),
            next: array,
            i: 0,
            n: n,
            c: vec![0; n]
        }
    }
}

impl CombinationStream {
    pub fn new(array: Vec<Datum>, size: usize) -> Self {
        let n = array.len();
        Self {
            array: array.clone(),
            indices: vec![0; size],
            n: n,
            size: size,
            done: false
        }
    }

    pub fn step(&mut self) {
        let mut carry = 1;
        for i in 0..self.size {
            let next = carry + self.indices[i];
            if next >= self.n {
                self.indices[i] = 0;
            } else {
                self.indices[i] = next;
                carry = 0;
            }
        }

        self.done = carry != 0;
    }
}

impl LispIterator for CombinationStream {
    fn next(&mut self, _eval: &mut eval::Evaluator, _env_ref: EnvRef) -> Option<Datum> {
        if self.done {
            None
        } else {
            let ret = self.indices.clone().into_iter().map(|i| self.array[i].clone()).collect();
            self.step();
            Some(Datum::make_list_from_vec(ret))
        }
    }
}

impl LispIterator for PermutationStream {
    fn next(&mut self, _eval: &mut eval::Evaluator, _env_ref: EnvRef) -> Option<Datum> {
        if self.i == self.n {
            None
        } else {
            let ret = self.next.clone();

            while self.i < self.n {
                if self.c[self.i] < self.i {
                    if self.i % 2 == 0 {
                        self.array.swap(0, self.i);
                    } else {
                        self.array.swap(self.c[self.i], self.i);
                    }
                    self.next = self.array.clone();
                    self.c[self.i] += 1;
                    self.i = 0;
                    break
                } else {
                    self.c[self.i] = 0;
                    self.i += 1;
                }
            }

            Some(Datum::make_list_from_vec(ret))
        }
    }
}


impl RangeStream {
    pub fn new(from: isize, to: isize, step: isize) -> RangeStream {
        RangeStream {
            from: from,
            to: to,
            step: step,
            current: from
        }
    }
}

impl LispIterator for RangeStream {
    fn next(&mut self, _eval: &mut eval::Evaluator, _env_ref: EnvRef) -> Option<Datum> {
        if self.current > self.to {
            None
        } else {
            let ret = self.current;
            self.current += self.step;
            Some(Datum::Integer(ret))
        }
    }
}

impl LispIterator for SelectStream {
    fn next(&mut self, eval: &mut eval::Evaluator, env_ref: EnvRef) -> Option<Datum> {
        loop {
            let next = self.source.borrow_mut().next(eval, env_ref.clone());
            match next {
                Some(v) => {
                    if let Datum::Bool(true) = eval.full_apply((*self.fun).clone(), vec![v.clone()], env_ref.clone()) {
                        return Some(v);
                    }
                },
                None => return None
            }
        }
    }
}
