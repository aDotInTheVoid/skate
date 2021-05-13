/// Tree walk interpriter
use std::collections::HashMap;
use std::convert::TryInto;
use std::mem;

use codespan_reporting::diagnostic::Diagnostic;
use eyre::{bail, Result};

use crate::ast::{
    self, Block, BlockType, Expr, Function, Item, Program, RawExpr, Span, Spanned, Stmt, UnaryOp,
};
use crate::diagnostics::RtError;
use crate::env::{Env, Scope};
use crate::value::{BigValue, Value, ValueDbg};

mod binop;
mod lvalue;

// Err -> Exit err due to type error/rt error
// Ok(false) -> Exit sucess
// Ok(true) -> Exit err due to user code request
pub fn run(p: Program) -> Result<bool> {
    // If nothing failed, we suceed
    let mut env = Env::new(&p)?;

    let result = env.call(
        Spanned {
            node: "main",
            // TODO: give a real span to ensure the no main fn error is good
            span: Default::default(),
        },
        &[],
        // ditto
        Default::default(),
    )?;

    // TODO: Figure out exit codes 0/1/101
    // One for script exited with error, one for IIE
    let is_fail = match result {
        Value::Null => false,
        Value::Int(x) => x != 0,
        x => {
            // TODO: Give a span
            return Err(RtError(Diagnostic::error().with_message(format!(
                "`main` returned `{:?}` with type `{}`, expected `null` or `int`",
                env.dbg_val(&x),
                env.type_name(&x),
            )))
            .into());
        }
    };

    Ok(is_fail)
}

#[must_use = "use the get! macro to potentialy early return"]
pub(crate) enum BlockEvalResult {
    FnRet(Value),
    LocalRet(Value),
}

// Note: always get! an `BlockEvalResult` before creating another to avoid #29
#[macro_export]
macro_rules! get {
    ($e: expr) => {
        match $e {
            BlockEvalResult::LocalRet(x) => x,
            x @ BlockEvalResult::FnRet(_) => return Ok(x),
        }
    };
}

impl<'a> Env<'a> {
    pub fn new(p: &'a [Item<'a>]) -> eyre::Result<Self> {
        let mut functions: HashMap<&str, &Spanned<Function>> = HashMap::new();
        for i in p {
            match i {
                Item::Function(f) => {
                    if let Some(old_fn) = functions.insert(&f.name, &f) {
                        // TODO: Should these be the function span of the name span
                        let err = Diagnostic::error()
                            .with_message(format!("Function `{}` defined twice", &f.name.node))
                            .with_labels(vec![
                                old_fn.secondary_label().with_message("Defined once here"),
                                f.secondary_label().with_message("Defined again here"),
                            ]);
                        return Err(RtError(err).into());
                    }
                }
            };
        }

        Ok(Self {
            functions,
            heap: Default::default(),
        })
    }

    pub fn call(&mut self, name: ast::Name, args: &[Value], call_span: Span) -> Result<Value> {
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
        Ok(match self.eval_block_in(body, &mut scope)? {
            BlockEvalResult::FnRet(x) => x,
            BlockEvalResult::LocalRet(x) => x,
        })
    }

    fn eval_block_in(
        &mut self,
        block: &Block<'a>,
        scope: &mut Scope<'a>,
    ) -> Result<BlockEvalResult> {
        scope.push();

        use Stmt::*;

        let mut last_val = Value::Null;
        for i in &block.0 {
            match i {
                Let(name, expr) => {
                    let val = get!(self.eval_in(scope, expr)?);
                    scope.declare(name, val);
                    last_val = Value::Null;
                }
                Assign(lvalue, rvalue) => {
                    get!(self.lvalue(lvalue, rvalue, scope)?);
                    last_val = Value::Null;
                }
                Print(e) => {
                    let val = get!(self.eval_in(scope, e)?);
                    self.print_value(&val);
                    last_val = Value::Null;
                }
                Return(e) => {
                    let val = get!(self.eval_in(scope, e)?);
                    // Clear scope, so we bug if we try to access anything afterwards
                    mem::take(scope);
                    return Ok(BlockEvalResult::FnRet(val));
                }
                Expr(e) => {
                    let e = get!(self.eval_in(scope, e)?);
                    last_val = e;
                }
            }
        }

        // TODO: Clean up
        let ret = match block.1 {
            BlockType::Discard => Value::Null,
            BlockType::ReturnExpr => last_val,
        };

        // We dont need to do this is the FnRet case, as the scope is cleared
        scope.pop();

        Ok(BlockEvalResult::LocalRet(ret))
    }

    pub(crate) fn eval_in(
        &mut self,
        scope: &mut Scope<'a>,
        e: &Expr<'a>,
    ) -> Result<BlockEvalResult> {
        use crate::ast::Literal;
        use RawExpr::*;

        Ok(BlockEvalResult::LocalRet(match &e.node {
            Literal(l) => match l.node {
                // Literal::String(s) => Value::String((*s).to_owned()),
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
                let lv = get!(self.eval_in(scope, &l)?);
                let rv = get!(self.eval_in(scope, &r)?);
                self.binop(lv, o.node, rv, l.span, o.span, r.span)?
            }
            UnaryOp(o, e) => {
                let val = get!(self.eval_in(scope, &e)?);
                self.unary_op(*o, val, e.span)?
            }
            Call(function, args) => {
                if let RawExpr::Var(name) = function.node {
                    let mut args_evald = Vec::with_capacity(args.len());
                    for i in args {
                        args_evald.push(get!(self.eval_in(scope, &i)?));
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
            Var(Spanned { node, span }) => scope.lookup(node).ok_or_else(|| {
                RtError(
                    Diagnostic::error()
                        .with_message(format!("Couldn't find variable `{}` in scope", node))
                        .with_labels(vec![span.primary_label()])
                        .with_notes(vec![format!("Variables in scope: {:?}", scope.in_scope())]),
                )
            })?,
            If(test, ifcase, elsecase) => {
                let test_val = get!(self.eval_in(scope, &*test)?);
                let eval_result = if self.as_bool(test_val, test.span)? {
                    self.eval_block_in(&ifcase, scope)?
                } else if let Some(block) = elsecase {
                    self.eval_block_in(&block, scope)?
                } else {
                    BlockEvalResult::LocalRet(Value::Null)
                };
                get!(eval_result)
            }
            Block(b) => {
                let bval = self.eval_block_in(&b, scope)?;
                get!(bval)
            }
            Array(x) => {
                let mut ret = Vec::with_capacity(x.len());
                for i in x {
                    ret.push(get!(self.eval_in(scope, i)?));
                }

                let key = self.heap.insert(BigValue::Array(ret));
                Value::Complex(key)
            }
            ArrayAccess(arr_s, idx_s) => {
                let array_val = get!(self.eval_in(scope, arr_s)?);

                // Nessesary for BorrowCk, as we cant borrow from the heap while we evaluate
                // the index, and potentialy mutate. Probably killing perf, LMAO
                let arr = self.as_array_mut(array_val, arr_s.span)?.to_owned();

                let idx_val = get!(self.eval_in(scope, idx_s)?);
                let idx = self.as_uint(idx_val, idx_s.span)?;

                // TODO: This is wrong if we mutate the array when evaluating the index, I think
                self.check_array_bounds(&arr, idx, array_val, idx_val, arr_s.span, idx_s.span)?;
                arr[idx]
            }
            // TODO: Nice error / fill out
            other => bail!("Unimplemented {:?}", other),
        }))
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
                            vs.secondary_label()
                                .with_message(format!("Evaluated to {:?}", self.dbg_val(&v))),
                        ]),
                )
                .into())
            }
        })
    }

    pub(crate) fn check_array_bounds(
        &self,
        array: &[Value],
        idx: usize,
        array_val: Value,
        idx_val: Value,
        array_span: Span,
        idx_span: Span,
    ) -> Result<()> {
        if array.len() <= idx {
            Err(RtError(
                Diagnostic::error()
                    .with_message(format!(
                        "Array index out of bound, len is `{}`, but got `{}`",
                        array.len(),
                        idx
                    ))
                    .with_labels(vec![
                        idx_span
                            .primary_label()
                            .with_message(format!("Evaluated to {:?}", self.dbg_val(&idx_val))),
                        array_span
                            .primary_label()
                            .with_message(format!("Evaluated to {:?}", self.dbg_val(&array_val))),
                    ]),
            )
            .into())
        } else {
            Ok(())
        }
    }

    // TODO: DRY these methods
    fn as_bool(&self, val: Value, s: Span) -> Result<bool> {
        if let Value::Bool(b) = val {
            Ok(b)
        } else {
            Err(RtError(
                Diagnostic::error()
                    .with_message(format!("Expected `bool`, got `{}`", self.type_name(&val)))
                    .with_labels(vec![s
                        .primary_label()
                        .with_message(format!("Evaluated to `{:?}`", self.dbg_val(&val)))]),
            )
            .into())
        }
    }

    fn as_uint(&self, val: Value, s: Span) -> Result<usize> {
        if let Value::Int(i) = val {
            if let Ok(u) = i.try_into() {
                Ok(u)
            } else {
                Err(RtError(
                    Diagnostic::error()
                        .with_message("Expected a positive number")
                        .with_labels(vec![s
                            .primary_label()
                            .with_message(format!("Evaluated to `{}`", i))]),
                )
                .into())
            }
        } else {
            Err(RtError(
                Diagnostic::error()
                    .with_message(format!("Expected `int`, got `{}`", self.type_name(&val)))
                    .with_labels(vec![s
                        .primary_label()
                        .with_message(format!("Evaluated to `{:?}`", self.dbg_val(&val)))]),
            )
            .into())
        }
    }
    fn as_array_mut(&mut self, val: Value, s: Span) -> Result<&mut [Value]> {
        if let Value::Complex(id) = val {
            if let BigValue::Array(_) = &self.heap[id] {
                // Fun polonious workaround
                match &mut self.heap[id] {
                    BigValue::Array(a) => return Ok(a),
                    _ => unreachable!(),
                }
            }
        }
        Err(self.not_array_error(val, s).into())
    }

    // TODO: Unify
    fn not_array_error(&self, val: Value, s: Span) -> RtError {
        RtError(
            Diagnostic::error()
                .with_message(format!("Expected `array`, got `{}`", self.type_name(&val)))
                .with_labels(vec![s
                    .primary_label()
                    .with_message(format!("Evaluated_to `{:?}`", self.dbg_val(&val)))]),
        )
    }

    fn as_array(&self, val: Value, s: Span) -> Result<&[Value]> {
        if let Value::Complex(id) = val {
            if let BigValue::Array(_) = &self.heap[id] {
                // Fun polonious workaround
                match &self.heap[id] {
                    BigValue::Array(a) => return Ok(a),
                    _ => unreachable!(),
                }
            }
        }
        Err(self.not_array_error(val, s).into())
    }

    pub(crate) fn dbg_val<'v>(&'v self, v: &'v Value) -> ValueDbg<'v> {
        ValueDbg { v, e: self }
    }

    fn print_value(&self, v: &Value) {
        let dbg = self.dbg_val(v);
        println!("{}", dbg);
    }

    fn type_name(&self, v: &Value) -> &'static str {
        match v {
            Value::Int(_) => "int",
            Value::Float(_) => "float",
            Value::Bool(_) => "bool",
            Value::Complex(id) => match self.heap[*id] {
                BigValue::String(_) => "string",
                BigValue::Array(_) => "array",
            },
            Value::Null => "null",
        }
    }
}
