use std::io;

use bytecode::Instr;
use eyre::Result;

use compiler::bytecode;
use parser::Literal;
use rt_common::RT;
use value::{BigValue, Heap, HeapKey, Value, ValueDbg};

// We put Code outside the VM for BorrowCK reasons
pub struct VM<'w> {
    output: &'w mut dyn io::Write,
    stack: Vec<Value>,
    heap: Heap,
}

impl VM<'_> {
    pub fn run(&mut self, code: bytecode::Code, main_key: bytecode::FuncKey) -> Result<()> {
        let main = &code.fns[main_key];

        let mut ip = 0;

        while let Some(instr) = main.code.get(ip) {
            match instr {
                Instr::LoadLit(l) => {
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
                Instr::BinOp(o) => {
                    let rhs = self.pop();
                    let lhs = self.pop();

                    // In the fast path, we dont encounter a type error, and
                    // therefor dont need to access spans. We should only do
                    // this load once we know its type error, for better cache.
                    // TODO: do this.
                    let ast = *main.spans[ip].as_expr().unwrap();

                    let (ls, os, rs) = ast.as_bin_op().unwrap();

                    let val = self.binop(lhs, *o, rhs, ls.span, os.span, rs.span)?;
                    self.push(val)
                }
                Instr::UnOp(_) => todo!(),
                Instr::Print => {
                    let val = self.pop();
                    self.print_value(&val)?;
                }
                Instr::Pop => {
                    self.pop();
                }
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

    // This cant be in rt_common, as it cant know that the borrow of
    // self.heap and self.output are disjoint if it gets them through
    // methods, and not struct fields.
    pub(crate) fn print_value(&mut self, v: &Value) -> io::Result<()> {
        let dbg = ValueDbg {
            v,
            heap: &self.heap,
        };
        writeln!(self.output, "{}", dbg)?;
        Ok(())
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

#[test]
fn print() {
    let code = "print \"hello\" + \" \" + \"world\";";
    let stmt = parser::StmtParser::new()
        .parse(diagnostics::FileId(0), code)
        .unwrap();
    let mut func = compiler::FnComping::default();
    func.push_stmt(&stmt);
    let mut code = bytecode::Code::default();
    let main_fn = code.fns.insert(func.output);
    let mut stdout = Vec::new();
    let mut vm = VM {
        output: &mut stdout,
        stack: vec![],
        heap: Heap::with_key(),
    };

    vm.run(code, main_fn).unwrap();
    assert_eq!(vm.stack.len(), 0);
    assert_eq!(String::from_utf8(stdout).unwrap(), "hello world\n");
}

#[test]
fn empty_stack() {
    let code = "2 * 3 + 31 - 1;";
    let stmt = parser::StmtParser::new()
        .parse(diagnostics::FileId(0), code)
        .unwrap();
    let mut func = compiler::FnComping::default();
    func.push_stmt(&stmt);
    let mut code = bytecode::Code::default();
    let main_fn = code.fns.insert(func.output);
    let mut stdout = Vec::new();
    let mut vm = VM {
        output: &mut stdout,
        stack: vec![],
        heap: Heap::with_key(),
    };

    vm.run(code, main_fn).unwrap();
    assert_eq!(vm.stack.len(), 0);
}
