#[allow(clippy::all)]
mod grammar; // synthesized by LALRPOP

mod ast;
mod diagnostics;
mod env;
mod exec;
mod value;

use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term::emit;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use fs_err as fs;
use lalrpop_util::ParseError;

use crate::diagnostics::{CompError, RtError};

// TODO: Make this good.
#[cfg(test)]
mod ast_tests;

pub enum ExitCode {
    Ok,
    CompErr,
    RtErr,
    ProgErr,
}

fn run(prog: &str, main_file_id: usize) -> eyre::Result<ExitCode> {
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
                // TODO: Use nice reporting for the rest
                e => todo!(),
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

// Err(_) -> Fail with interpriter code error. This should be less common, as we dont realy want a
//           backtrace through user code
// Ok(false) -> Sucess, exit 0
// Ok(true) -> Error reported, main should exit with error code
// This is because for an error in user code, we dont want print the eyre backtrace through
// interpriter code, as this is a hot path
pub fn realmain() -> eyre::Result<ExitCode> {
    let progname = std::env::args()
        .nth(1)
        .ok_or_else(|| eyre::eyre!("Useage: skate <program>"))?;

    let mut err_files = SimpleFiles::new();
    let err_writer = StandardStream::stderr(ColorChoice::Auto);
    let err_config = codespan_reporting::term::Config::default();

    let prog = fs::read_to_string(&progname)?;

    let main_file_id = err_files.add(&progname, &prog);

    let emit_err = |d| emit(&mut err_writer.lock(), &err_config, &err_files, d);

    let main_res = run(&prog, main_file_id);

    match main_res {
        ok @ Ok(_) => ok,
        Err(e) => {
            // if let Ok(comperr) = e.downcast::<CompError>() {
            //     emit_err(&comperr.0)?;
            //     Ok(ExitCode::CompErr)
            // } else if let Ok(rterr) = e.downcast::<RtError>() {
            //     emit_err(&rterr.0)?;
            //     Ok(ExitCode::CompErr)
            // } else {
            //     Err(e)
            // }
            match e.downcast::<CompError>() {
                Ok(comperr) => {
                    emit_err(&comperr.0)?;
                    Ok(ExitCode::CompErr)
                }
                Err(e) => match e.downcast::<RtError>() {
                    Ok(rterr) => {
                        emit_err(&rterr.0)?;
                        Ok(ExitCode::RtErr)
                    }
                    Err(e) => Err(e),
                },
            }
        }
    }
}
