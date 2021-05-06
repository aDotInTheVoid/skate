use codespan_reporting::diagnostic::Diagnostic;
use eyre::Result;

use crate::ast::{Expr, RawExpr};
use crate::diagnostics::RtError;
use crate::env::Scope;
use crate::value::Value;

use super::{BlockEvalResult, Env};

impl<'a> Env<'a> {
    pub(crate) fn lvalue(
        &mut self,
        lvalue: &Expr<'a>,
        rvalue: &Expr<'a>,
        scope: &mut Scope<'a>,
    ) -> Result<BlockEvalResult> {
        match lvalue.node {
            RawExpr::Var(var) => {
                let val = crate::get!(self.eval_in(scope, rvalue)?);
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
            // RawExpr::ArrayAccess(_, _) => {}
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
