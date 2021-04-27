#[allow(clippy::all)]
mod grammar; // synthesized by LALRPOP

mod ast;
mod diagnostics;
mod exec;
mod value;

use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term::emit;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use fs_err as fs;
use lalrpop_util::ParseError;

// TODO: Make this good.
#[cfg(test)]
mod ast_tests;

fn main() -> eyre::Result<()> {
    let has_error = realmain()?;
    // Error codes
    // 101 -> Rust panic
    // 1 -> Rust main returned Err(_)
    // 66 -> Skate Compiller error
    // TODO: Custom exit for Rt error (currently 1)
    if has_error {
        std::process::exit(66)
    }
    Ok(())
}

// Err(_) -> Fail with interpriter code error. This should be less common, as we dont realy want a
//           backtrace through user code
// Ok(false) -> Sucess, exit 0
// Ok(true) -> Error reported, main should exit with error code
// This is because for an error in user code, we dont want print the eyre backtrace through
// interpriter code, as this is a hot path
fn realmain() -> eyre::Result<bool> {
    let progname = std::env::args()
        .nth(1)
        .ok_or_else(|| eyre::eyre!("Useage: skate <program>"))?;

    let mut err_files = SimpleFiles::new();
    let err_writer = StandardStream::stderr(ColorChoice::Auto);
    let err_config = codespan_reporting::term::Config::default();

    let prog = fs::read_to_string(&progname)?;

    let main_file_id = err_files.add(&progname, &prog);

    // Due to lifetime reasons, we cant convert a parse err to an eyre err
    let prog = grammar::ProgramParser::new().parse(diagnostics::FileId(main_file_id), &prog);

    let prog = match prog {
        Ok(p) => p,
        Err(e) => {
            match e {
                ParseError::UnrecognizedToken { token, expected } => {
                    let parse_error = Diagnostic::error()
                        .with_message("Unexpected token")
                        .with_labels(vec![Label::primary(main_file_id, token.0..token.2)])
                        .with_notes(
                            expected
                                .iter()
                                .map(|e| format!("Expected: {}", e))
                                .collect(),
                        );

                    emit(
                        &mut err_writer.lock(),
                        &err_config,
                        &err_files,
                        &parse_error,
                    )?;
                }

                ParseError::InvalidToken { location } => {
                    let parse_error = Diagnostic::error()
                        .with_message("Invalid token")
                        .with_labels(vec![Label::primary(main_file_id, location..location + 1)]);

                    emit(
                        &mut err_writer.lock(),
                        &err_config,
                        &err_files,
                        &parse_error,
                    )?;
                }
                // TODO: Use nice reporting for the rest
                _ => eprintln!("{}", e),
            }

            return Ok(true);
        }
    };

    exec::run(prog)?;

    Ok(false)
}
