use std::io::{self, BufWriter, Write};

use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term::emit;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use eyre::Result;
use fs_err as fs;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    #[structopt(short, long)]
    debug: bool,
    progname: String,
}

fn main() -> Result<()> {
    let Args { debug, progname } = Args::from_args();

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
            // TODO: Drop things
            std::process::exit(1);
        }
    };

    let (code, main_id) = match compiler::compile(&prog) {
        Ok(x) => x,
        Err(comperr) => {
            emit_err(&comperr.0)?;
            std::process::exit(1);
        }
    };

    if debug {
        eprintln!("{}", debug2::pprint(&code));
    }

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
