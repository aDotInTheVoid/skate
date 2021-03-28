use std::collections::HashMap;

use eyre::{bail, Result};

use crate::ast::{BinOp, Expr, Function, Item, Program};

use serde_json::Value;

pub fn run(p: Program) -> Result<i64> {
    // If nothing failed, we suceed
    let mut env = Env::new(&p);

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

impl<'a> Env<'a> {
    pub fn new(p: &'a Program) -> Self {
        let mut functions = HashMap::new();
        for i in p {
            match i {
                Item::Function(f) => assert!(functions.insert(f.name, f).is_none()),
            };
        }

        Self { functions }
    }

    // TODO: Should this take &mut
    pub fn call(&mut self, fn_name: &str, args: &[Value]) -> Result<Value> {
        let function = self.functions[fn_name];
        assert_eq!(args.len(), function.args.len());
        let mut scope = Scope::default();

        for (name, val) in function.args.iter().zip(args.iter()) {
            // TODO: less cloning
            scope.vars.insert(name.name, val.clone());
        }

        use crate::ast::Stmt::*;

        for i in &function.body {
            match i {
                Let(name, expr) => {
                    let val = self.eval_in(&mut scope, expr)?;
                    scope.vars.insert(*name, val);
                }
                Print(e) => {
                    let val = self.eval_in(&mut scope, e)?;
                    print_value(&val);
                }
                // TODO: only show the stmt type, not the whole expr
                other => bail!("Unimplemented {:?}", other),
            }
        }

        // Default exit value
        Ok(Value::Null)
    }

    fn eval_in(&self, scope: &mut Scope, e: &Expr) -> Result<Value> {
        use crate::ast::Literal;
        use Expr::*;
        Ok(match e {
            Literal(l) => match l {
                Literal::String(s) => Value::String((*s).to_owned()),
                Literal::Float(f) => Value::Number(serde_json::Number::from_f64(*f).unwrap()),
                Literal::Integer(i) => {
                    Value::Number(serde_json::Number::from_f64(*i as f64).unwrap())
                }
            },
            BinOp(l, o, r) => {
                let l = self.eval_in(scope, l)?;
                let r = self.eval_in(scope, r)?;
                binop(l, *o, r)?
            }
            Var(name) => scope.vars.get(name).unwrap().clone(),
            other => bail!("Unimplemented {:?}", other),
        })
    }
}

fn binop(l: Value, o: BinOp, r: Value) -> Result<Value> {
    use Value::*;
    Ok(match (o, l, r) {
        (BinOp::Plus, String(l), String(r)) => String(l + &r),

        // Base case
        (o, l, r) => bail!("Unknown binop {:?}, {:?}, {:?}", l, o, r),
    })
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
