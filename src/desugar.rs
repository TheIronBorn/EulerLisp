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
                            "let" => desugar_let(&elems[1..]),
                            "let*" => desugar_let_star(&elems[1..]),
                            // "and" => desugar_and(&elems[1..]),
                            // "or" => desugar_or(&elems[1..]),
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

fn desugar_let(args: &[Datum]) -> Datum {
    let usage = "Usage: (let (bindings...) body)";
    let bindings = args.get(0).expect(usage);

    let mut new_body = vec![Datum::Symbol("do".to_string())];
    new_body.extend(args[1..].iter().map(|v| desugar(v)));

    let mut variables: Vec<Datum> = Vec::new();
    let mut values: Vec<Datum> = Vec::new();

    if let Datum::List(ref elements) = *bindings {
        for binding in elements.iter() {
            if let Datum::List(ref varval) = *binding {
                variables.push(varval.get(0).expect("No binding variable").clone());
                values.push(varval.get(1).expect("No binding value").clone());
            } else {
                panic!("Each let binding must be a list")
            }
        }
    } else {
        panic!("First argument of let must be a list")
    }

    let mut result: Vec<Datum> = Vec::new();

    let function = Datum::List(vec![
        Datum::Symbol("fn".to_string()),
        Datum::List(variables),
        Datum::List(new_body)
    ]);

    result.push(function);
    result.append(&mut values.clone());

    desugar(&Datum::List(result))
}

// A version of `let` that allows bindings
// to reference the variables of earlier bindings
fn desugar_let_star(args: &[Datum]) -> Datum {
    let usage = "Usage: (let* (bindings...) body)";
    let bindings = args.get(0).expect(usage);

    let mut new_body = vec![Datum::Symbol("do".to_string())];
    new_body.extend(args[1..].iter().map(|v| desugar(v)));

    let mut variables: Vec<Datum> = Vec::new();
    let mut values: Vec<Datum> = Vec::new();

    if let Datum::List(ref elements) = *bindings {
        for binding in elements.iter() {
            if let Datum::List(ref varval) = *binding {
                variables.push(varval.get(0).expect("No binding variable").clone());
                values.push(varval.get(1).expect("No binding value").clone());
            } else {
                panic!("Each let binding must be a list")
            }
        }
    } else {
        panic!("First argument of let must be a list")
    }

    let mut result: Datum = Datum::List(new_body);
    for (key, value) in variables.into_iter().zip(values.into_iter()).rev() {
        let binding = Datum::List(vec![key, value]);
        let new = Datum::List(vec![
            Datum::Symbol("let".to_string()),
            Datum::List(vec![binding]),
            result
        ]);
        result = new;
    }

    desugar(&result)
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
