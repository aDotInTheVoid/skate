use std::io::{self, BufWriter, Write};

// use compiler::compile;
use eyre::Result;

fn main() -> Result<()> {
    let prog = std::fs::read_to_string(
        std::env::args()
            .nth(1)
            .ok_or_else(|| eyre::eyre!("Pass an arg"))?,
    )?;

    let prog = parser::ProgramParser::new()
        .parse(diagnostics::FileId(0), &prog)
        .unwrap();

    let (code, main_id) = compiler::compile(&prog);

    let mut stdout_writer = BufWriter::new(io::stdout());

    let mut vm = vm::VM::new(&mut stdout_writer);

    vm.run(code, main_id)?;

    stdout_writer.flush()?;

    //let bytecode = compile(prog);
    // dbg!(bytecode);

    Ok(())
}
