use std::collections::HashMap;

use value::Value;

#[derive(Debug, Default)]
pub struct Scope<'a> {
    pub(crate) vars: Vec<HashMap<&'a str, Value>>,
}

impl<'a> Scope<'a> {
    // Used on `let`, or to create scope for function call
    pub fn declare(&mut self, name: &'a str, v: Value) {
        // TODO: Handle collisions
        // self.vars.insert(name, v);
        let last = match self.vars.last_mut() {
            Some(x) => x,
            None => {
                self.vars.push(HashMap::new());
                self.vars.last_mut().unwrap()
            }
        };
        last.insert(name, v);
    }

    pub fn lookup(&mut self, name: &str) -> Option<Value> {
        self.find(name).map(|x| *x)
    }

    pub fn find(&mut self, name: &str) -> Option<&mut Value> {
        let mut ret = None;
        for i in self.vars.iter_mut().rev() {
            if let Some(var) = i.get_mut(name) {
                ret = Some(var);
                break;
            }
        }
        ret
    }

    pub fn in_scope(&self) -> Vec<&'a str> {
        self.vars
            .iter()
            .rev()
            .flat_map(|x| {
                let mut keys: Vec<&str> = x.keys().copied().collect();
                keys.sort_unstable();
                keys
            })
            .collect()
    }

    pub fn push(&mut self) {
        self.vars.push(HashMap::new());
    }

    pub fn pop(&mut self) {
        self.vars.pop().unwrap();
    }
}