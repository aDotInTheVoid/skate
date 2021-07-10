use codespan_reporting::diagnostic::Diagnostic;
use eyre::Result;

use diagnostics::RtError;
use parser::{Expr, RawExpr};
use rt_common::RT;

use crate::scope::Scope;

use super::VM;

impl<'a> VM<'a, '_> {
    pub(crate) fn lvalue(
        &mut self,
        // The location were assigning to
        lvalue: &Expr<'a>,
        // The rhs, the value to assign
        rvalue: &Expr<'a>,
        scope: &mut Scope<'a>,
    ) -> Result<()> {
        match &lvalue.node {
            RawExpr::Var(var) => {
                assert_eq!(var.len(), 1);
                let var = &var[0];
                let val = self.eval_in(scope, rvalue)?;
                let ptr = scope.find(var)?;
                *ptr = val;
            }
            RawExpr::FieldAccess(map_s, key_s) => {
                // TODO: This evaluates the RHS before checking if LHS is map, is this good?
                // TODO: Handle insertion vs replacement
                // TODO: Allow dynamic keys
                let map_val = self.eval_in(scope, map_s)?;
                let rv = self.eval_in(scope, rvalue)?;
                let map = self.as_map_mut(map_val, map_s.span)?;
                map.insert(key_s.to_string(), rv);
            }
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
                // Unwrap as weve already check its an array
                let array = self.as_array(array_val, arr_s.span).unwrap();
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
