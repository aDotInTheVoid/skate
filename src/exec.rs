/// Tree walk interpriter
use std::collections::HashMap;

use eyre::{bail, eyre, Result};

use crate::ast::{BinOp, Block, BlockType, Expr, Function, Item, Program, RawExpr, Spanned, Stmt};

use crate::value::Value;

pub fn run(p: Program) -> Result<i64> {
    // If nothing failed, we suceed
    let env = Env::new(&p);

    let result = env.call("main", &[])?;

    // TODO: Figure out exit codes 0/1/101
    // One for script exited with error, one for IIE
    let exit_code = match result {
        Value::Null => 0,
        Value::Int(x) => x,
        _ => 101,
    };

    Ok(exit_code)
}

// TODO: unify env and scope, by understanding scheme
#[derive(Debug, Default)]
struct Env<'a> {
    functions: HashMap<&'a str, &'a Function<'a>>,
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
    pub fn new(p: &'a Program) -> Self {
        let mut functions: HashMap<&str, &Function> = HashMap::new();
        for i in p {
            match i {
                Item::Function(f) => assert!(functions.insert(&f.name, &f).is_none()),
            };
        }

        Self { functions }
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
        // TODO: Flesh this out, and preferably without repitition
        (BinOp::Plus, String(l), String(r)) => String(l + &r),

        (BinOp::Plus, Int(l), Int(r)) => Int(l + r),
        (BinOp::Plus, Float(l), Float(r)) => Float(l + r),
        (BinOp::Minus, Int(l), Int(r)) => Int(l - r),
        (BinOp::Minus, Float(l), Float(r)) => Float(l - r),
        (BinOp::Times, Int(l), Int(r)) => Int(l * r),
        (BinOp::Times, Float(l), Float(r)) => Float(l * r),
        (BinOp::Devide, Int(l), Int(r)) => Int(l / r),
        (BinOp::Devide, Float(l), Float(r)) => Float(l / r),
        (BinOp::GreaterThanEquals, Int(l), Int(r)) => Bool(l >= r),

        (BinOp::Equals, Bool(l), Bool(r)) => Bool(l == r),
        (BinOp::Equals, Int(l), Int(r)) => Bool(l == r),

        (BinOp::LogicalOr, Bool(l), Bool(r)) => Bool(l || r),

        // TODO: Nicer error
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
        Int(x) => println!("{}", x),
        Float(f) => println!("{}", f),
        Null => println!("null"),
        Bool(b) => println!("{}", b),
    }
}
