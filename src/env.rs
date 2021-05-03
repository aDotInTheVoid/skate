use std::collections::HashMap;

use slotmap::SlotMap;

use crate::value::{BigValue, HeapKey, Value};

#[derive(Debug, Default)]
pub struct Scope<'a> {
    pub(crate) vars: Vec<HashMap<&'a str, Value>>,
    pub(crate) heap: SlotMap<HeapKey, BigValue>,
}

impl<'a> Scope<'a> {
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
        // self.vars.get(name).map(Clone::clone)
        let mut ret = None;
        for i in self.vars.iter().rev() {
            if let Some(var) = i.get(name) {
                ret = Some(var.clone());
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

    pub fn print_value(&self, v: &Value) {}

    pub fn add_to_heap(&mut self, v: BigValue) -> HeapKey {
        self.heap.insert(v)
    }
}
