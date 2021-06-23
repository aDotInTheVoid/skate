use std::io;

use compiler::bytecode;
use parser::Literal;
use value::{BigValue, Heap, HeapKey, Value};

// We put Code outside the VM for BorrowCK reasons
pub struct VM<'w> {
    output: &'w dyn io::Write,
    // TODO: Should this be on bytecode::Code?
    main_fn: bytecode::FuncKey,
    stack: Vec<Value>,
    heap: Heap,
}

impl VM<'_> {
    fn run(&mut self, code: bytecode::Code, main_key: bytecode::FuncKey) {
        let main = &code.fns[main_key];

        let ip = 0;

        loop {
            match &main.code[ip] {
                bytecode::Instr::LoadLit(l) => {
                    let val = match *l {
                        Literal::String(s) => {
                            Value::Complex(self.add_to_heap(BigValue::String(s.to_owned())))
                        }
                        Literal::Float(f) => Value::Float(f),
                        Literal::Integer(i) => Value::Int(i),
                        Literal::Bool(b) => Value::Bool(b),
                        Literal::Null => Value::Null,
                    };
                    self.push(val);
                }
                bytecode::Instr::BinOp(o) => {
                    let rhs = self.pop();
                    let lhs = self.pop();
                    todo!()
                }
                bytecode::Instr::UnOp(_) => todo!(),
                bytecode::Instr::Print => todo!(),
                bytecode::Instr::Return => todo!(),
            }
        }
    }

    fn push(&mut self, val: Value) {
        self.stack.push(val);
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }

    pub fn add_to_heap(&mut self, v: BigValue) -> HeapKey {
        self.heap.insert(v)
    }
}
