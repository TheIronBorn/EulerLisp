use std::collections::HashMap;
use ::Datum;

// TODO: Implement SyntaxRuleErrors instead of using unwrap() everywhere
// Based on R5RS, Section 4.2.3
#[derive(Debug, Clone)]
pub struct SyntaxRule {
    name: String,
    literals: Vec<String>,
    rules: Vec<Rule>
}

#[derive(Debug, Clone)]
pub struct Rule(Pattern, Template);

impl Rule {
    pub fn parse(datum: Datum) -> Rule {
        let rule = datum.as_list().unwrap();

        let pattern = Pattern::parse(rule[0].clone());
        let template = Template::parse(rule[1].clone());

        Rule(pattern, template)
    }
}

impl SyntaxRule {
    pub fn parse(name: String, literals: Vec<Datum>, rules: Vec<Datum>) -> SyntaxRule {
            let literals = literals.iter().map( |l| l.as_symbol().unwrap() ).collect();
            let rules = rules.into_iter().map( |r| Rule::parse(r) ).collect();

        SyntaxRule {
            name: name,
            literals: literals,
            rules: rules
        }
    }

    pub fn apply(&self, mut datums: Vec<Datum>) -> Datum {
        // Inside the preprocessing step,
        // each function application only has access to its arguments,
        // not its own name,
        // to have the patterns work (somewhat) like specified in R5RS,
        // we need to put it in front again
        datums.insert(0, Datum::Symbol(self.name.clone()));
        let datum = Datum::List(datums);

        for rule in self.rules.iter() {
            let &Rule(ref pattern, ref template) = rule;

            let mut bindings: HashMap<String, Datum> = HashMap::new();
            if self.matches(pattern, datum.clone(), &mut bindings) {
                return template.apply(&mut bindings);
            }
        }
        panic!("No matching pattern for {:?} in {:?}", datum, self);
    }

    pub fn matches(&self, pattern: &Pattern, datum: Datum, bindings: &mut HashMap<String, Datum>) -> bool {
        match *pattern {
            Pattern::Ident(ref name) => {
                if let Datum::Symbol(s) = datum.clone() {
                    if s == self.name || self.literals.contains(&s) {
                        true
                    } else {
                        bindings.insert(name.clone(), datum.clone());
                        true
                    }
                } else {
                    bindings.insert(name.clone(), datum.clone());
                    true
                }
            },
            Pattern::List(ref patterns) => {
                if let Datum::List(s) = datum {
                    if s.len() != patterns.len() {
                        return false;
                    }

                    for (s, p) in s.into_iter().zip(patterns.iter()) {
                        if !self.matches(p, s, bindings) {
                            return false;
                        }
                    }
                    return true;
                } else {
                    false
                }
            },
            Pattern::ListWithRest(ref patterns, ref rest) => {
                if let Datum::List(mut s) = datum {
                    if s.len() < patterns.len() {
                        return false;
                    }

                    let remaining = s.split_off(patterns.len());
                    for (s, p) in s.into_iter().zip(patterns.iter()) {
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

                    if subbindings.len() == 0 {
                        return true;
                    }

                    let keys = subbindings[0].keys();

                    for k in keys {
                        let mut coll = Vec::new();
                        for subbinding in subbindings.iter() {
                            coll.push(subbinding.get(k).unwrap().clone());
                        }
                        bindings.insert(k.clone(), Datum::List(coll));
                    }

                    return true;
                } else {
                    false
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Ident(String),
    // (<pattern> ...)
    List(Vec<Pattern>),
    // TODO: (<pattern> <pattern> ... . <pattern>)
    // (<pattern> ... <pattern> <ellipsis>)
    ListWithRest(Vec<Pattern>, Box<Pattern>)
}

fn is_ellipsis(datum: Datum) -> bool {
    if let Datum::Symbol(s) = datum {
        s == "..."
    } else {
        false
    }
}

impl Pattern {
    pub fn parse(datum: Datum) -> Pattern {
        match datum {
            Datum::List(mut s) => {
                if s.len() == 0 {
                    panic!("Empty macro pattern");
                } 

                let last = s.get(s.len() - 1).unwrap().clone();
                if is_ellipsis(last) {
                    s.pop();
                    let rest = Pattern::parse(s.pop().unwrap());
                    Pattern::ListWithRest(
                        s.into_iter().map( |d| Pattern::parse(d) ).collect(),
                        Box::new(rest)
                    )
                } else {
                    Pattern::List(
                        s.into_iter().map( |d| Pattern::parse(d) ).collect()
                    )
                }

            },
            Datum::Symbol(s) => {
                Pattern::Ident(s)
            },
            other => {
                panic!("Invalid macro pattern: {:?}", other);
            }
        }
    }
}

// TODO: Find out what elements are used for
#[derive(Debug, Clone)]
pub enum Template {
    Ident(String),
    Constant(Datum),
    // (<element> ...)
    List(Vec<Element>),
    // // (<element> <element> ... . <template>)
    // Dotted(Vec<Element>, Box<Template>)
}

impl Template {
    pub fn parse(datum: Datum) -> Template {
        match datum {
            Datum::Symbol(s) => Template::Ident(s),
            Datum::List(mut elems) => {
                let mut res = Vec::new();
                loop {
                    if elems.len() == 0 {
                        break;
                    }

                    let t = Template::parse(elems.remove(0));

                    if elems.len() == 0 {
                        res.push(Element::Normal(t));
                    } else {
                        // "peek" next element
                        let n = elems.remove(0);

                        if is_ellipsis(n.clone()) {
                            res.push(Element::Ellipsed(t));
                        } else {
                            elems.insert(0, n);
                            res.push(Element::Normal(t));
                        }
                    }
                }
                Template::List(res)
            },
            other => Template::Constant(other)
        }
    }

    pub fn apply(&self, bindings: &HashMap<String, Datum>) -> Datum {
        match *self {
            Template::Ident(ref n) => {
                let binding = bindings.get(n);
                if let Some(d) = binding {
                    d.clone()
                } else {
                    Datum::Symbol(n.clone())
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

                            if let Datum::List(mut inner) = foo {
                                res.append(&mut inner);
                            } else {
                                println!("{:?}", self);
                                panic!("macro templates `<identifier> ...` only work if binding is list");
                            }
                        }
                    }
                }

                Datum::List(res)
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
