use std::collections::HashMap;

use crate::env::{Env, Scope};

slotmap::new_key_type! { pub struct HeapKey; }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    Complex(HeapKey),
    Null,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]

pub enum BigValue {
    String(String),
    // Array(Vec<Value>),
    // Map(HashMap<String, Value>),
}

impl Value {
    pub(crate) fn type_name(&self, env: &Env) -> &'static str {
        match self {
            Value::Int(_) => "int",
            Value::Float(_) => "float",
            Value::Bool(_) => "bool",
            Value::Complex(id) => match env.heap[*id] {
                BigValue::String(_) => "string",
            },
            Value::Null => "null",
        }
    }
}
