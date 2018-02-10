use std::collections::HashMap;
use ::Datum;
use ::Symbol;
use symbol_table::SymbolTable;

// Based on R5RS, Section 4.2.3
//
// TODO: Implement SyntaxRuleErrors instead of using unwrap() everywhere
// TODO: Templates like `(name val) ...` don't seem to work
#[derive(Debug, Clone)]
pub struct SyntaxRule {
    name: Symbol,
    literals: Vec<Symbol>,
    rules: Vec<Rule>
}

#[derive(Debug, Clone)]
pub struct Rule(Pattern, Template);

impl Rule {
    pub fn parse(expr: Datum, symbol_table: &SymbolTable) -> Rule {
        let rule = expr.as_list().unwrap();
        let pattern = Pattern::parse(rule[0].clone(), symbol_table);
        let template = Template::parse(rule[1].clone(), symbol_table);

        Rule(pattern, template)
    }
}

impl SyntaxRule {
    pub fn parse(name: Symbol, literals: Vec<Datum>, rules: Vec<Datum>, symbol_table: &SymbolTable) -> SyntaxRule {
            let literals = literals.iter().map( |l| l.as_symbol().unwrap() ).collect();
            let rules = rules.into_iter().map( |r| Rule::parse(r, symbol_table) ).collect();

        SyntaxRule {
            name: name,
            literals: literals,
            rules: rules
        }
    }

    pub fn apply(&self, mut datums: Vec<Datum>) -> Option<Datum> {
        // Inside the preprocessing step,
        // each function application only has access to its arguments,
        // not its own name,
        // to have the patterns work (somewhat) like specified in R5RS,
        // we need to put it in front again
        datums.insert(0, Datum::Symbol(self.name));
        let datum = Datum::make_list_from_vec(datums);

        for rule in self.rules.iter() {
            let &Rule(ref pattern, ref template) = rule;

            let mut bindings: HashMap<Symbol, Datum> = HashMap::new();
            if self.matches(pattern, datum.clone(), &mut bindings) {
                return Some(template.apply(&mut bindings));
            }
        }
        None
    }

    pub fn matches(&self, pattern: &Pattern, datum: Datum, bindings: &mut HashMap<Symbol, Datum>) -> bool {
        match *pattern {
            Pattern::Ident(name) => {
                if let Datum::Symbol(s) = datum.clone() {
                    if s == self.name || self.literals.contains(&s) {
                        true
                    } else {
                        bindings.insert(name, datum.clone());
                        true
                    }
                } else {
                    bindings.insert(name, datum.clone());
                    true
                }
            },
            Pattern::List(ref patterns) => {
                match datum {
                    Datum::Pair(ptr) => {
                        // TODO: Handle errors
                        let elems = ptr.borrow().collect_list().unwrap();

                        if elems.len() != patterns.len() {
                            return false;
                        }

                        for (s, p) in elems.into_iter().zip(patterns.iter()) {
                            if !self.matches(p, s, bindings) {
                                return false;
                            }
                        }
                        true
                    },
                    Datum::Nil => patterns.len() == 0,
                    _ => false,
                }
            },
            Pattern::ListWithRest(ref patterns, ref rest) => {
                match datum {
                    Datum::Pair(ptr) => {
                        let mut elems = ptr.borrow().collect_list().unwrap();

                        if elems.len() < patterns.len() {
                            return false;
                        }

                        let remaining = elems.split_off(patterns.len());
                        for (s, p) in elems.into_iter().zip(patterns.iter()) {
                            if !self.matches(p, s, bindings) {
                                return false;
                            }
                        }

                        let mut subbindings = Vec::new();
                        for s in remaining.into_iter() {
                            let mut b = HashMap::new();
                            if !self.matches(rest, s, &mut b) {
                                return false;
                            }
                            subbindings.push(b);
                        }

                        let keys = rest.keys();
                        for k in keys {
                            let mut coll = Vec::new();
                            for subbinding in subbindings.iter() {
                                coll.push(subbinding.get(&k).unwrap().clone());
                            }
                            bindings.insert(k.clone(), Datum::make_list_from_vec(coll));
                        }

                        true
                    },
                    // Datum::Nil => patterns.len() == 0,
                    _ => false
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Ident(Symbol),
    // (<pattern> ...)
    List(Vec<Pattern>),
    // TODO: (<pattern> <pattern> ... . <pattern>)
    // (<pattern> ... <pattern> <ellipsis>)
    ListWithRest(Vec<Pattern>, Box<Pattern>)
}

fn is_ellipsis(datum: Datum, symbol_table: &SymbolTable) -> bool {
    if let Datum::Symbol(s) = datum {
        symbol_table.lookup(s) == "..."
    } else {
        false
    }
}

impl Pattern {
    pub fn parse(expr: Datum, symbol_table: &SymbolTable) -> Pattern {
        match expr {
            Datum::Pair(ptr) => {
                let mut elems = ptr.borrow().collect_list().unwrap();

                let last = elems.get(elems.len() - 1).unwrap().clone();
                if is_ellipsis(last, symbol_table) {
                    elems.pop();
                    let rest = Pattern::parse(elems.pop().unwrap(), symbol_table);
                    Pattern::ListWithRest(
                        elems.into_iter().map( |d| Pattern::parse(d, symbol_table) ).collect(),
                        Box::new(rest)
                    )
                } else {
                    Pattern::List(
                        elems.into_iter().map( |d| Pattern::parse(d, symbol_table) ).collect()
                    )
                }

            },
            Datum::Nil => Pattern::List(vec!()),
            Datum::Symbol(s) => {
                Pattern::Ident(s)
            },
            other => {
                panic!("Invalid macro pattern: {:?}", other);
            }
        }
    }

    pub fn keys(&self) -> Vec<Symbol> {
        match self {
            &Pattern::List(ref elems) => {
                let mut res = Vec::new();
                for e in elems {
                    let mut k = e.keys(); 
                    res.append(&mut k);
                }
                res
            },
            &Pattern::ListWithRest(ref elems, ref rest) => {
                let mut res = Vec::new();
                for e in elems {
                    let mut k = e.keys(); 
                    res.append(&mut k);
                }
                let mut k = rest.keys(); 
                res.append(&mut k);
                res
            },
            &Pattern::Ident(ref key) => {
                vec![key.clone()]
            }
        }
    }
}

// TODO: Find out what elements are used for
#[derive(Debug, Clone)]
pub enum Template {
    Ident(Symbol),
    Constant(Datum),
    // (<element> ...)
    List(Vec<Element>),
    // // (<element> <element> ... . <template>)
    // Dotted(Vec<Element>, Box<Template>)
}

impl Template {
    pub fn parse(datum: Datum, symbol_table: &SymbolTable) -> Template {
        match datum {
            Datum::Symbol(s) => Template::Ident(s),
            Datum::Pair(ptr) => {
                let mut elems = ptr.borrow().collect_list().unwrap();
                let mut res = Vec::new();
                loop {
                    if elems.len() == 0 {
                        break;
                    }

                    let t = Template::parse(elems.remove(0), symbol_table);

                    if elems.len() == 0 {
                        res.push(Element::Normal(t));
                    } else {
                        // "peek" next element
                        let n = elems.remove(0);

                        if is_ellipsis(n.clone(), symbol_table) {
                            res.push(Element::Ellipsed(t));
                        } else {
                            elems.insert(0, n);
                            res.push(Element::Normal(t));
                        }
                    }
                }
                Template::List(res)
            },
            Datum::Nil => Template::List(vec!()),
            other => Template::Constant(other)
        }
    }

    pub fn apply(&self, bindings: &HashMap<Symbol, Datum>) -> Datum {
        match *self {
            Template::Ident(n) => {
                if let Some(d) = bindings.get(&n) {
                    d.clone()
                } else {
                    Datum::Symbol(n)
                }
            },
            Template::Constant(ref c) => c.clone(),
            Template::List(ref es) => {
                let mut res = Vec::new();

                for e in es.iter() {
                    match *e {
                        Element::Normal(ref t) => {
                            res.push(t.apply(bindings));
                        },
                        Element::Ellipsed(ref t) => {
                            let foo = t.apply(bindings);

                            match foo {
                                Datum::Pair(ptr) => {
                                    let mut inner = ptr.borrow().collect_list().unwrap();
                                    res.append(&mut inner);
                                },
                                Datum::Nil => {
                                    // Do nothing
                                },
                                _ => panic!("macro templates `<identifier> ...` only work if binding is list,
                                            not in {:?}", foo),
                            }
                        }
                    }
                }

                Datum::make_list_from_vec(res)
            }
        }
    }
}

// <element> is a <template> optionally followed
// by an <ellipsis> ("...")
#[derive(Debug, Clone)]
pub enum Element {
    Normal(Template),
    Ellipsed(Template)
}
