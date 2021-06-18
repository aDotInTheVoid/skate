use crate::env::VM;
use crate::value::Value;
use codespan_reporting::diagnostic::Label;
use diagnostics::span::Span;

// TODO: Replace with methods on VM
pub(crate) trait SpanHack {
    fn evaled_to_primary(&self, v: Value, e: &VM) -> Label<usize>;
    fn evaled_to(&self, v: Value, e: &VM) -> Label<usize>;
}

impl SpanHack for Span {
    fn evaled_to_primary(&self, v: Value, e: &VM) -> Label<usize> {
        self.primary_label()
            .with_message(format!("Evaluated to `{:?}`", e.dbg_val(&v)))
    }

    fn evaled_to(&self, v: Value, e: &VM) -> Label<usize> {
        self.secondary_label()
            .with_message(format!("Evaluated to `{:?}`", e.dbg_val(&v)))
    }
}
