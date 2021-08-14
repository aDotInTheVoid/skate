use std::io::{self, BufWriter};

use codespan_reporting::diagnostic::Diagnostic;
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term::emit;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use crossterm::style::Stylize;
use eyre::Result;
use fs_err as fs;
use structopt::StructOpt;

use crate::command::Command;

mod command;
mod prefix_writer;

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

    let emit_err = |d| emit(&mut err_writer.lock(), &err_config, &err_files, &d);

    let prog = match parser::ProgramParser::new().parse(diagnostics::FileId(main_file_id), &prog) {
        Ok(x) => x,
        Err(x) => {
            let err = rt_common::parse_error_labeled(x, main_file_id);
            emit_err(err)?;
            // TODO: Drop things
            std::process::exit(1);
        }
    };

    // TODO: Unify compilation boilerplate between skatrace and skate
    let (code, main_id) = match compiler::compile(&prog) {
        Ok(x) => x,
        Err(comperr) => {
            emit_err(comperr.0)?;
            std::process::exit(1);
        }
    };

    if debug {
        eprintln!("{}", debug2::pprint(&code));
    }

    let write_out = "Output: ".yellow().to_string();

    let mut stdout_writer =
        prefix_writer::PrefixWriter::new(write_out.as_bytes(), BufWriter::new(io::stderr()));

    let mut vm = vm::debug::Stepper::new(code, main_id, &mut stdout_writer);

    // let res = vm.run(&code, main_id);

    let mut prev = None;

    let mut rl = rustyline::Editor::<()>::new();
    while let Ok(line) = rl.readline(&">> ".red().to_string()) {
        vm.flush()?;
        rl.add_history_entry(&line);

        let mut command = command::parse(&line).unwrap();
        if command == Command::Prev {
            if let Some(prev) = prev.clone() {
                command = prev;
            } else {
                continue;
            }
        }

        match command {
            Command::Instr => {
                // TODO: Decode things like what a load is of, or what a function key means
                // TODO: Skip things like `Pop` Instr
                eprintln!("{} {:?}", "Instr".yellow(), vm.next_instr());
                if let Some(span) = vm.next_loc() {
                    // TODO: Better output here
                    // - Syntax highlight code
                    // - Underline to be more consice
                    // - Give lines up and down
                    // - Dont print "Note"
                    // lldb is great at this
                    let d = Diagnostic::note().with_labels(vec![span.primary_label()]);
                    emit_err(d)?;
                };

                match vm.step() {
                    Ok(None) => {}
                    Ok(Some(_n)) => break,
                    Err(e) => match e.downcast() {
                        Ok(diagnostics::RtError(e)) => {
                            vm.flush()?;
                            emit_err(e)?;
                            std::process::exit(1);
                        }
                        Err(e) => return Err(e),
                    },
                }
                vm.print_stack("Stack".yellow());
            }
            Command::Quit => break,
            Command::Prev => unreachable!(),
        }
        prev = Some(command);
        vm.flush()?;
    }

    vm.flush()?;

    Ok(())
}
