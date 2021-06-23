use std::io;

use eyre::Result;

use compiler::bytecode;
use parser::{Literal, RawExpr};
use rt_common::RT;
use value::{BigValue, Heap, HeapKey, Value};

// We put Code outside the VM for BorrowCK reasons
pub struct VM<'w> {
    output: &'w dyn io::Write,
    stack: Vec<Value>,
    heap: Heap,
}

impl VM<'_> {
    fn run(&mut self, code: bytecode::Code, main_key: bytecode::FuncKey) -> Result<()> {
        let main = &code.fns[main_key];

        let mut ip = 0;

        while let Some(instr) = main.code.get(ip) {
            match instr {
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

                    // In the fast path, we dont encounter a type error, and
                    // therefor dont need to access spans. We should only do
                    // this load once we know its type error, for better cache.
                    // TODO: do this.
                    let ast = main.spans[ip];

                    // TODO: Derive all these
                    let (ls, os, rs) = if let RawExpr::BinOp(ls, os, rs) = &**ast {
                        (ls, os, rs)
                    } else {
                        panic!()
                    };

                    let val = self.binop(lhs, *o, rhs, ls.span, os.span, rs.span)?;
                    self.push(val)
                }
                bytecode::Instr::UnOp(_) => todo!(),
                bytecode::Instr::Print => todo!(),
                bytecode::Instr::Return => todo!(),
            }
            ip += 1;
        }

        Ok(())
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

impl RT for VM<'_> {
    fn heap(&self) -> &value::Heap {
        &self.heap
    }

    fn heap_mut(&mut self) -> &mut value::Heap {
        &mut self.heap
    }
}

#[test]
fn basic() {
    let code = "2 + (7 / 5) * 13";
    let expr = parser::ExprParser::new()
        .parse(diagnostics::FileId(0), code)
        .unwrap();
    let mut func = compiler::FnComping::default();
    func.push_expr(&expr);
    let mut code = bytecode::Code::default();
    let main_fn = code.fns.insert(func.output);
    let mut vm = VM {
        output: &mut vec![],
        stack: vec![],
        heap: Heap::with_key(),
    };
    vm.run(code, main_fn).unwrap();

    assert_eq!(vm.stack.len(), 1);
    assert_eq!(format!("{}", vm.dbg_val(&vm.stack[0])), "15");
}
