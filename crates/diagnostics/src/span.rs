use std::ops::Range;

use codespan_reporting::diagnostic::Label;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Copy, Default)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub file_id: crate::FileId,
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

impl Span {
    pub fn range(&self) -> Range<usize> {
        self.start..self.end
    }

    pub fn primary_label(&self) -> Label<usize> {
        Label::primary(self.file_id.0, self.range())
    }

    // pub(crate) fn evaled_to_primary(&self, v: value::Value, e: &env::VM) -> Label<usize> {
    //     self.primary_label()
    //         .with_message(format!("Evaluated to `{:?}`", e.dbg_val(&v)))
    // }

    // pub(crate) fn evaled_to(&self, v: value::Value, e: &env::VM) -> Label<usize> {
    //     self.secondary_label()
    //         .with_message(format!("Evaluated to `{:?}`", e.dbg_val(&v)))
    // }

    pub fn secondary_label(&self) -> Label<usize> {
        Label::secondary(self.file_id.0, self.range())
    }
}

impl<T> Spanned<T> {
    pub fn primary_label(&self) -> Label<usize> {
        self.span.primary_label()
    }

    pub fn secondary_label(&self) -> Label<usize> {
        self.span.secondary_label()
    }
}

// TODO: Decide if this is a good idea
impl<T> std::ops::Deref for Spanned<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}
impl<T> std::ops::DerefMut for Spanned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.node
    }
}
