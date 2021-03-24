use std::collections::HashMap;

use eyre::Result;

use crate::ast::{Function, Item, Program};

pub fn run(p: Program) -> Result<i64> {
    // If nothing failed, we suceed
    let mut env = Env::new(&p);

    let result = env.call("main", &[])?.to_int()?;

    Ok(result)
}

struct Env<'a> {
    functions: HashMap<&'a str, &'a Function<'a>>,
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
        eyre::bail!("No")
    }
}

enum Value {}

impl Value {
    pub fn to_int(self) -> Result<i64> {
        match self {}
    }
}
