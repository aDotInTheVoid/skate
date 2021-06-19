use std::collections::BTreeMap;
use std::fmt::{self, Debug, Display, Formatter, Write};

use slotmap::SlotMap;

pub type Heap = SlotMap<HeapKey, BigValue>;

slotmap::new_key_type! { pub struct HeapKey; }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Copy)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    Complex(HeapKey),
    Null,
}

// TODO: Remove PartialEq impl when we add Array and Value, as these may have
// different heap ids that should be treated equal.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum BigValue {
    String(String),
    Array(Vec<Value>),
    // TODO: String innerning + gc
    Map(Map),
}

// TODO: Benchmap changing to hashmap.
// But we do this for deterministic field ordering
pub type Map = BTreeMap<String, Value>;

// Wrapper to properly implemt Debug for a Value, taking into accound the Heap
// This will need to be changed when heep values can access other heap vals
// (Array and Map), but is fine for string.
pub struct ValueDbg<'a> {
    pub v: &'a Value,
    pub heap: &'a Heap,
}

impl Debug for ValueDbg<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Value::Complex(id) = self.v {
            if let BigValue::String(s) = &self.heap[*id] {
                return Debug::fmt(s, f);
            }
        }
        Display::fmt(self, f)
    }
}

// Even when displaying an array, the items are in debug view
fn print_array(f: &mut Formatter, a: &[Value], heap: &Heap) -> fmt::Result {
    let mut f = f.debug_list();
    for v in a {
        f.entry(&ValueDbg { v, heap });
    }
    f.finish()
}

fn print_map(f: &mut Formatter, m: &Map, heap: &Heap) -> fmt::Result {
    f.write_char('%')?;
    let mut f = f.debug_map();
    for (k, v) in m {
        f.entry(&DebugAsDisplay(k), &ValueDbg { v, heap });
    }
    f.finish()
}

impl Display for ValueDbg<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.v {
            Value::Int(v) => Display::fmt(v, f),
            Value::Float(v) => Display::fmt(v, f),
            Value::Bool(v) => Display::fmt(v, f),
            Value::Complex(id) => match &self.heap[*id] {
                BigValue::String(v) => Display::fmt(&v, f),
                BigValue::Array(a) => print_array(f, a, self.heap),
                BigValue::Map(m) => print_map(f, m, self.heap),
            },
            Value::Null => f.write_str("null"),
        }
    }
}

#[derive(Hash, PartialEq, Eq)]
struct DebugAsDisplay<T>(T);

impl<T: Display> Debug for DebugAsDisplay<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[test]
fn sizeof_value() {
    use std::mem::size_of;

    // The are common, so keep them small
    assert_eq!(size_of::<Value>(), 8 * 2);
    assert_eq!(size_of::<HeapKey>(), 8);
    assert_eq!(size_of::<BigValue>(), 4 * 8);
}
