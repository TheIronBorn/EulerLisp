use ::Datum;

pub fn desugar(v: &Datum) -> Datum {
    match *v {
        Datum::List(ref elems) => {
            if elems.len() >= 1 {
                match elems[0].clone() {
                    Datum::Symbol(s) => {
                        match s.as_ref() {
                            "defn" => desugar_defn(&elems[1..]),
                            "fn" => desugar_fn(&elems[1..]),
                            "stream-cons" => desugar_stream_cons(&elems[1..]),
                            _ => {
                                Datum::List(elems.iter().map(|e| desugar(e)).collect())
                            },
                        }
                    },
                    _ => Datum::List(elems.iter().map(|e| desugar(e)).collect()),
                }
            } else {
                v.clone()
            }
        },
        ref other => other.clone()
    }
}

// (defn name args body) -> (def name (fn args body))
fn desugar_defn(args: &[Datum]) -> Datum {
    let usage = "Usage: (defn name (args...) body)";
    let name = args.get(0).expect(usage);

    Datum::List(vec![
        Datum::Symbol("def".to_string()),
        desugar(name),
        desugar_fn(&args[1..])
    ])
}

// (fn args body*) -> (fn args (do *body))
fn desugar_fn(args: &[Datum]) -> Datum {
    let usage = "Usage: (defn name (args...) body)";
    let params = args.get(0).expect(usage);

    let mut new_body = vec![Datum::Symbol("do".to_string())];
    new_body.extend(args[1..].iter().map(|v| desugar(v)));

    Datum::List(vec![
        Datum::Symbol("fn".to_string()),
        params.clone(),
        Datum::List(new_body)
    ])
}

// (cons-stream head tail) -> (fn args (do *body))
fn desugar_stream_cons(args: &[Datum]) -> Datum {
    let usage = "Usage: (stream-cons fst rst)";
    let fst = args.get(0).expect(usage);
    let rst = args.get(1).expect(usage);

    Datum::List(vec![
        Datum::Symbol("cons".to_string()),
        desugar(fst),
        Datum::List(vec![
            Datum::Symbol("delay".to_string()),
            desugar(rst)
        ])
    ])
}
