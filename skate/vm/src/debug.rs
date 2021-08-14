use std::fmt::Display;

use diagnostics::span::Span;
use eyre::Result;

use compiler::bytecode::{self, Instr};

pub struct Stepper<'a> {
    vm: super::VM<'a>,
    code: bytecode::Code<'a, 'a>,
}

impl<'a> Stepper<'a> {
    pub fn new(
        code: bytecode::Code<'a, 'a>,
        main_key: bytecode::FuncKey,
        output: &'a mut dyn std::io::Write,
    ) -> Self {
        let mut vm = crate::VM::new(output);
        vm.seed(main_key);
        Stepper { vm, code }
    }

    pub fn step(&mut self) -> Result<Option<i64>> {
        self.vm.step(&self.code)
    }

    pub fn run_to_end(&mut self) -> Result<i64> {
        self.vm.run_to_end(&self.code)
    }

    pub fn flush(&mut self) -> std::io::Result<()> {
        self.vm.output.flush()
    }

    pub fn next_instr(&self) -> &Instr {
        &self.vm.get_func(&self.code).code[self.vm.ip()]
    }

    pub fn next_loc(&self) -> Option<Span> {
        self.vm.get_func(&self.code).spans[self.vm.ip()].span()
    }

    pub fn print_stack(&self, name: impl Display) {
        eprintln!(
            "{} {:?}",
            name,
            value::StackDbg {
                stack: &self.vm.stack,
                heap: &self.vm.heap
            }
        );
    }
}
