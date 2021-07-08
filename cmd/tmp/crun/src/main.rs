use std::io::{self, BufWriter, Write};

use eyre::{Context, Result};

use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term::emit;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use fs_err as fs;

fn main() -> Result<()> {
    let progname = std::env::args()
        .nth(1)
        .ok_or_else(|| eyre::eyre!("Useage: crun <program>"))?;

    let mut err_files = SimpleFiles::new();
    let err_writer = StandardStream::stderr(ColorChoice::Auto);
    let err_config = codespan_reporting::term::Config::default();

    let prog = fs::read_to_string(&progname)?;

    let main_file_id = err_files.add(&progname, &prog);

    let emit_err = |d| emit(&mut err_writer.lock(), &err_config, &err_files, d);

    let prog = match parser::ProgramParser::new().parse(diagnostics::FileId(main_file_id), &prog) {
        Ok(x) => x,
        Err(x) => {
            let err = rt_common::parse_error_labeled(x, main_file_id);
            emit_err(&err)?;

            std::process::exit(1);
        }
    };

    let (code, main_id) = compiler::compile(&prog);

    dbg!(&code);

    let mut stdout_writer = BufWriter::new(io::stdout());

    let mut vm = vm::VM::new(&mut stdout_writer);

    let res = vm.run(&code, main_id);
    if let Err(e) = res {
        match e.downcast() {
            Ok(diagnostics::RtError(e)) => {
                emit_err(&e)?;
                std::process::exit(1);
            }
            Err(e) => return Err(e),
        }
    }

    stdout_writer.flush()?;

    Ok(())
}
