use std::io;

use codespan_reporting::diagnostic::Diagnostic;
use eyre::Result;

use compiler::bytecode::{self, FuncKey, Instr};
use diagnostics::RtError;
use parser::{Literal, RawStmt};
use rt_common::RT;
use value::{BigValue, Heap, Map, Value};

mod utils;

// TODO: Make this a flag
const DEBUG: bool = false;

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
    pub fn run(&mut self, code: &bytecode::Code, main_key: bytecode::FuncKey) -> Result<i64> {
        // All of this assumes a fresh VM, which is probably bad design

        self.frames.push(Frame {
            func: main_key,
            ip: 0,
            stack_offset: 0,
        });

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
                /*
                 * Basic
                 */
                Instr::LoadLit(l) => {
                    let val = match *l {
                        Literal::String(s) => (self.add_to_heap(BigValue::String(s.to_owned()))),
                        Literal::Float(f) => Value::Float(f),
                        Literal::Integer(i) => Value::Int(i),
                        Literal::Bool(b) => Value::Bool(b),
                        Literal::Null => Value::Null,
                    };
                    self.push(val);
                }
                Instr::Print => {
                    let val = self.pop();
                    self.print_value(&val)?;
                }
                Instr::Pop => {
                    self.pop();
                }
                /*
                 * Expression Ops
                 */
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
                Instr::UnOp(op) => {
                    let val = self.pop();
                    let (uop, expr) = self.get_func(code).spans[ip]
                        .as_expr()
                        .unwrap()
                        .as_unary_op()
                        .unwrap();

                    // Because currently we get the op from the ast, lmao
                    debug_assert_eq!(*op, uop.node);

                    // Ditto wrt lazy ast load
                    let nv = self.unary_op(*uop, val, expr.span)?;
                    self.push(nv);
                }

                /*
                 * Local Access
                 */
                Instr::GetLocal(id) => {
                    self.push(self.stack[self.stack_offset()..][*id]);
                }
                Instr::SetLocal(id) => self.stack[*id] = self.peak(),

                /*
                 * Jumping
                 */
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

                    let span = match &self.get_func(code).spans[ip].as_stmt().unwrap().node {
                        RawStmt::If(cond, _, _) => cond.span,
                        RawStmt::While(cond, _) => cond.span,
                        other => panic!("Unexpected {:?}", other),
                    };

                    if !self.as_bool(val, span)? {
                        *self.ip_mut() += by;
                    }
                }

                /*
                 * Compex Creation
                 */
                Instr::MakeArray(num) => {
                    let ar = self.stack.split_off(self.stack.len() - num);
                    let hid = self.add_to_heap(BigValue::Array(ar));
                    self.push(hid);
                }
                Instr::MakeMap(len) => {
                    let mut map = Map::new();
                    for _ in 0..*len {
                        let val = self.pop();
                        let key = self.pop();
                        let str = self.heap[*key.as_complex().unwrap()]
                            .as_string()
                            .unwrap()
                            .clone();
                        assert!(map.insert(str, val).is_none());
                    }
                    let val = self.add_to_heap(BigValue::Map(map));
                    self.push(val);
                }

                /*
                 * Complex access and setters
                 */
                Instr::ArrayAccess => {
                    let index_val = self.pop();
                    let array_val = self.pop();

                    // TODO: Lazy load ast
                    let (array_span, idx_span) = self.get_func(code).spans[ip]
                        .as_expr()
                        .unwrap()
                        .as_array_access()
                        .unwrap();

                    let idx = self.as_uint(index_val, idx_span.span);
                    let array = self.as_array_mut(array_val, array_span.span)?;
                    let idx = idx?;

                    if let Some(res) = array.get(idx) {
                        let res = *res;
                        self.push(res);
                    } else {
                        // TODO: Extract
                        return Err(RtError(
                            Diagnostic::error()
                                .with_message(format!(
                                    "Array index out of bound, len is `{}`, but got `{}`",
                                    array.len(),
                                    idx
                                ))
                                .with_labels(vec![
                                    self.evaled_to(index_val, idx_span.span),
                                    self.evaled_to(array_val, array_span.span),
                                ]),
                        )
                        .into());
                    }
                }
                Instr::ArraySet => {
                    let expr = self.pop();
                    let index_val = self.pop();
                    let array_val = self.pop();

                    // TODO: Lazy load ast
                    let (array_span, idx_span) = self.get_func(code).spans[ip]
                        .as_stmt()
                        .unwrap()
                        .as_assign()
                        .unwrap()
                        .0
                        .as_array_access()
                        .unwrap();

                    let index = self.as_uint(index_val, idx_span.span);
                    let array = self.as_array_mut(array_val, array_span.span)?;
                    let index = index?;

                    if let Some(dest) = array.get_mut(index) {
                        *dest = expr;
                    } else {
                        return Err(RtError(
                            Diagnostic::error()
                                .with_message(format!(
                                    "Array index out of bound, len is `{}`, but got `{}`",
                                    array.len(),
                                    index
                                ))
                                .with_labels(vec![
                                    self.evaled_to(index_val, idx_span.span),
                                    self.evaled_to(array_val, array_span.span),
                                ]),
                        )
                        .into());
                    }
                }
                Instr::FieldAccess(name) => {
                    let map_value = self.pop();
                    // TODO: Lazy load location

                    let (map_span, key_span) = self.get_func(code).spans[ip]
                        .as_expr()
                        .unwrap()
                        .as_field_access()
                        .unwrap();

                    let map = self.as_map(map_value, map_span.span)?;

                    if let Some(val) = map.get(name.node) {
                        let val = *val;
                        self.push(val);
                    } else {
                        return Err(RtError(
                            Diagnostic::error()
                                .with_message(format!("Map doesnt have key `{}`", name.node))
                                .with_labels(vec![
                                    self.evaled_to(map_value, map_span.span),
                                    key_span.span.primary_label().with_message("This key"),
                                ]),
                        )
                        .into());
                    }
                }
                Instr::FieldSet(name) => {
                    let expr = self.pop();
                    let map = self.pop();

                    let span = self.get_func(code).spans[ip]
                        .as_stmt()
                        .unwrap()
                        .as_assign()
                        .unwrap()
                        .0
                        .as_field_access()
                        .unwrap()
                        .0
                        .span;

                    let map = self.as_map_mut(map, span)?;
                    map.insert(name.node.to_owned(), expr);
                }

                /*
                 * Functions
                 */
                Instr::Return => {
                    let result = self.pop();

                    let frame = self.frames.pop().unwrap();
                    self.stack.truncate(frame.stack_offset);
                    self.push(result);

                    if self.frames.is_empty() {
                        return Ok(match result {
                            Value::Int(n) => n,

                            Value::Null => 0,
                            _ => {
                                return Err(RtError(Diagnostic::error().with_message(format!(
                                "`main` returned `{:?}` with type `{}`, expected `null` or `int`",
                                self.dbg_val(&result),
                                self.type_name(&result),)))
                                .into())
                            }
                        });
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
        }
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

    let (code, main_fn) = compiler::compile(&prog).unwrap();

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

    let (code, main_fn) = compiler::compile(&prog).unwrap();

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

    let (code, main_fn) = compiler::compile(&prog).unwrap();

    vm.run(&code, main_fn).unwrap();
    assert_eq!(vm.stack.len(), 1);
    assert_eq!(stdout.len(), 0);
}
