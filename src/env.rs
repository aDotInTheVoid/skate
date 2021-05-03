use std::collections::HashMap;

use slotmap::SlotMap;

use crate::ast::{Function, Spanned};
use crate::value::{BigValue, HeapKey, Value};

// Scope only sticks around for the duration of a function call, wheras env
// exists for the duration Of a program. Both of these will neeed to be
// signifagently changed when we do GC and RR, so try not to make it too
// hard to do that.

#[derive(Debug, Default)]
pub struct Scope<'a> {
    pub(crate) vars: Vec<HashMap<&'a str, Value>>,
}

#[derive(Debug, Default)]
pub(crate) struct Env<'a> {
    pub(crate) functions: HashMap<&'a str, &'a Spanned<Function<'a>>>,
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
}

impl<'a> Env<'a> {
    pub fn add_to_heap(&mut self, v: BigValue) -> HeapKey {
        self.heap.insert(v)
    }
}
