use std::collections::HashMap;

use codespan_reporting::diagnostic::Diagnostic;
use diagnostics::RtError;
use parser::Name;
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

    pub fn lookup(&mut self, name: &Name) -> Result<Value, RtError> {
        self.find(name).map(|x| *x)
    }

    pub fn find(&mut self, name: &Name) -> Result<&mut Value, RtError> {
        // Polonius workaround
        let in_scope = self.in_scope();

        for i in self.vars.iter_mut().rev() {
            if let Some(var) = i.get_mut(name.node) {
                return Ok(var);
            }
        }

        return Err(RtError(
            Diagnostic::error()
                .with_message(format!("Couldn't find variable `{}` in scope", name.node))
                .with_labels(vec![name.span.primary_label()])
                .with_notes(vec![format!("Variables in scope: {:?}", in_scope)]),
        ));
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
