use std::fmt::{Debug, Formatter, Result};

use super::{Code, Func};

impl Debug for Code<'_, '_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_map()
            .entries(self.fns.iter().map(|(_, f)| (f.name, FnDbg(f))))
            .finish()
    }
}

struct FnDbg<'a>(&'a Func<'a, 'a>);

impl Debug for FnDbg<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // TODO: Print FuncKey's as their name
        f.debug_list().entries(self.0.code.iter()).finish()
    }
}
