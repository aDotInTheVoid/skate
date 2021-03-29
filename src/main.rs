#![allow(clippy::all)]
mod grammar; // synthesized by LALRPOP

mod ast;
mod exec;

use fs_err as fs;

// TODO: Make this good.
#[cfg(test)]
mod ast_tests;

fn main() -> eyre::Result<()> {
    let prog = std::env::args()
        .nth(1)
        .ok_or_else(|| eyre::eyre!("Useage: skate <program>"))?;
    let prog = fs::read_to_string(&prog)?;

    // Due to lifetime reasons, we cant convert a parse err to an eyre err
    let prog = grammar::ProgramParser::new().parse(&prog);

    let prog = match prog {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e);
            eyre::bail!("Parse error");
        }
    };

    exec::run(prog)?;

    Ok(())
}
