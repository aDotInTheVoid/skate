use binop::binop;
use eyre::Result;

use codespan_reporting::diagnostic::Label;
use diagnostics::span::Span;
use parser::BinOp;
use value::{BigValue, Value, ValueDbg};

mod binop;

pub trait RT: Sized {
    // Consider a macro for these, or make all these methods on `Heap`
    fn heap(&self) -> &value::Heap;
    fn heap_mut(&mut self) -> &mut value::Heap;

    fn type_name(&self, v: &Value) -> &'static str {
        match v {
            Value::Int(_) => "int",
            Value::Float(_) => "float",
            Value::Bool(_) => "bool",
            Value::Complex(id) => match self.heap()[*id] {
                BigValue::String(_) => "string",
                // TODO: Show inner type?
                BigValue::Array(_) => "array",
                BigValue::Map(_) => "map",
            },
            Value::Null => "null",
        }
    }

    fn dbg_val<'v>(&'v self, v: &'v Value) -> ValueDbg<'v> {
        ValueDbg {
            v,
            heap: self.heap(),
        }
    }

    fn evaled_to_primary(&self, v: Value, s: Span) -> Label<usize> {
        s.primary_label()
            .with_message(format!("Evaluated to `{:?}`", self.dbg_val(&v)))
    }

    fn evaled_to(&self, v: Value, s: Span) -> Label<usize> {
        s.secondary_label()
            .with_message(format!("Evaluated to `{:?}`", self.dbg_val(&v)))
    }

    fn binop(
        &mut self,
        l: Value,
        o: BinOp,
        r: Value,
        l_span: Span,
        o_span: Span,
        r_span: Span,
    ) -> Result<Value> {
        binop(self, l, o, r, l_span, o_span, r_span)
    }
}
