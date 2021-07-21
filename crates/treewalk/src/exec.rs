use std::collections::BTreeMap;
use std::mem;

use codespan_reporting::diagnostic::Diagnostic;
use diagnostics::span::Span;
use diagnostics::RtError;
use eyre::Result;
use parser::{Block, Expr, FnBody, RawExpr, RawStmt};
use rt_common::RT;
use value::{BigValue, Value};

use crate::scope::Scope;
use crate::VM;

#[must_use = "use the get! macro to potentialy early return"]
pub(crate) enum BlockExecResult {
    FnRet(Value),
    None,
}

// Note: always get! an `BlockExecResult` before creating another to avoid #29
#[macro_export]
macro_rules! get {
    ($e: expr) => {
        match $e {
            BlockExecResult::None => {}
            x @ BlockExecResult::FnRet(_) => return Ok(x),
        }
    };
}

impl<'a, 'b> VM<'a, 'b> {
    pub fn call(&mut self, name: parser::Name, args: &[Value], call_span: Span) -> Result<Value> {
        let fn_name = name.node;

        let function = self.functions.get(&fn_name).ok_or_else(|| {
            RtError(
                Diagnostic::error()
                    .with_message(format!("No function named `{}`", &fn_name))
                    .with_labels(vec![name.primary_label()]),
            )
        })?;

        if args.len() != function.args.len() {
            // TODO: Be more perise with spans
            return Err(RtError(
                Diagnostic::error()
                    .with_message(format!(
                        "Expected {} args, found {}",
                        function.args.len(),
                        args.len()
                    ))
                    .with_labels(vec![
                        call_span
                            .primary_label()
                            .with_message(format!("Calling here with {} arguments", args.len())),
                        function.span.secondary_label().with_message(format!(
                            "Defined here with {} arguments",
                            function.args.len()
                        )),
                    ]),
            )
            .into());
        }

        let mut scope = Scope::default();

        for (name, val) in function.args.iter().zip(args.iter()) {
            scope.declare(&name.name, *val);
        }

        // Weird workaroud for rust-lang/rust#59159, see #41
        let body = &function.body;

        // In functions, a trailing expression returns
        let res = match body {
            FnBody::Expr(e) => self.eval_in(&mut scope, e)?,
            FnBody::Block(b) => match self.eval_block_in(b, &mut scope)? {
                BlockExecResult::FnRet(v) => v,
                BlockExecResult::None => Value::Null,
            },
        };

        Ok(res)
    }

    fn eval_block_in(
        &mut self,
        block: &Block<'a>,
        scope: &mut Scope<'a>,
    ) -> Result<BlockExecResult> {
        scope.push();

        use RawStmt::*;

        for i in &block.node {
            match &i.node {
                Let(name, expr) => {
                    let val = self.eval_in(scope, expr)?;
                    scope.declare(name, val);
                }
                Assign(lvalue, rvalue) => {
                    (self.lvalue(lvalue, rvalue, scope)?);
                }
                Print(e) => {
                    let val = self.eval_in(scope, e)?;
                    self.print_value(&val)?;
                }
                Return(e) => {
                    let val = self.eval_in(scope, e)?;
                    // Clear scope, so we bug if we try to access anything afterwards
                    mem::take(scope);
                    return Ok(BlockExecResult::FnRet(val));
                }
                Expr(e) => {
                    self.eval_in(scope, e)?;
                }
                If(test, ifcase, elsecase) => {
                    let test_val = self.eval_in(scope, &*test)?;
                    let eval_result = if self.as_bool(test_val, test.span)? {
                        self.eval_block_in(ifcase, scope)?
                    } else if let Some(block) = elsecase {
                        self.eval_block_in(block, scope)?
                    } else {
                        BlockExecResult::None
                    };
                    get!(eval_result)
                }
                Block(block) => {
                    let res = self.eval_block_in(block, scope)?;
                    get!(res);
                }
                While(cond, block) => {
                    while {
                        let val = self.eval_in(scope, cond)?;
                        self.as_bool(val, cond.span)?
                    } {
                        get!(self.eval_block_in(block, scope)?);
                    }
                }
                _ => todo!(),
            }
        }

        // We dont need to do this is the FnRet case, as the scope is cleared
        scope.pop();

        Ok(BlockExecResult::None)
    }

    pub(crate) fn eval_in(&mut self, scope: &mut Scope<'a>, e: &Expr<'a>) -> Result<Value> {
        // https://github.com/rust-lang/rust/issues/86418
        // TODO: Move this import to the top
        use parser::Literal;
        use RawExpr::*;
        Ok(match &e.node {
            Literal(l) => match l.node {
                Literal::String(s) => {
                    Value::Complex(self.add_to_heap(BigValue::String(s.to_owned())))
                }
                Literal::Float(f) => Value::Float(f),
                Literal::Integer(i) => Value::Int(i),
                Literal::Bool(b) => Value::Bool(b),
                Literal::Null => Value::Null,
            },
            BinOp(l, o, r) => {
                // TODO: is this eval order right
                let lv = self.eval_in(scope, l)?;
                let rv = self.eval_in(scope, r)?;
                self.binop(lv, o.node, rv, l.span, o.span, r.span)?
            }
            UnaryOp(o, e) => {
                let val = self.eval_in(scope, e)?;
                self.unary_op(*o, val, e.span)?
            }
            Call(function, args) => {
                if let RawExpr::Var(name) = &function.node {
                    assert_eq!(name.len(), 1);
                    let name = name[0];

                    let mut args_evald = Vec::with_capacity(args.len());
                    for i in args {
                        args_evald.push(self.eval_in(scope, i)?);
                    }
                    self.call(name, &args_evald, e.span)?
                } else {
                    // TODO: Proper first class functions
                    // See #32
                    return Err(RtError(
                        Diagnostic::error()
                            .with_message("Expected a name")
                            .with_labels(vec![function.span.primary_label()]),
                    )
                    .into());
                }
            }
            Var(path) => {
                // TODO: Handle Module's here
                assert_eq!(path.len(), 1);
                let node = path[0];
                scope.lookup(&node)?
            }

            // Block(b) => {
            //     let bval = self.eval_block_in(&b, scope)?;
            //     get!(bval)
            // }
            Array(x) => {
                let mut ret = Vec::with_capacity(x.len());
                for i in x {
                    ret.push(self.eval_in(scope, i)?);
                }

                let key = self.heap.insert(BigValue::Array(ret));
                Value::Complex(key)
            }
            Map(m) => {
                let mut ret = BTreeMap::new();
                for (name, val) in m {
                    if ret
                        .insert(name.node.to_owned(), self.eval_in(scope, val)?)
                        .is_some()
                    {
                        let (prev, _) =
                            m.iter().find(|(pname, _)| pname.node == name.node).unwrap();

                        return Err(RtError(
                            Diagnostic::error()
                                .with_message(format!(
                                    "Duplicate key `{}` in map literal",
                                    name.node
                                ))
                                .with_labels(vec![
                                    prev.primary_label().with_message("First defined here"),
                                    name.primary_label().with_message("Defined here again"),
                                ]),
                        )
                        .into());
                    }
                }
                let key = self.heap.insert(BigValue::Map(ret));
                Value::Complex(key)
            }
            ArrayAccess(arr_s, idx_s) => {
                let array_val = self.eval_in(scope, arr_s)?;

                // Nessesary for BorrowCk, as we cant borrow from the heap while we evaluate
                // the index, and potentialy mutate. Probably killing perf, LMAO
                let arr = self.as_array_mut(array_val, arr_s.span)?.to_owned();

                let idx_val = self.eval_in(scope, idx_s)?;
                let idx = self.as_uint(idx_val, idx_s.span)?;

                // TODO: This is wrong if we mutate the array when evaluating the index, I think
                self.check_array_bounds(&arr, idx, array_val, idx_val, arr_s.span, idx_s.span)?;
                arr[idx]
            }
            FieldAccess(map_s, key_s) => {
                let map_val = self.eval_in(scope, map_s)?;
                let map = self.as_map(map_val, map_s.span)?;

                // nessesary for deref coersion, as they compiller can figure
                // out map key type
                let key_string: &str = key_s;

                self.check_map_has_key(map, key_string, map_s.span, key_s.span, map_val)?;
                map[key_string]
            }
        })
    }
}
