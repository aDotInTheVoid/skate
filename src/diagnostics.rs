use std::fmt::Display;
use std::usize;

use codespan_reporting::diagnostic::Diagnostic;

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, Hash)]
pub struct FileId(pub usize);

#[derive(Debug)]
pub struct RTError(pub Diagnostic<usize>);

impl Display for RTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // This error should be reported via codespan, not Display
        f.write_str("THIS SHOULD NEVER COME UP. PLEASE FILE A BUG")
    }
}
