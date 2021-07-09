use std::convert::TryInto;

use codespan_reporting::diagnostic::{Diagnostic, Label};
use eyre::Result;
use lalrpop_util::ParseError;

use binop::binop;
use diagnostics::span::{Span, Spanned};
use diagnostics::RtError;
use parser::{BinOp, UnaryOp};
use value::{BigValue, Map, Value, ValueDbg};

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
    ) -> Result<Value, RtError> {
        binop(self, l, o, r, l_span, o_span, r_span)
    }

    fn unary_op(&self, o: Spanned<UnaryOp>, v: Value, vs: Span) -> Result<Value> {
        Ok(match (o.node, v) {
            (UnaryOp::Not, Value::Bool(b)) => Value::Bool(!b),
            (UnaryOp::Minus, Value::Int(i)) => Value::Int(-i),
            (UnaryOp::Minus, Value::Float(f)) => Value::Float(-f),
            (_, v) => {
                return Err(RtError(
                    Diagnostic::error()
                        .with_message(format!(
                            "Unknown UnaryOp `{}` for `{}`",
                            o.node,
                            self.type_name(&v)
                        ))
                        .with_labels(vec![
                            o.span.primary_label().with_message("In this operator"),
                            self.evaled_to(v, vs),
                        ]),
                )
                .into())
            }
        })
    }

    fn check_map_has_key(
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

    fn check_array_bounds(
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

    fn as_bool(&self, val: Value, s: Span) -> Result<bool, RtError> {
        if let Value::Bool(b) = val {
            Ok(b)
        } else {
            Err(self.unexpected_type_error(val, s, "bool"))
        }
    }

    fn unexpected_type_error(&self, val: Value, s: Span, expected: &str) -> RtError {
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

    fn as_uint(&self, val: Value, s: Span) -> Result<usize, RtError> {
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

    fn as_array_mut(&mut self, val: Value, s: Span) -> Result<&mut [Value], RtError> {
        if let Value::Complex(id) = val {
            if let BigValue::Array(_) = self.heap()[id] {
                // Fun polonious workaround
                match &mut self.heap_mut()[id] {
                    BigValue::Array(a) => return Ok(a),
                    _ => unreachable!(),
                }
            }
        }
        Err(self.unexpected_type_error(val, s, "array"))
    }

    fn as_array(&self, val: Value, s: Span) -> Result<&[Value], RtError> {
        if let Value::Complex(id) = val {
            if let BigValue::Array(a) = &self.heap()[id] {
                return Ok(a);
            }
        }

        Err(self.unexpected_type_error(val, s, "array"))
    }

    fn as_map(&self, val: Value, s: Span) -> Result<&Map, RtError> {
        if let Value::Complex(id) = val {
            if let BigValue::Map(m) = &self.heap()[id] {
                return Ok(m);
            }
        }
        Err(self.unexpected_type_error(val, s, "map"))
    }

    fn as_map_mut(&mut self, val: Value, s: Span) -> Result<&mut Map, RtError> {
        if let Value::Complex(id) = val {
            if let BigValue::Map(_) = &self.heap()[id] {
                match &mut self.heap_mut()[id] {
                    BigValue::Map(m) => return Ok(m),
                    _ => unreachable!(),
                }
            }
        }
        Err(self.unexpected_type_error(val, s, "map"))
    }
}

pub fn parse_error_labeled(
    e: ParseError<usize, lalrpop_util::lexer::Token, &str>,
    main_file_id: usize,
) -> Diagnostic<usize> {
    let err = match e {
        ParseError::UnrecognizedToken { token, expected } => Diagnostic::error()
            .with_message("Unexpected token")
            .with_labels(vec![Label::primary(main_file_id, token.0..token.2)])
            .with_notes(
                expected
                    .iter()
                    .map(|e| format!("Expected: {}", e))
                    .collect(),
            ),

        ParseError::InvalidToken { location } => Diagnostic::error()
            .with_message("Invalid token")
            .with_labels(vec![Label::primary(main_file_id, location..location + 1)]),

        ParseError::UnrecognizedEOF { location, expected } => Diagnostic::error()
            .with_message("Unexpected EOF")
            .with_labels(vec![Label::primary(main_file_id, location..location + 1)])
            .with_notes(
                expected
                    .iter()
                    .map(|e| format!("Expected: {}", e))
                    .collect(),
            ),
        // TODO: Use nice reporting for the rest, if it exists
        _ => todo!(),
    };
    err
}
