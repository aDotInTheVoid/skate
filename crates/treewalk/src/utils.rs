use std::convert::TryInto;

use codespan_reporting::diagnostic::Diagnostic;
use diagnostics::span::Span;
use diagnostics::RtError;
use eyre::Result;
use rt_common::RT;
use value::{BigValue, Map, Value, ValueDbg};

use crate::VM;

// TODO: Move all to rt_common

impl<'a, 'b> VM<'a, 'b> {
    pub(crate) fn check_map_has_key(
        &self,
        map: &Map,
        key: &str,
        map_span: Span,
        key_span: Span,
        map_value: Value,
    ) -> Result<(), RtError> {
        if !map.contains_key(key) {
            Err(RtError(
                Diagnostic::error()
                    .with_message(format!("Map doesnt have key `{}`", key))
                    .with_labels(vec![
                        self.evaled_to(map_value, map_span),
                        key_span.primary_label().with_message("This key"),
                    ]),
            ))
        } else {
            Ok(())
        }
    }

    pub(crate) fn check_array_bounds(
        &self,
        array: &[Value],
        idx: usize,
        array_val: Value,
        idx_val: Value,
        array_span: Span,
        idx_span: Span,
    ) -> Result<(), RtError> {
        if array.len() <= idx {
            Err(RtError(
                Diagnostic::error()
                    .with_message(format!(
                        "Array index out of bound, len is `{}`, but got `{}`",
                        array.len(),
                        idx
                    ))
                    .with_labels(vec![
                        self.evaled_to(idx_val, idx_span),
                        self.evaled_to(array_val, array_span),
                    ]),
            ))
        } else {
            Ok(())
        }
    }

    pub(crate) fn as_bool(&self, val: Value, s: Span) -> Result<bool, RtError> {
        if let Value::Bool(b) = val {
            Ok(b)
        } else {
            Err(self.unexpected_type_error(val, s, "bool"))
        }
    }

    pub(crate) fn unexpected_type_error(&self, val: Value, s: Span, expected: &str) -> RtError {
        RtError(
            Diagnostic::error()
                .with_message(format!(
                    "Expected `{}`, got `{}`",
                    expected,
                    self.type_name(&val)
                ))
                .with_labels(vec![self.evaled_to_primary(val, s)]),
        )
    }

    pub(crate) fn as_uint(&self, val: Value, s: Span) -> Result<usize, RtError> {
        if let Value::Int(i) = val {
            if let Ok(u) = i.try_into() {
                Ok(u)
            } else {
                Err(RtError(
                    Diagnostic::error()
                        .with_message("Expected a positive number")
                        .with_labels(vec![self.evaled_to_primary(Value::Int(i), s)]),
                ))
            }
        } else {
            Err(self.unexpected_type_error(val, s, "int"))
        }
    }
    pub(crate) fn as_array_mut(&mut self, val: Value, s: Span) -> Result<&mut [Value], RtError> {
        if let Value::Complex(id) = val {
            if let BigValue::Array(_) = &self.heap[id] {
                // Fun polonious workaround
                match &mut self.heap[id] {
                    BigValue::Array(a) => return Ok(a),
                    _ => unreachable!(),
                }
            }
        }
        Err(self.unexpected_type_error(val, s, "array"))
    }

    pub(crate) fn as_array(&self, val: Value, s: Span) -> Result<&[Value], RtError> {
        if let Value::Complex(id) = val {
            if let BigValue::Array(a) = &self.heap[id] {
                return Ok(a);
            }
        }

        Err(self.unexpected_type_error(val, s, "array"))
    }

    pub(crate) fn as_map(&self, val: Value, s: Span) -> Result<&Map, RtError> {
        if let Value::Complex(id) = val {
            if let BigValue::Map(m) = &self.heap[id] {
                return Ok(m);
            }
        }
        Err(self.unexpected_type_error(val, s, "map"))
    }

    pub(crate) fn as_map_mut(&mut self, val: Value, s: Span) -> Result<&mut Map, RtError> {
        if let Value::Complex(id) = val {
            if let BigValue::Map(_) = &self.heap[id] {
                match &mut self.heap[id] {
                    BigValue::Map(m) => return Ok(m),
                    _ => unreachable!(),
                }
            }
        }
        Err(self.unexpected_type_error(val, s, "map"))
    }

    pub(crate) fn print_value(&mut self, v: &Value) -> Result<()> {
        let dbg = ValueDbg {
            v,
            heap: &self.heap,
        };
        writeln!(self.output, "{}", dbg)?;
        Ok(())
    }
}
