#[allow(clippy::all)]
mod grammar; // synthesized by LALRPOP

mod ast;
pub mod diagnostics;
mod env;
mod exec;
mod value;

use codespan_reporting::diagnostic::{Diagnostic, Label};
use lalrpop_util::ParseError;

use crate::diagnostics::CompError;

// TODO: Make this good.
#[cfg(test)]
mod ast_tests;

pub enum ExitCode {
    Ok,
    CompErr,
    RtErr,
    ProgErr,
}

pub fn run(prog: &str, main_file_id: usize) -> eyre::Result<ExitCode> {
    let prog = grammar::ProgramParser::new().parse(diagnostics::FileId(main_file_id), &prog);

    let prog = match prog {
        Ok(p) => p,
        Err(e) => {
            let err = match e {
                ParseError::UnrecognizedToken { token, expected } => Diagnostic::error()
                    .with_message("Unexpected token")
                    .with_labels(vec![Label::primary(main_file_id, token.0..token.2)])
                    .with_notes(
                        expected
                            .iter()
                            .map(|e| format!("Expected: {}", e))
                            .collect(),
                    ),

                ParseError::InvalidToken { location } => Diagnostic::error()
                    .with_message("Invalid token")
                    .with_labels(vec![Label::primary(main_file_id, location..location + 1)]),

                ParseError::UnrecognizedEOF { location, expected } => Diagnostic::error()
                    .with_message("Unexpected EOF")
                    .with_labels(vec![Label::primary(main_file_id, location..location + 1)])
                    .with_notes(
                        expected
                            .iter()
                            .map(|e| format!("Expected: {}", e))
                            .collect(),
                    ),
                // TODO: Use nice reporting for the rest, if it exists
                _ => todo!(),
            };

            return Err(CompError(err).into());
        }
    };

    match exec::run(prog) {
        // TODO: Ok case has an exit status from skate code, handle that
        Ok(is_fail) => {
            if !is_fail {
                Ok(ExitCode::Ok)
            } else {
                Ok(ExitCode::ProgErr)
            }
        }
        Err(e) => Err(e)

        // match e.downcast::<diagnostics::RtError>() {
        //     Ok(rterrot) => {
        //         emit_err(&rterrot.0)?;
        //         Ok(ExitCode::RtErr)
        //     }
        //     Err(e) => Err(e),
        // },
    }
}
