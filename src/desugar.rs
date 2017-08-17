use ::Value;

pub fn desugar(v: &Value) -> Value {
    match *v {
        Value::List(ref elems) => {
            if elems.len() >= 1 {
                match elems[0].clone() {
                    Value::Atom(s) => {
                        match s.as_ref() {
                            "defn" => desugar_defn(&elems[1..]),
                            _ => v.clone()
                            // TODO: desugar the whole list
                        }
                    },
                    // TODO: desugar the whole list
                    _ => v.clone()
                }
            } else {
                v.clone()
            }
        },
        ref other => other.clone()
    }
}

fn desugar_defn(args: &[Value]) -> Value {
    let usage = "Usage: (defn name (args...) body)";
    let name = args.get(0).expect(usage);
    let fn_args = args.get(1).expect(usage);
    let body = args.get(2).expect(usage);

    Value::List(vec![
        Value::Atom("def".to_string()),
        desugar(name),
        Value::List(vec![
            Value::Atom("fn".to_string()),
            desugar(fn_args),
            desugar(body),
        ]),
    ])
}
