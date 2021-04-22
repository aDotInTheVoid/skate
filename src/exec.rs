/// Tree walk interpriter
use std::collections::HashMap;

use eyre::{bail, eyre, Result};

use crate::ast::{BinOp, Expr, Function, Item, Program, RawExpr, Spanned, Stmt};

use serde_json::Value;

// Err -> Exit err due to type error/rt error
// Ok(false) -> Exit sucess
// Ok(true) -> Exit err due to user code request
pub fn run(p: Program) -> Result<bool> {
    // If nothing failed, we suceed
    let env = Env::new(&p)?;

    let result = env.call("main", &[])?;

    // TODO: Figure out exit codes 0/1/101
    // One for script exited with error, one for IIE
    let is_fail = match result {
        Value::Null => false,
        Value::Number(x) => {
            // TODO: Sort out numbers (probably move away from json)
            x.as_f64() != Some(0.0)
        }
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

macro_rules! get {
    ($e: expr) => {
        match $e {
            BlockEvalResult::LocalRet(x) => x,
            x @ BlockEvalResult::FnRet(_) => return Ok(x),
        }
    };
}

impl<'a> Env<'a> {
    pub fn new(p: &'a Program) -> eyre::Result<Self> {
        let mut functions: HashMap<&str, &Spanned<Function>> = HashMap::new();
        for i in p {
            match i {
                Item::Function(f) => {
                    if let Some(old_fn) = functions.insert(&f.name, &f) {
                        // TODO: Should these be the function span of the name span
                        bail!(
                            "Duplicated function `{} defined {:?} and {:?}`",
                            &f.name.node,
                            f.span,
                            old_fn.span
                        )
                    }
                }
            };
        }

        Ok(Self { functions })
    }

    pub fn call(&self, fn_name: &str, args: &[Value]) -> Result<Value> {
        // TODO: nice error if no function found
        let function = self
            .functions
            .get(fn_name)
            .ok_or_else(|| eyre!("No function with name `{}`", fn_name))?;

        // TODO: Nice errors
        if args.len() != function.args.len() {
            bail!(
                "Calling `{}`: Expected {} args, found {}",
                fn_name,
                function.args.len(),
                args.len()
            )
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

    fn eval_block_in(&self, block: &[Stmt<'a>], scope: &mut Scope<'a>) -> Result<BlockEvalResult> {
        use Stmt::*;

        let mut last_val = Value::Null;
        for i in block {
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
        Ok(BlockEvalResult::LocalRet(last_val))
    }

    fn eval_in(&self, scope: &mut Scope<'a>, e: &Expr<'a>) -> Result<BlockEvalResult> {
        use crate::ast::Literal;
        use RawExpr::*;

        Ok(BlockEvalResult::LocalRet(match &e.node {
            Literal(l) => match l.node {
                Literal::String(s) => Value::String((*s).to_owned()),
                Literal::Float(f) => Value::Number(serde_json::Number::from_f64(f).unwrap()),
                Literal::Integer(i) => {
                    Value::Number(serde_json::Number::from_f64(i as f64).unwrap())
                }
                Literal::Bool(b) => Value::Bool(b),
            },
            BinOp(l, o, r) => {
                let l = self.eval_in(scope, &l)?;
                let r = self.eval_in(scope, &r)?;
                binop(get!(l), o.node, get!(r))?
            }
            Call(function, args) => {
                if let RawExpr::Var(name) = function.node {
                    let mut args_evald = Vec::with_capacity(args.len());
                    for i in args {
                        args_evald.push(get!(self.eval_in(scope, &i)?));
                    }
                    self.call(&name, &args_evald)?
                } else {
                    // TODO: Nice error
                    bail!("Expeced {:?} to be a plain var", function)
                }
            }
            Var(Spanned { node, .. }) => scope
                .vars
                .get(node)
                .ok_or_else(|| eyre!("No var in scope with name `{}`", node))?
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

fn binop(l: Value, o: BinOp, r: Value) -> Result<Value> {
    use Value::*;
    Ok(match (o, l, r) {
        (BinOp::Plus, String(l), String(r)) => String(l + &r),
        (BinOp::Plus, Number(l), Number(r)) => {
            Number(serde_json::Number::from_f64(l.as_f64().unwrap() + r.as_f64().unwrap()).unwrap())
        }
        (BinOp::Times, Number(l), Number(r)) => {
            Number(serde_json::Number::from_f64(l.as_f64().unwrap() * r.as_f64().unwrap()).unwrap())
        }
        (BinOp::Equals, l, r) => Bool(l == r),
        (BinOp::LogicalOr, Bool(l), Bool(r)) => Bool(l || r),
        (BinOp::Minus, Number(l), Number(r)) => {
            let l = l.as_f64().unwrap();
            let r = r.as_f64().unwrap();
            Number(serde_json::Number::from_f64(l - r).unwrap())
        }
        (BinOp::GreaterThanEquals, Number(l), Number(r)) => {
            let l = l.as_f64().unwrap();
            let r = r.as_f64().unwrap();
            let e = l >= r;
            Bool(e)
        }
        // Base case
        // TODO: Nice error
        (o, l, r) => bail!("Unknown binop {:?}, {:?}, {:?}", l, o, r),
    })
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
        Number(n) => {
            if let Some(u) = n.as_u64() {
                println!("{}", u);
            } else if let Some(i) = n.as_i64() {
                println!("{}", i)
            } else if let Some(f) = n.as_f64() {
                println!("{}", f)
            } else {
                println!("Nan")
            }
        }
        // TODO: Do better for these cases
        Array(x) => println!("{:?}", x),
        Object(o) => println!("{:?}", o),

        Null => println!("null"),
        Bool(b) => println!("{}", b),
    }
}
