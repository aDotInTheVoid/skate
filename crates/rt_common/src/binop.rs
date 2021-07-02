use codespan_reporting::diagnostic::Diagnostic;
use diagnostics::{span::Span, RtError};
use eyre::Result;
use parser::BinOp;
use value::{BigValue, HeapKey, Value};

use crate::RT;

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

pub fn binop<T: RT>(
    this: &mut T,
    l: Value,
    o: BinOp,
    r: Value,
    l_span: Span,
    o_span: Span,
    r_span: Span,
) -> Result<Value, RtError> {
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

            (BinOp::Equals, l, r) => Value::Bool(binop_eq(this,l, o, r, l_span, o_span, r_span)?),
            (BinOp::NotEquals, l, r) => Value::Bool(!binop_eq(this,l, o, r, l_span, o_span, r_span)?),

            (op, Complex(lid), Complex(rid)) => complex_binop(this,
                lid,
                op,
                rid,
                l_span,
                o_span,
                r_span,
            )?,
            (o, l, r) => return Err(binop_err(this,l, o, r, l_span, o_span, r_span))
        },
    ))
}

fn binop_eq<T: RT>(
    this: &T,
    l: Value,
    o: BinOp,
    r: Value,
    l_span: Span,
    o_span: Span,
    r_span: Span,
) -> Result<bool, RtError> {
    Ok(match (l, r) {
        // Simple cases
        (Value::Int(l), Value::Int(r)) => l == r,
        (Value::Float(l), Value::Float(r)) => l == r,
        (Value::Bool(l), Value::Bool(r)) => l == r,
        (Value::Null, Value::Null) => true,

        (Value::Complex(lid), Value::Complex(rid)) => {
            match (&this.heap()[lid], &this.heap()[rid]) {
                (BigValue::String(l), BigValue::String(r)) => l == r,
                (BigValue::Array(ls), BigValue::Array(rs)) => {
                    if ls.len() != rs.len() {
                        false
                    } else {
                        // Iterate over all the items, and equal each one, returning true if their all true,
                        // or the first error
                        ls.iter().zip(rs).try_fold(true, |acc, (l, r)| {
                            Ok(
                                // TODO: l_span and `o_span` are wrong.
                                acc && binop_eq(this, *l, o, *r, l_span, o_span, r_span)?,
                            )
                        })?
                    }
                }
                _ => {
                    return Err(binop_err(
                        this,
                        Value::Complex(lid),
                        o,
                        Value::Complex(rid),
                        l_span,
                        o_span,
                        r_span,
                    ))
                }
            }
        }
        (l, r) => return Err(binop_err(this, l, o, r, l_span, o_span, r_span)),
    })
}

fn complex_binop<T: RT>(
    this: &mut T,
    lid: HeapKey,
    o: BinOp,
    rid: HeapKey,
    l_span: Span,
    o_span: Span,
    r_span: Span,
) -> Result<Value, RtError> {
    Ok(match (o, &this.heap()[lid], &this.heap()[rid]) {
        // (BinOp::Equals, _, _) => Value::Bool(binop_eq(this,lid, rid, l_span, o_span, r_span)?),
        // (BinOp::NotEquals, _, _) => {
        //     Value::Bool(!binop_eq(this,lid, rid, l_span, o_span, r_span)?)
        // }
        (BinOp::Plus, BigValue::String(l), BigValue::String(r)) => {
            let res = l.to_owned() + r;
            let key = this.heap_mut().insert(BigValue::String(res));
            Value::Complex(key)
        }
        _ => {
            return Err(binop_err(
                this,
                Value::Complex(lid),
                o,
                Value::Complex(rid),
                l_span,
                o_span,
                r_span,
            ))
        }
    })
}

fn binop_err<T: RT>(
    this: &T,
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
                this.type_name(&l),
                this.type_name(&r),
            ))
            .with_labels(vec![
                o_span.primary_label().with_message("In this operator"),
                this.evaled_to(l, l_span),
                this.evaled_to(r, r_span),
            ]),
    )
}
