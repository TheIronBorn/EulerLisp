use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SymbolTable {
    pub index: usize,
    mapping: HashMap<String, usize>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable { index: 0, mapping: HashMap::new() }
    }

    pub fn insert(&mut self, key: &String) -> usize {
        if self.mapping.contains_key(key) {
            *self.mapping.get(key).unwrap()
        } else {
            let i = self.index;
            self.mapping.insert(key.clone(), i);
            self.index += 1;

            i
        }
    }
}

#[test]
fn insert_and_lookup() {
    let mut st = SymbolTable::new();
    assert_eq!(st.insert("foo"), 0);
    assert_eq!(st.insert("bar"), 1);
    assert_eq!(st.insert("foo"), 0);
}
