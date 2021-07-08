use std::io;

use bytecode::Instr;
use eyre::Result;

use compiler::bytecode::{self, FuncKey};
use parser::Literal;
use rt_common::RT;
use value::{BigValue, Heap, HeapKey, Value, ValueDbg};

const DEBUG: bool = true;

// We put Code outside the VM for BorrowCK reasons
pub struct VM<'w> {
    output: &'w mut dyn io::Write,
    stack: Vec<Value>,
    heap: Heap,
    frames: Vec<Frame>,
}

struct Frame {
    func: FuncKey,
    ip: usize,
    stack_offset: usize,
}

impl<'w> VM<'w> {
    pub fn new(output: &'w mut dyn io::Write) -> Self {
        Self {
            output,
            stack: Default::default(),
            heap: Default::default(),
            frames: Default::default(),
        }
    }

    pub fn stack_len(&self) -> usize {
        self.stack.len()
    }

    fn ip(&self) -> usize {
        self.frames.last().unwrap().ip
    }

    fn ip_mut(&mut self) -> &mut usize {
        &mut self.frames.last_mut().unwrap().ip
    }

    fn stack_offset(&self) -> usize {
        self.frames.last().unwrap().stack_offset
    }

    fn get_func<'b, 'a, 's>(&self, code: &'b bytecode::Code<'a, 's>) -> &'b bytecode::Func<'a, 's> {
        let key = self.frames.last().unwrap().func;
        &code.fns[key]
    }

    pub fn run(&mut self, code: &bytecode::Code, main_key: bytecode::FuncKey) -> Result<()> {
        // All of this assumes a fresh VM, which is probably bad design

        self.frames.push(Frame {
            func: main_key,
            ip: 0,
            stack_offset: 0,
        });

        // if DEBUG {
        //     for (idx, el) in main.code.iter().enumerate() {
        //         eprintln!("{} {:?}", idx, el);
        //     }
        //     eprintln!();
        // }

        loop {
            let ip = self.ip();
            let instr = &self.get_func(code).code[ip];

            *self.ip_mut() += 1;
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
                    let ast = self.get_func(code).spans[ip].as_expr().unwrap();

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
                    self.push(self.stack[self.stack_offset()..][*id]);
                }
                Instr::SetLocal(id) => self.stack[*id] = self.peak(),
                // Compensate for universal increment
                Instr::JumpForward(by) => {
                    debug_assert_ne!(*by, 0);
                    *self.ip_mut() += by;
                }
                Instr::JumpBackward(by) => {
                    debug_assert_ne!(*by, 0);
                    *self.ip_mut() -= by;
                }
                Instr::JumpForwardIfFalse(by) => {
                    debug_assert_ne!(*by, 0);
                    let val = self.peak();
                    if !self.as_bool(val, Default::default())? {
                        *self.ip_mut() += by;
                    }
                }
                Instr::MakeArray(num) => {
                    let ar = self.stack.split_off(self.stack.len() - num);
                    let hid = self.add_to_heap(BigValue::Array(ar));
                    self.push(Value::Complex(hid));
                }
                Instr::Return => {
                    let result = self.pop();

                    let frame = self.frames.pop().unwrap();
                    self.stack.truncate(frame.stack_offset);
                    self.push(result);

                    if self.frames.len() == 0 {
                        return Ok(());
                    }
                }
                Instr::Call(id) => {
                    let func = &code.fns[*id];
                    let n_args = func.n_args;
                    let frame = Frame {
                        func: *id,
                        ip: 0,
                        stack_offset: self.stack_len() - n_args,
                    };
                    self.frames.push(frame);
                }
            }
            // *self.ip_mut() += 1;
        }
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

    vm.run(&code, main_fn).unwrap();
    // The null from main
    assert_eq!(vm.stack.len(), 1);
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

    vm.run(&code, main_fn).unwrap();
    // The null from main
    assert_eq!(vm.stack.len(), 1);
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

    vm.run(&code, main_fn).unwrap();
    assert_eq!(vm.stack.len(), 1);
    assert_eq!(stdout.len(), 0);
}
