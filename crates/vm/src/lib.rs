use std::io;

use bytecode::Instr;
use eyre::Result;

use compiler::bytecode;
use parser::Literal;
use rt_common::RT;
use value::{BigValue, Heap, HeapKey, Value, ValueDbg};

const DEBUG: bool = false;

// We put Code outside the VM for BorrowCK reasons
pub struct VM<'w> {
    output: &'w mut dyn io::Write,
    stack: Vec<Value>,
    heap: Heap,
}

impl<'w> VM<'w> {
    pub fn new(output: &'w mut dyn io::Write) -> Self {
        Self {
            output,
            stack: Default::default(),
            heap: Default::default(),
        }
    }

    pub fn stack_len(&self) -> usize {
        self.stack.len()
    }

    pub fn run(&mut self, code: bytecode::Code, main_key: bytecode::FuncKey) -> Result<()> {
        let main = &code.fns[main_key];

        let mut ip = 0;

        if DEBUG {
            for (idx, el) in main.code.iter().enumerate() {
                eprintln!("{} {:?}", idx, el);
            }
            eprintln!();
        }

        while let Some(instr) = main.code.get(ip) {
            if DEBUG {
                eprintln!(
                    "{:?}",
                    value::StackDbg {
                        stack: &self.stack,
                        heap: &self.heap
                    }
                );
                eprintln!("{:?}", instr);
                eprintln!();
            }

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
                Instr::GetLocal(id) => {
                    self.push(self.stack[*id]);
                }
                Instr::SetLocal(id) => self.stack[*id] = self.peak(),
                // Compensate for universal increment
                Instr::JumpForward(by) => {
                    debug_assert_ne!(*by, 0);
                    ip += by;
                }
                Instr::JumpBackward(by) => {
                    debug_assert_ne!(*by, 0);
                    ip -= by;
                }
                Instr::JumpForwardIfFalse(by) => {
                    debug_assert_ne!(*by, 0);
                    let val = self.peak();
                    if !self.as_bool(val, Default::default())? {
                        ip += by;
                    }
                }
                Instr::MakeArray(num) => {
                    let ar = self.stack.split_off(self.stack.len() - num);
                    let hid = self.add_to_heap(BigValue::Array(ar));
                    self.push(Value::Complex(hid));
                }
                Instr::Return => {
                    // TODO
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

    fn peak(&mut self) -> Value {
        *self.stack.last().unwrap()
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
    let code = "
    fn main() {
    print 2 + (7 / 5) * 13;
    }";
    let prog = parser::ProgramParser::new()
        .parse(diagnostics::FileId(0), code)
        .unwrap();

    let mut stdout = Vec::new();

    let mut vm = VM::new(&mut stdout);

    let (code, main_fn) = compiler::compile(&prog);

    vm.run(code, main_fn).unwrap();
    assert_eq!(vm.stack.len(), 0);
    assert_eq!(String::from_utf8(stdout).unwrap(), "15\n");
}

#[test]
fn print() {
    let code = "
    fn main() {
    print \"hello\" + \" \" + \"world\";
    }";
    let prog = parser::ProgramParser::new()
        .parse(diagnostics::FileId(0), code)
        .unwrap();

    let mut stdout = Vec::new();

    let mut vm = VM::new(&mut stdout);

    let (code, main_fn) = compiler::compile(&prog);

    vm.run(code, main_fn).unwrap();
    assert_eq!(vm.stack.len(), 0);
    assert_eq!(String::from_utf8(stdout).unwrap(), "hello world\n");
}

#[test]
fn empty_stack() {
    let code = "
    fn main() {
    2 * 3 + 31 - 1;
    }";
    let prog = parser::ProgramParser::new()
        .parse(diagnostics::FileId(0), code)
        .unwrap();

    let mut stdout = Vec::new();

    let mut vm = VM::new(&mut stdout);

    let (code, main_fn) = compiler::compile(&prog);

    vm.run(code, main_fn).unwrap();
    assert_eq!(vm.stack.len(), 0);
    assert_eq!(stdout.len(), 0);
}
