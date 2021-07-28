use std::io;

use compiler::bytecode;
use rt_common::RT;
use value::{BigValue, Value, ValueDbg};

use crate::VM;

impl<'w> VM<'w> {
    pub fn new(output: &'w mut dyn io::Write) -> Self {
        Self {
            output,
            stack: Default::default(),
            heap: Default::default(),
            frames: Default::default(),
        }
    }

    pub(crate) fn stack_len(&self) -> usize {
        self.stack.len()
    }

    pub(crate) fn ip(&self) -> usize {
        self.frames.last().unwrap().ip
    }

    pub(crate) fn ip_mut(&mut self) -> &mut usize {
        &mut self.frames.last_mut().unwrap().ip
    }

    pub(crate) fn stack_offset(&self) -> usize {
        self.frames.last().unwrap().stack_offset
    }

    pub(crate) fn get_func<'b, 'a, 's>(
        &self,
        code: &'b bytecode::Code<'a, 's>,
    ) -> &'b bytecode::Func<'a, 's> {
        let key = self.frames.last().unwrap().func;
        &code.fns[key]
    }

    pub(crate) fn push(&mut self, val: Value) {
        self.stack.push(val);
    }

    pub(crate) fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }

    pub(crate) fn peak(&mut self) -> Value {
        *self.stack.last().unwrap()
    }

    pub fn add_to_heap(&mut self, v: BigValue) -> Value {
        Value::Complex(self.heap.insert(v))
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
