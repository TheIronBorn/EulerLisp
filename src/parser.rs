use ::Value;

use ::std::fs::File;
use ::std::str::FromStr;
use ::std::str;

use ::nom::*;

named!(
    atom<&[u8], String>,
    do_parse!(
        word: is_a_s!("abcdefghijklmnopqrstuvwxyz=?+-/*><") >>
        (String::from_utf8(word.to_vec()).unwrap())
    )
);

named!(
    number<&[u8], i64>,
    map_res!(
        map_res!(
            digit,
            ::std::str::from_utf8
        ),
        (::std::str::FromStr::from_str)
    )
);

named!(list<&[u8], Vec<Value>>,
    do_parse!(
        tag!("(") >>
        elements: ws!(many0!(value)) >>
        tag!(")") >>
        (elements)
    )
);

named!(boolean<&[u8], bool>,
    alt!(
        map!(tag!("true"), |_| true) |
        map!(tag!("false"), |_| false)
    )
);

named!(value<&[u8], Value>,
    alt!(
      map!(ws!(boolean), |x| Value::Bool(x)) |
      map!(ws!(list), |x| Value::List(x)) |
      map!(ws!(number), |x| Value::Number(x)) |
      map!(ws!(tag!("'()")), |x| Value::Nil) |
      map!(ws!(atom), |x| Value::Atom(x))
    )
);

pub fn parse(s: &String) -> Value {
    match value(s.as_bytes()) {
      IResult::Done(_, v) => v,
      _ => panic!("Failed to parse value")
    }
}

#[test]
fn string_parser() {
    let comp_string = String::from("hi");

    match atom(b"hi") {
        IResult::Done(_, s) => assert_eq!(comp_string, s),
        _ => panic!("Failed to parse string")
    }

    match atom(b"   hi  ") {
        IResult::Done(_, s) => assert_eq!(comp_string, s),
        _ => panic!("Failed to parse string")
    }
}

#[test]
fn op_parser() {
    match op(b"atomop") {
        IResult::Done(_, s) => assert_eq!(Op::Atom(String::from("atomop")), s),
        _ => panic!("Failed to parse string")
    }

    match op(b"  +  ") {
        IResult::Done(_, s) => assert_eq!(Op::Primitive(Prim::Add), s),
        _ => panic!("Failed to parse string")
    }

    match op(b"  let ") {
        IResult::Done(_, s) => assert_eq!(Op::SpecialFrom(SForm::Let), s),
        _ => panic!("Failed to parse string")
    }
}

