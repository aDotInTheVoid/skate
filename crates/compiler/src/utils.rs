use crate::bytecode::Instr;

use super::FnComping;

#[must_use]
#[derive(Debug)]
pub(crate) struct JumpHelper(usize);

#[must_use]
#[derive(Debug)]
pub(crate) struct BackJumpHelper(usize);

impl<'a, 's> FnComping<'a, 's, '_> {
    fn add_loop(
        &mut self,
        ty: impl Fn(usize) -> Instr<'static>,
        stmt: &'a parser::Stmt<'s>,
    ) -> JumpHelper {
        self.add_instr_sloc(ty(0), stmt);
        JumpHelper(self.n_instrs())
    }

    pub(crate) fn patch_fjump(&mut self, h: JumpHelper) {
        let current_ip = self.n_instrs();
        let diff = current_ip - h.0;

        if let Instr::JumpForward(i) | Instr::JumpForwardIfFalse(i) = &mut self.output.code[h.0 - 1]
        {
            *i = diff;
        } else {
            unreachable!();
        }
    }

    /// Add [`Instr::JumpForwardIfFalse`]
    pub(crate) fn add_jfif(&mut self, stmt: &'a parser::Stmt<'s>) -> JumpHelper {
        self.add_loop(Instr::JumpForwardIfFalse, stmt)
    }

    /// Add [`Instr::JumpForward`]
    pub(crate) fn add_jf(&mut self, stmt: &'a parser::Stmt<'s>) -> JumpHelper {
        self.add_loop(Instr::JumpForward, stmt)
    }

    pub(crate) fn add_jump_back_to_point(&self) -> BackJumpHelper {
        BackJumpHelper(self.n_instrs())
    }

    pub(crate) fn add_jb(&mut self, pos: BackJumpHelper, stmt: &'a parser::Stmt<'s>) {
        let ip = self.output.code.len();
        // TODO: Why this + 1
        let diff = ip - pos.0 + 1;
        self.add_instr_sloc(Instr::JumpBackward(diff), stmt)
    }
}
