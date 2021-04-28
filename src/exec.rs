/// Tree walk interpriter
use std::collections::HashMap;

use codespan_reporting::diagnostic::Diagnostic;
use eyre::{bail, Result};

use crate::ast::{
    self, BinOp, Block, BlockType, Expr, Function, Item, Program, RawExpr, Span, Spanned, Stmt,
};
use crate::diagnostics::RtError;
use crate::value::Value;

// Err -> Exit err due to type error/rt error
// Ok(false) -> Exit sucess
// Ok(true) -> Exit err due to user code request
pub fn run(p: Program) -> Result<bool> {
    // If nothing failed, we suceed
    let env = Env::new(&p)?;

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
        x => bail!("`main` returned `{:?}`, expected number or Null", x),
    };

    Ok(is_fail)
}

// TODO: unify env and scope, by understanding scheme
#[derive(Debug, Default)]
struct Env<'a> {
    functions: HashMap<&'a str, &'a Spanned<Function<'a>>>,
}

#[derive(Debug, Default)]
struct Scope<'a> {
    vars: HashMap<&'a str, Value>,
}

enum BlockEvalResult {
    FnRet(Value),
    LocalRet(Value),
}

// Note: always get! an `BlockEvalResult` before creating another to avoid #29
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
                                // TODO: Make this a method on Spaned type
                                old_fn.secondary_label().with_message("Defined once here"),
                                f.secondary_label().with_message("Defined again here"),
                            ]);
                        return Err(RtError(err).into());
                    }
                }
            };
        }

        Ok(Self { functions })
    }

    pub fn call(&self, name: ast::Name, args: &[Value], call_span: Span) -> Result<Value> {
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
            // TODO: less cloning
            scope.vars.insert(&name.name, val.clone());
        }

        // In functions, a trailing expression returns
        Ok(match self.eval_block_in(&function.body, &mut scope)? {
            BlockEvalResult::FnRet(x) => x,
            BlockEvalResult::LocalRet(x) => x,
        })
    }

    fn eval_block_in(&self, block: &Block<'a>, scope: &mut Scope<'a>) -> Result<BlockEvalResult> {
        use Stmt::*;

        let mut last_val = Value::Null;
        for i in &block.0 {
            match i {
                Let(name, expr) => {
                    let val = get!(self.eval_in(scope, expr)?);
                    scope.vars.insert(name, val);
                    last_val = Value::Null;
                }
                Print(e) => {
                    let val = get!(self.eval_in(scope, e)?);
                    print_value(&val);
                    last_val = Value::Null;
                }
                Return(e) => {
                    let val = get!(self.eval_in(scope, e)?);
                    return Ok(BlockEvalResult::FnRet(val));
                }
                Expr(e) => {
                    let e = get!(self.eval_in(scope, e)?);
                    last_val = e;
                }
            }
        }

        let ret = match block.1 {
            BlockType::Discard => Value::Null,
            BlockType::ReturnExpr => last_val,
        };

        Ok(BlockEvalResult::LocalRet(ret))
    }

    fn eval_in(&self, scope: &mut Scope<'a>, e: &Expr<'a>) -> Result<BlockEvalResult> {
        use crate::ast::Literal;
        use RawExpr::*;

        Ok(BlockEvalResult::LocalRet(match &e.node {
            Literal(l) => match l.node {
                Literal::String(s) => Value::String((*s).to_owned()),
                Literal::Float(f) => Value::Float(f),
                Literal::Integer(i) => Value::Int(i),
                Literal::Bool(b) => Value::Bool(b),
                Literal::Null => Value::Null,
            },
            BinOp(l, o, r) => {
                // TODO: is this eval right
                let lv = get!(self.eval_in(scope, &l)?);
                let rv = get!(self.eval_in(scope, &r)?);
                binop(lv, o.node, rv, l.span, o.span, r.span)?
            }
            Call(function, args) => {
                if let RawExpr::Var(name) = function.node {
                    let mut args_evald = Vec::with_capacity(args.len());
                    for i in args {
                        args_evald.push(get!(self.eval_in(scope, &i)?));
                    }
                    self.call(name, &args_evald, e.span)?
                } else {
                    // TODO: Nice error
                    bail!("Expeced {:?} to be a plain var", function)
                }
            }
            Var(Spanned { node, span }) => scope
                .vars
                .get(node)
                .ok_or_else(|| {
                    RtError(
                        Diagnostic::error()
                            .with_message(format!("Couldn't find variable `{}` in scope", node))
                            .with_labels(vec![span.primary_label()])
                            .with_notes(vec![format!("Variables in scope: {:?}", {
                                let mut items = scope.vars.keys().collect::<Vec<_>>();
                                items.sort();
                                items
                            })]),
                    )
                })?
                .clone(),
            If(test, ifcase, elsecase) => {
                let test = get!(self.eval_in(scope, &*test)?);
                let eval_result = if is_truthy(test)? {
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
            // TODO: Nice error
            other => bail!("Unimplemented {:?}", other),
        }))
    }
}

macro_rules! binop_match {
    (
        $bindings:expr,
        ($l_span:expr, $o_span:expr, $r_span:expr),
        // Integer math ops
        { $($math_op_name:path => $math_op:tt),*$(,)? },
        // Equality
        { $($eq_type:path),*$(,)? },
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
                (BinOp::Equals, $eq_type(l), $eq_type(r)) => Bool(l == r),
                // TODO: Should we say 1 != 1.0, what about "true" != 3
                (BinOp::NotEquals, $eq_type(l), $eq_type(r)) => Bool(l != r),
            )*
            $(
                ($comarison_name, Int(l), Int(r)) => Bool(l $comparison_op r),
                ($comarison_name, Float(l), Float(r)) => Bool(l $comparison_op r),
            )*

            $( $user_lhs => $user_rhs, )*

            (o, l, r) => Err(binop_err(l,o,r,$l_span,$o_span,$r_span))?,
        }
    };
}

fn binop_err(l: Value, o: BinOp, r: Value, l_span: Span, o_span: Span, r_span: Span) -> RtError {
    RtError(
        Diagnostic::error()
            .with_message(format!(
                "Unknown binop `{}`, for `{}` and `{}`",
                o,
                l.type_name(),
                r.type_name()
            ))
            .with_labels(vec![
                o_span.primary_label().with_message("In this operator"),
                l_span
                    .secondary_label()
                    .with_message(format!("LHS evaluated to {:?}", l)),
                r_span
                    .secondary_label()
                    .with_message(format!("LHS evaluated to {:?}", r)),
            ]),
    )
}

fn binop(l: Value, o: BinOp, r: Value, l_span: Span, o_span: Span, r_span: Span) -> Result<Value> {
    use Value::*;

    Ok(binop_match!(
        (o, l, r),
        (l_span, o_span, r_span),
        {
            BinOp::Plus   => +,
            BinOp::Minus  => -,
            BinOp::Times  => *,
            BinOp::Devide => /,
        },
        {
            // TODO: What should null == null return
            String, Int, Float, Bool
        },
        {
            BinOp::GreaterThan       => >,
            BinOp::GreaterThanEquals => >=,
            BinOp::LessThan          => <,
            BinOp::LessThanEquals    => <=,
        },
        {
            (BinOp::Plus,       String(l), String(r)) => String(l + &r),
            (BinOp::LogicalOr,  Bool(l),   Bool(r))   => Bool  (l || r),
            (BinOp::LogicalAnd, Bool(l),   Bool(r))   => Bool  (l && r),
        },
    ))
}

fn is_truthy(val: Value) -> Result<bool> {
    if let Value::Bool(b) = val {
        Ok(b)
    } else {
        // TODO: Nice error
        bail!("Not a boolean: {:?}", val)
    }
}

// TODO: This should return a string
fn print_value(v: &Value) {
    use Value::*;
    match v {
        String(s) => println!("{}", s),
        Int(x) => println!("{}", x),
        Float(f) => println!("{}", f),
        Null => println!("null"),
        Bool(b) => println!("{}", b),
    }
}
