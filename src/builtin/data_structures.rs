use std::collections::HashMap;
use std::cmp::Ordering;

use ::Datum;
use ::LispFn;
use ::LispErr::*;
use ::LispResult;
use ::Arity;

use ::builtin::register;
use ::eval::Evaluator;
use ::EnvRef;

use ::data_structures::priority_queue;

fn make_pq(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let content = vs[0].as_pair()?;
    let kv_pairs = content.collect_list()?;

    let mut elements = Vec::new();
    let mut priorities = Vec::new();
    for kv_pair in kv_pairs {
        let kv = kv_pair.as_pair()?;
        elements.push(kv.0.clone());
        priorities.push(kv.1.clone());
    }

    let pq = priority_queue::PriorityQueue::new(elements, priorities, Ordering::Greater);
    Ok(Datum::make_priority_queue(pq))
}

fn make_min_pq(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let content = vs[0].as_pair()?;
    let kv_pairs = content.collect_list()?;

    let mut elements = Vec::new();
    let mut priorities = Vec::new();
    for kv_pair in kv_pairs {
        let kv = kv_pair.as_pair()?;
        elements.push(kv.0.clone());
        priorities.push(kv.1.clone());
    }

    let pq = priority_queue::PriorityQueue::new(elements, priorities, Ordering::Less);
    Ok(Datum::make_priority_queue(pq))
}

fn pq_maximum(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let pq = vs[0].as_priority_queue()?;
    Ok(pq.maximum())
}

fn pq_pop(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let mut pq = vs[0].as_mut_priority_queue()?;
    Ok(pq.pop())
}

fn pq_insert(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let mut pq = vs[0].as_mut_priority_queue()?;
    let element = vs[1].clone();
    let priority = vs[2].clone();
    pq.insert(element, priority);

    Ok(Datum::Undefined)
}


pub fn load(hm: &mut HashMap<String, LispFn>) {
    register(hm, "make-priority-queue", make_pq, Arity::Exact(1));
    register(hm, "make-min-priority-queue", make_min_pq, Arity::Exact(1));
    register(hm, "priority-queue-max", pq_maximum, Arity::Exact(1));
    register(hm, "priority-queue-pop!", pq_pop, Arity::Exact(1));
    register(hm, "priority-queue-insert!", pq_insert, Arity::Exact(3));
}
