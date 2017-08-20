use ::Value;

use ::std::fs::File;
use ::std::str::FromStr;
use ::std::str;

use ::nom::*;

static identifier_start: &'static str = "abcdefghijklmnopqrstuvwxyz!$%&*+-./:<=>?@^_~";
static identifier_main: &'static str = "abcdefghijklmnopqrstuvwxyz0123456789!$%&*+-./:<=>?@^_~";

named!(
    identifier<&[u8], String>,
    do_parse!(
        word: recognize!(
            do_parse!(
                one_of!(identifier_start) >>
                many0!(one_of!(identifier_main)) >>
                ()
            )
        ) >>
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

named!(string<&[u8], String>,
    do_parse!(
        tag!("\"") >>
        body: alphanumeric >>
        tag!("\"") >>
        (String::from_utf8(body.to_vec()).unwrap())
    )
);

named!(quote<&[u8], Vec<Value>>,
    do_parse!(
        tag!("'") >>
        value: value >>
        (vec![Value::Atom(String::from("quote")), value])
    )
);

named!(value<&[u8], Value>,
    alt!(
      map!(ws!(boolean), |x| Value::Bool(x)) |
      map!(ws!(list), |x| Value::List(x)) |
      map!(ws!(number), |x| Value::Number(x)) |
      map!(ws!(tag!("'()")), |x| Value::Nil) |
      map!(ws!(string), |x| Value::Str(x)) |
      map!(ws!(identifier), |x| Value::Atom(x)) |
      map!(ws!(quote), |x| Value::List(x))
    )
);

pub fn parse(s: &String) -> Value {
    match value(s.as_bytes()) {
      IResult::Done(_, v) => v,
      _ => panic!("Failed to parse value")
    }
}
