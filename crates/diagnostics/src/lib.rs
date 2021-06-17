use std::fmt::Display;

use codespan_reporting::diagnostic::Diagnostic;

pub mod span;

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, Hash, Default)]
pub struct FileId(pub usize);

#[derive(Debug)]
pub struct RtError(pub Diagnostic<usize>);

impl Display for RtError {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // This error should be reported via codespan, not Display
        unreachable!("THIS SHOULD NEVER COME UP. PLEASE FILE A BUG")
    }
}

impl std::error::Error for RtError {}

#[derive(Debug)]
pub struct CompError(pub Diagnostic<usize>);

impl Display for CompError {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // This error should be reported via codespan, not Display
        unreachable!("THIS SHOULD NEVER COME UP. PLEASE FILE A BUG")
    }
}

impl std::error::Error for CompError {}
