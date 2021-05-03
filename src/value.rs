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

impl Value {}
