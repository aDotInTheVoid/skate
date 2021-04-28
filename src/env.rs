use std::collections::HashMap;

use crate::value::Value;

#[derive(Debug, Default)]
pub struct Scope<'a> {
    vars: HashMap<&'a str, Value>,
}

impl<'a> Scope<'a> {
    pub fn declare(&mut self, name: &'a str, v: Value) {
        // TODO: Handle collisions
        self.vars.insert(name, v);
    }

    pub fn lookup(&mut self, name: &str) -> Option<Value> {
        self.vars.get(name).map(Clone::clone)
    }

    pub fn in_scope(&self) -> Vec<&'a str> {
        let mut keys: Vec<&str> = self.vars.keys().copied().collect();
        keys.sort_unstable();
        keys
    }
}
