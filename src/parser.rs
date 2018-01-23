use std::str::from_utf8;

use nom::*;
use nom::{digit, oct_digit, hex_digit};
use nom::{anychar, multispace};
use nom::{not_line_ending, eol};

use ::Datum;

fn is_bin_digit(byte: u8) -> bool {
    byte == b'0' || byte == b'1'
}

named!(bin_digit, take_while1!(is_bin_digit));

named!(sign, recognize!(opt!(one_of!("+-"))));

named!(
    integer_literal2,
    recognize!(do_parse!(sign >> bin_digit >> ()))
);

named!(
    integer_literal8,
    recognize!(do_parse!(sign >> oct_digit >> ()))
);

named!(
    integer_literal10,
    recognize!(do_parse!(sign >> digit >> ()))
);

named!(
    integer_literal16,
    recognize!(do_parse!(sign >> hex_digit >> ()))
);

named!(
    integer2<isize>,
    map_res!(
        map_res!(integer_literal2, from_utf8),
        |s| isize::from_str_radix(s, 2)
    )
);

named!(
    integer8<isize>,
    map_res!(
        map_res!(integer_literal8, from_utf8),
        |s| isize::from_str_radix(s, 8)
    )
);

named!(
    integer10<isize>,
    map_res!(
        map_res!(integer_literal10, from_utf8),
        |s| isize::from_str_radix(s, 10)
    )
);

named!(
    integer16<isize>,
    map_res!(
        map_res!(integer_literal16, from_utf8),
        |s| isize::from_str_radix(s, 16)
    )
);

named!(
    integer<isize>,
    alt!(
        preceded!(tag!("#b"), integer2) |
        preceded!(tag!("#o"), integer8) |
        preceded!(opt!(tag!("#d")), integer10) |
        preceded!(tag!("#x"), integer16)
    )
);

named!(
    boolean<bool>,
    alt!(
        tag!("#t") => { |_| true } |
        tag!("#f") => { |_| false }
    )
);

named!(
    character<char>,
    preceded!(
        tag!("#\\"),
        alt_complete!(
            tag!("space") => { |_| ' ' } |
            tag!("newline") => { |_| '\n' } |
            anychar
        )
    )
);

named!(string<String>,
   alt!(
     tag!("\"\"") => { |_| String::from("")} |
     delimited!(tag!("\""), string_content, tag!("\""))
   )
);

fn to_s(i: Vec<u8>) -> String {
  String::from_utf8_lossy(&i).into_owned()
}

named!(comment,
    // preceded!(tag!(";"), nom::not_line_ending)
    do_parse!(
        tag!(";") >>
        not_line_ending >>
        alt!(eof!() | eol) >>
        (&b""[..])
    )
);

named!(
    intertoken_space,
    recognize!(
        do_parse!(
            many0!(multispace) >>
            many0!(comment) >>
            many0!(multispace) >>
            ()
        )
    )
);


named!(
    string_content<String>,
    map!(
        escaped_transform!(
            take_until_either!("\"\\"),
            '\\',
            alt!(
                tag!("\\") => { |_| &b"\\"[..] } |
                tag!("\"") => { |_| &b"\""[..] } |
                tag!("n") => { |_| &b"\n"[..] } |
                tag!("r") => { |_| &b"\r"[..] } |
                tag!("t") => { |_| &b"\t"[..] }
            )
        ),
        to_s
    )
);

named!(letter<char>, one_of!("abcdefghijklmnopqrstuvwxyz"));
named!(single_digit<char>, one_of!("0123456789"));
named!(special_initial<char>, one_of!("!$%&*/:<=>?^_~-"));
named!(special_subsequent<char>, one_of!("+-.@"));

named!(initial<char>, alt!(letter | special_initial));
named!(subsequent<char>, alt!(initial | single_digit | special_subsequent));

named!(
    common_identifier,
    recognize!(do_parse!(initial >> many0!(subsequent) >> ()))
);

named!(peculiar_identifier, alt!(tag!("+") | tag!("-") | tag!("...")));

named!(
    identifier<String>,
    map!(
        alt_complete!(common_identifier | peculiar_identifier),
        |s| String::from_utf8_lossy(s).into_owned()
    )
);

named!(
    lbracket,
    delimited!(intertoken_space, tag!("("), intertoken_space)
);

named!(
    lbracket2,
    delimited!(intertoken_space, tag!("["), intertoken_space)
);

named!(
    andbracket,
    delimited!(intertoken_space, tag!("&("), intertoken_space)
);

named!(
    rbracket,
    delimited!(intertoken_space, tag!(")"), intertoken_space)
);

named!(
    rbracket2,
    delimited!(intertoken_space, tag!("]"), intertoken_space)
);

named!(
    dot,
    delimited!(intertoken_space, tag!("."), intertoken_space)
);

named!(
    quote<Datum>,
    preceded!(
        delimited!(intertoken_space, tag!("'"), intertoken_space),
        datum
    )
);
named!(
    quasiquote<Datum>,
    preceded!(
        delimited!(intertoken_space, tag!("`"), intertoken_space),
        datum
    )
);
named!(
    unquote<Datum>,
    preceded!(
        delimited!(intertoken_space, tag!(","), intertoken_space),
        datum
    )
);
named!(
    unquote_splicing<Datum>,
    preceded!(
        delimited!(intertoken_space, tag!(",@"), intertoken_space),
        datum
    )
);

named!(
    list<Vec<Datum>>,
    alt!(
        do_parse!(
            lbracket >>
            datums: many0!(datum) >>
            rbracket >>
            (datums)
        ) |
        do_parse!(
            lbracket2 >>
            datums: many0!(datum) >>
            rbracket2 >>
            (datums)
        )
    )
);

named!(
    dotted_list<(Vec<Datum>, Datum)>,
    do_parse!(
        lbracket >>
        datums: many0!(datum) >>
        dot >>
        datum: datum >>
        rbracket >>
        (datums, datum)
    )
);

named!(
    hole_lambda<Vec<Datum>>,
    do_parse!(
        andbracket >>
        datums: many0!(datum) >>
        rbracket >>
        (datums)
    )
);

fn make_symbol(sym: &str) -> Datum {
    Datum::Symbol(String::from(sym))
}

// TODO: Find a better way to handle '() = nil
named!(
    datum<Datum>,
    delimited!(
        intertoken_space,
        alt!(
            tag!("'()") => { |_| Datum::Nil } |
            hole_lambda => { |ds| convert_hole_lambda_to_lambda(ds) } |
            boolean     => { |b| Datum::Bool(b) } |
            integer     => { |n| Datum::Integer(n) } |
            character   => { |c| Datum::Char(c) } |
            string      => { |s| Datum::Str(s) } |
            identifier  => { |s| Datum::Symbol(s) } |
            list        => { |ds| Datum::List(ds) } |
            dotted_list => { |(ds, d)| Datum::DottedList(ds, Box::new(d)) } |
            quote       => { |q| Datum::List(vec!(make_symbol("quote"), q)) } |
            quasiquote  => { |q| Datum::List(vec!(make_symbol("quasiquote"), q)) } |
            unquote     => { |q| Datum::List(vec!(make_symbol("unquote"), q)) } |
            unquote_splicing => { |q| Datum::List(vec!(make_symbol("unquote-splicing"), q)) }
        ),
        intertoken_space
    )
);

named!(datums<Vec<Datum>>,
       do_parse!(
           datums: many0!(datum) >>
           eof!() >>
           (datums)));

pub fn parse_program(s: &str) -> Vec<Datum> {
    match datums(s.as_bytes()) {
      IResult::Done(rest, v) => {
          if rest.len() > 0 {
              panic!("Failed to parse the full input");
          }
          v
      },
      _ => panic!("Failed to parse datum")
    }
}

pub fn parse_datum(s: &str) -> Datum {
    match datum(s.as_bytes()) {
      IResult::Done(_, v) => v,
      _ => panic!("Failed to parse datum")
    }
}

fn find_max_hole(datum: &Datum) -> isize {
    let mut max = 0;
    match datum {
        &Datum::List(ref elems) => {
            for d in elems {
                let res = find_max_hole(d);
                if res > max {
                    max = res;
                }
            }
        }
        &Datum::Symbol(ref name) => {
            let mut tmp = name.clone();
            let first = tmp.remove(0);

            if first == '&' {
                let res = tmp.parse::<isize>().expect("Could not parse hole index");
                if res > max {
                    max = res
                }
            }
        }
        _ => ()
    }
    max
}

fn convert_hole_lambda_to_lambda(datums: Vec<Datum>) -> Datum {
    let body = Datum::List(datums);
    let max = find_max_hole(&body);

    let mut params: Vec<Datum> = Vec::new();

    for i in 1..(max + 1) {
        let param = format!("&{}", i);
        params.push(Datum::Symbol(param));
    }

    Datum::List(vec![
       Datum::Symbol(String::from("fn")),
       Datum::List(params),
       body
    ])
}

macro_rules! assert_parsed_fully {
($parser:expr, $input:expr, $result:expr) => {
    assert_eq!($parser($input.as_bytes()), nom::IResult::Done(&b""[..], $result));
} 
}
