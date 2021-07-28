use std::fmt::{Debug, Formatter, Result};

use super::{Code, Func, Instr};

// Keep in sync with debug2_impl.rs

impl Debug for Code<'_, '_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut s = f.debug_struct("Code");
        for (name, f) in self.fns.iter().map(|(_, f)| (f.name, FnDbg(f))) {
            s.field(name, &f);
        }
        s.finish()
    }
}

struct FnDbg<'a>(&'a Func<'a, 'a>);

impl Debug for FnDbg<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // TODO: Print FuncKey's as their name
        f.debug_list().entries(self.0.code.iter()).finish()
    }
}

struct InstrDebug<'a>(&'a Instr<'a>, &'a Code<'a, 'a>);

impl Debug for InstrDebug<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if let Instr::Call(id) = self.0 {
            f.debug_tuple("Call").field(&self.1.fns[*id].name).finish()
        } else {
            self.0.fmt(f)
        }
    }
}
