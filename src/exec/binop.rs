use codespan_reporting::diagnostic::Diagnostic;
use eyre::Result;

use crate::ast::{BinOp, Span};
use crate::diagnostics::RtError;
use crate::value::{BigValue, HeapKey, Value};

use super::Env;

macro_rules! binop_match {
    (
        $bindings:expr,
        // Integer math ops
        { $($math_op_name:path => $math_op:tt),*$(,)? },
        // Comparison
        { $($comarison_name:path => $comparison_op:tt),*$(,)? },
        // Misc user stuff
        { $($user_lhs:pat => $user_rhs:expr),*$(,)? },
    ) => {
        // TODO: decide if this is a good idea
        #[allow(clippy::float_cmp)]
        match $bindings {
            $(
                // TODO: Should we coerce int to float in 1 + 2.0
                ($math_op_name, Int(l), Int(r)) => Int(l $math_op r),
                ($math_op_name, Float(l), Float(r)) => Float(l $math_op r),
            )*
            $(
                ($comarison_name, Int(l), Int(r)) => Bool(l $comparison_op r),
                ($comarison_name, Float(l), Float(r)) => Bool(l $comparison_op r),
            )*
            $( $user_lhs => $user_rhs, )*
        }
    };
}

impl<'a> Env<'a> {
    pub fn binop(
        &mut self,
        l: Value,
        o: BinOp,
        r: Value,
        l_span: Span,
        o_span: Span,
        r_span: Span,
    ) -> Result<Value> {
        use Value::*;

        Ok(binop_match!(
            (o, l, r),
            {
                BinOp::Plus   => +,
                BinOp::Minus  => -,
                BinOp::Times  => *,
                BinOp::Devide => /,
            },
            {
                BinOp::GreaterThan       => >,
                BinOp::GreaterThanEquals => >=,
                BinOp::LessThan          => <,
                BinOp::LessThanEquals    => <=,
            },
            {
                (BinOp::LogicalOr,  Bool(l),   Bool(r))   => Bool  (l || r),
                (BinOp::LogicalAnd, Bool(l),   Bool(r))   => Bool  (l && r),

                (BinOp::Equals, l, r) => Value::Bool(self.binop_eq(l, o, r, l_span, o_span, r_span)?),
                (BinOp::NotEquals, l, r) => Value::Bool(!self.binop_eq(l, o, r, l_span, o_span, r_span)?),

                (op, Complex(lid), Complex(rid)) => self.complex_binop(
                    lid,
                    op,
                    rid,
                    l_span,
                    o_span,
                    r_span,
                )?,
                (o, l, r) => return Err(self.binop_err(l, o, r, l_span, o_span, r_span).into())
            },
        ))
    }

    fn binop_eq(
        &self,
        l: Value,
        o: BinOp,
        r: Value,
        l_span: Span,
        o_span: Span,
        r_span: Span,
    ) -> Result<bool> {
        Ok(match (l, r) {
            // Simple cases
            (Value::Int(l), Value::Int(r)) => l == r,
            (Value::Float(l), Value::Float(r)) => l == r,
            (Value::Bool(l), Value::Bool(r)) => l == r,
            (Value::Null, Value::Null) => true,

            (Value::Complex(lid), Value::Complex(rid)) => {
                match (&self.heap[lid], &self.heap[rid]) {
                    (BigValue::String(l), BigValue::String(r)) => l == r,
                    (BigValue::Array(ls), BigValue::Array(rs)) => {
                        if ls.len() != rs.len() {
                            false
                        } else {
                            // Iterate over all the items, and equal each one, returning true if their all true,
                            // or the first error
                            ls.iter().zip(rs).try_fold(true, |acc, (l, r)| {
                                Result::<_, eyre::Error>::Ok(
                                    // TODO: l_span and `o_span` are wrong.
                                    acc && self.binop_eq(*l, o, *r, l_span, o_span, r_span)?,
                                )
                            })?
                        }
                    }
                    _ => {
                        return Err(self
                            .binop_err(
                                Value::Complex(lid),
                                o,
                                Value::Complex(rid),
                                l_span,
                                o_span,
                                r_span,
                            )
                            .into())
                    }
                }
            }
            (l, r) => return Err(self.binop_err(l, o, r, l_span, o_span, r_span).into()),
        })
    }

    fn complex_binop(
        &mut self,
        lid: HeapKey,
        o: BinOp,
        rid: HeapKey,
        l_span: Span,
        o_span: Span,
        r_span: Span,
    ) -> Result<Value> {
        Ok(match (o, &self.heap[lid], &self.heap[rid]) {
            // (BinOp::Equals, _, _) => Value::Bool(self.binop_eq(lid, rid, l_span, o_span, r_span)?),
            // (BinOp::NotEquals, _, _) => {
            //     Value::Bool(!self.binop_eq(lid, rid, l_span, o_span, r_span)?)
            // }
            (BinOp::Plus, BigValue::String(l), BigValue::String(r)) => {
                let res = l.to_owned() + r;
                let key = self.heap.insert(BigValue::String(res));
                Value::Complex(key)
            }
            _ => {
                return Err(self
                    .binop_err(
                        Value::Complex(lid),
                        o,
                        Value::Complex(rid),
                        l_span,
                        o_span,
                        r_span,
                    )
                    .into())
            }
        })
    }

    fn binop_err(
        &self,
        l: Value,
        o: BinOp,
        r: Value,
        l_span: Span,
        o_span: Span,
        r_span: Span,
    ) -> RtError {
        RtError(
            Diagnostic::error()
                .with_message(format!(
                    "Unknown binop `{}`, for `{}` and `{}`",
                    o,
                    self.type_name(&l),
                    self.type_name(&r),
                ))
                .with_labels(vec![
                    o_span.primary_label().with_message("In this operator"),
                    l_span
                        .secondary_label()
                        .with_message(format!("LHS evaluated to {:?}", self.dbg_val(&l))),
                    r_span
                        .secondary_label()
                        .with_message(format!("LHS evaluated to {:?}", self.dbg_val(&r))),
                ]),
        )
    }
}
