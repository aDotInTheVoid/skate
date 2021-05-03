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

impl Value {}
