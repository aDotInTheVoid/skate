use codespan_reporting::diagnostic::Diagnostic;
use eyre::Result;

use crate::ast::{Expr, RawExpr};
use crate::diagnostics::RtError;
use crate::env::Scope;

use super::Env;

impl<'a> Env<'a, '_> {
    pub(crate) fn lvalue(
        &mut self,
        lvalue: &Expr<'a>,
        rvalue: &Expr<'a>,
        scope: &mut Scope<'a>,
    ) -> Result<()> {
        match &lvalue.node {
            RawExpr::Var(var) => {
                let val = self.eval_in(scope, rvalue)?;
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
                let array_val = self.eval_in(scope, arr_s)?;
                // Check it is an array, to early error out.
                self.as_array_mut(array_val, arr_s.span)?;

                // Evaluate the index
                let idx_val = self.eval_in(scope, idx_s)?;
                let idx = self.as_uint(idx_val, idx_s.span)?;

                // Evaluate the rvalue
                let val = self.eval_in(scope, rvalue)?;

                // First we get as an array and check bounds
                let array = self.as_array(array_val, arr_s.span)?;
                self.check_array_bounds(array, idx, array_val, idx_val, arr_s.span, idx_s.span)?;

                // Then we get again, mutably, unwraping as it is garenteed to
                // be an array, to store the value
                let array = self.as_array_mut(array_val, arr_s.span).unwrap();
                array[idx] = val;
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
        Ok(())
    }
}
