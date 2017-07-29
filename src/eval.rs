use std::collections::HashMap;

use ::Value;

pub struct Environment {
    bindings: HashMap<String, Value>
}

pub fn eval(v: Value) -> Value {
  match v {
    Value::List(elems) => {
        let mapped: Vec<Value> = elems.iter().map(|e| eval(e.clone())).collect();
        apply_function(&mapped[0], &mapped[1..])
    },
    other => other
  }
}

fn apply_function(v: &Value, args: &[Value]) -> Value {
  match *v {
      Value::Atom(ref x) => match &x[..] {
          "add" => primitive_add(args),
          "mul" => primitive_mul(args),
          ref other => panic!("Unknown function {:?}", other)
      },
      ref other => panic!("Unknown function {:?}", *other)
  }
}

fn primitive_add(args: &[Value]) -> Value {
    let mut sum = 0;
    for i in args {
        match i {
          &Value::Number(n) => {
              sum += n;
          },
          other => panic!("Invalid argument for `add`: {:?}", other)
        }
    }
    Value::Number(sum)
}

fn primitive_mul(args: &[Value]) -> Value {
    let mut sum = 1;
    for i in args {
        match i {
          &Value::Number(n) => {
              sum *= n;
          },
          other => panic!("Invalid argument for `add`: {:?}", other)
        }
    }
    Value::Number(sum)
}
