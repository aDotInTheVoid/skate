use std::io;

use value::{Value, ValueDbg};

use crate::VM;

impl<'a, 'b> VM<'a, 'b> {
    pub(crate) fn print_value(&mut self, v: &Value) -> io::Result<()> {
        let dbg = ValueDbg {
            v,
            heap: &self.heap,
        };
        writeln!(self.output, "{}", dbg)?;
        Ok(())
    }
}
