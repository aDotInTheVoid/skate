/// Tree walk interpriter
use std::collections::HashMap;

use eyre::{bail, Result};

use crate::ast::{BinOp, Expr, Function, Item, Program, Stmt};

use serde_json::Value;

pub fn run(p: Program) -> Result<i64> {
    // If nothing failed, we suceed
    let env = Env::new(&p);

    let result = env.call("main", &[])?;

    // TODO: Figure out exit codes 0/1/101
    let exit_code = match result {
        Value::Null => 0,
        Value::Number(x) => x.as_i64().unwrap_or(101),
        _ => 101,
    };

    Ok(exit_code)
}

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
        let mut functions = HashMap::new();
        for i in p {
            match i {
                Item::Function((f, _)) => assert!(functions.insert(f.name.0, f).is_none()),
            };
        }

        Self { functions }
    }

    pub fn call(&self, fn_name: &str, args: &[Value]) -> Result<Value> {
        let function = self.functions[fn_name];
        assert_eq!(args.len(), function.args.0.len());
        let mut scope = Scope::default();

        for (name, val) in function.args.0.iter().zip(args.iter()) {
            // TODO: less cloning
            scope.vars.insert(name.name, val.clone());
        }

        // In functions, a trailing expression returns
        Ok(match self.eval_block_in(&function.body.0, &mut scope)? {
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
                    scope.vars.insert(*name, val);
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
        use Expr::*;

        Ok(BlockEvalResult::LocalRet(match e {
            Literal(l) => match l {
                Literal::String(s) => Value::String((*s).to_owned()),
                Literal::Float(f) => Value::Number(serde_json::Number::from_f64(*f).unwrap()),
                Literal::Integer(i) => {
                    Value::Number(serde_json::Number::from_f64(*i as f64).unwrap())
                }
                Literal::Bool(b) => Value::Bool(*b),
            },
            BinOp(l, o, r) => {
                let l = self.eval_in(scope, l)?;
                let r = self.eval_in(scope, r)?;
                binop(get!(l), *o, get!(r))?
            }
            Call(function, args) => {
                if let Expr::Var(name) = **function {
                    let mut args_evald = Vec::with_capacity(args.len());
                    for i in args {
                        args_evald.push(get!(self.eval_in(scope, i)?));
                    }
                    self.call(name, &args_evald)?
                } else {
                    bail!("Expeced {:?} to be a plain var", function)
                }
            }
            Var(name) => scope.vars.get(name).unwrap().clone(),
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
        (o, l, r) => bail!("Unknown binop {:?}, {:?}, {:?}", l, o, r),
    })
}

fn is_truthy(val: Value) -> Result<bool> {
    if let Value::Bool(b) = val {
        Ok(b)
    } else {
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
