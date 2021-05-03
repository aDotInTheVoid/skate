use std::fmt::Debug;

use crate::env::Env;

slotmap::new_key_type! { pub struct HeapKey; }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    Complex(HeapKey),
    Null,
}

// TODO: Remove PartialEq impl when we add Array and Value, as these may have
// different heap ids that should be treated equal.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum BigValue {
    String(String),
    // Array(Vec<Value>),
    // Map(HashMap<String, Value>),
}

// Wrapper to properly implemt Debug for a Value, taking into accound the Heap
// This will need to be changed when heep values can access other heap vals
// (Array and Map), but is fine for string.
pub(crate) struct ValueDbg<'a> {
    pub v: &'a Value,
    pub e: &'a Env<'a>,
}

impl Debug for ValueDbg<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Value::Complex(id) = self.v {
            self.e.heap[*id].fmt(f)
        } else {
            self.v.fmt(f)
        }
    }
}
