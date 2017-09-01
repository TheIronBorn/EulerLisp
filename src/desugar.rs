use ::Value;

pub fn desugar(v: &Value) -> Value {
    match *v {
        Value::List(ref elems) => {
            if elems.len() >= 1 {
                match elems[0].clone() {
                    Value::Atom(s) => {
                        match s.as_ref() {
                            "defn" => desugar_defn(&elems[1..]),
                            "fn" => desugar_fn(&elems[1..]),
                            "stream-cons" => desugar_stream_cons(&elems[1..]),
                            _ => {
                                Value::List(elems.iter().map(|e| desugar(e)).collect())
                            },
                        }
                    },
                    _ => Value::List(elems.iter().map(|e| desugar(e)).collect()),
                }
            } else {
                v.clone()
            }
        },
        ref other => other.clone()
    }
}

// (defn name args body) -> (def name (fn args body))
fn desugar_defn(args: &[Value]) -> Value {
    let usage = "Usage: (defn name (args...) body)";
    let name = args.get(0).expect(usage);

    Value::List(vec![
        Value::Atom("def".to_string()),
        desugar(name),
        desugar_fn(&args[1..])
    ])
}

// (fn args body*) -> (fn args (do *body))
fn desugar_fn(args: &[Value]) -> Value {
    let usage = "Usage: (defn name (args...) body)";
    let params = args.get(0).expect(usage);

    let mut new_body = vec![Value::Atom("do".to_string())];
    new_body.extend(args[1..].iter().map(|v| desugar(v)));

    Value::List(vec![
        Value::Atom("fn".to_string()),
        params.clone(),
        Value::List(new_body)
    ])
}

// (cons-stream head tail) -> (fn args (do *body))
fn desugar_stream_cons(args: &[Value]) -> Value {
    let usage = "Usage: (stream-cons fst rst)";
    let fst = args.get(0).expect(usage);
    let rst = args.get(1).expect(usage);

    Value::List(vec![
        Value::Atom("cons".to_string()),
        desugar(fst),
        Value::List(vec![
            Value::Atom("delay".to_string()),
            desugar(rst)
        ])
    ])
}
