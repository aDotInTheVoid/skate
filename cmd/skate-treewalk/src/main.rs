use std::io::{self, BufWriter, Write};

use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term::emit;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use fs_err as fs;

use diagnostics::{CompError, RtError};
use skate_treewalk::ExitCode;

// Err(_) -> Fail with interpriter code error. This should be less common, as we dont realy want a
//           backtrace through user code
// Ok(false) -> Sucess, exit 0
// Ok(true) -> Error reported, main should exit with error code
// This is because for an error in user code, we dont want print the eyre backtrace through
// interpriter code, as this is a hot path
fn realmain() -> eyre::Result<ExitCode> {
    let progname = std::env::args()
        .nth(1)
        .ok_or_else(|| eyre::eyre!("Useage: skate <program>"))?;

    let mut err_files = SimpleFiles::new();
    let err_writer = StandardStream::stderr(ColorChoice::Auto);
    let err_config = codespan_reporting::term::Config::default();

    let prog = fs::read_to_string(&progname)?;

    let main_file_id = err_files.add(&progname, &prog);

    let emit_err = |d| emit(&mut err_writer.lock(), &err_config, &err_files, d);

    let mut stdout_writer = BufWriter::new(io::stdout());

    let main_res = skate_treewalk::run(&prog, main_file_id, &mut stdout_writer);
    stdout_writer.flush()?;

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

fn main() -> eyre::Result<()> {
    // Ensure no complex values exist in this function, as exit doesnt run them.
    let error = realmain()?;

    // Error codes
    // 101 -> Rust panic
    // 1 -> Rust main returned Err(_)
    // 66 -> Skate Compiller error
    // TODO: Custom exit for Rt error (currently 1)
    let code = match error {
        ExitCode::Ok => 0,
        ExitCode::CompErr => 66,
        // TODO: chaing me, when this catches all errs
        ExitCode::RtErr => 1,
        ExitCode::ProgErr => 22,
    };
    std::process::exit(code);
}
