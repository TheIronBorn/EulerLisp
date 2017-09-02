pub fn load(hm: &mut HashMap<String, Value>) {
    register(hm, "=", Rc::new(|vs| {
        check_arity!(vs, 2);
        Ok(Value::Bool(vs[0] == vs[1]))
    }));
    register(hm, "!=", Rc::new(|vs| {
        check_arity!(vs, 2);
        Ok(Value::Bool(vs[0] != vs[1]))
    }));
    // TODO: What should happen when compairing two different types?
    register(hm, ">", Rc::new(|vs| {
        check_arity!(vs, 2);
        Ok(Value::Bool(vs[0] > vs[1]))
    }));
    register(hm, "<", Rc::new(|vs| {
        check_arity!(vs, 2);
        Ok(Value::Bool(vs[0] < vs[1]))
    }));
    register(hm, ">=", Rc::new(|vs| {
        check_arity!(vs, 2);
        Ok(Value::Bool(vs[0] >= vs[1]))
    }));
    register(hm, "<=", Rc::new(|vs| {
        check_arity!(vs, 2);
        Ok(Value::Bool(vs[0] <= vs[1]))
    }));
}
