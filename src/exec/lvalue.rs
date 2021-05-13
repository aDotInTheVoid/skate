use codespan_reporting::diagnostic::Diagnostic;
use eyre::Result;

use crate::ast::{Expr, RawExpr};
use crate::diagnostics::RtError;
use crate::env::Scope;
use crate::get;
use crate::value::Value;

use super::{BlockEvalResult, Env};

impl<'a> Env<'a> {
    pub(crate) fn lvalue(
        &mut self,
        lvalue: &Expr<'a>,
        rvalue: &Expr<'a>,
        scope: &mut Scope<'a>,
    ) -> Result<BlockEvalResult> {
        match &lvalue.node {
            RawExpr::Var(var) => {
                let val = get!(self.eval_in(scope, rvalue)?);
                match scope.find(&var) {
                    Some(ptr) => *ptr = val,
                    None => {
                        return Err(RtError(
                            Diagnostic::error()
                                .with_message(format!("No variable named `{}` in scope", var.node))
                                .with_labels(vec![var.primary_label()]),
                        )
                        .into());
                    }
                };
            }
            // RawExpr::FieldAccess(_, _) => {}
            RawExpr::ArrayAccess(arr_s, idx_s) => {
                // TODO: This is too subtle due to borrowck

                // First evaluate the array.
                let arr = get!(self.eval_in(scope, arr_s)?);
                // Check it is an array, to early error out.
                self.as_array(arr, arr_s.span)?;

                // Evaluate the index
                let idx = get!(self.eval_in(scope, idx_s)?);
                let idx = self.as_uint(idx, idx_s.span)?;

                // Evaluate the rvalue
                let val = get!(self.eval_in(scope, rvalue)?);

                // Store the value
                self.as_array(arr, arr_s.span)?[idx] = val;
            }

            _ => {
                return Err(RtError(
                    Diagnostic::error()
                        .with_message("Excpected an lvalue")
                        .with_labels(vec![lvalue.span.primary_label()]),
                )
                .into())
            }
        }

        Ok(BlockEvalResult::LocalRet(Value::Null))
    }
}
