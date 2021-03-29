#![allow(clippy::all)]
mod grammar; // synthesized by LALRPOP

mod ast;
mod exec;

use fs_err as fs;

// TODO: Make this good.
#[cfg(test)]
mod ast_tests;

fn main() -> eyre::Result<()> {
    let has_error = realmain()?;
    if has_error {
        std::process::exit(1)
    }
    Ok(())
}

// Err(_) -> Fail with interpriter code error
// Ok(false) -> Sucess
// Ok(true) -> Error reported, exit(1) but dont fail
// This is because for an error in user code, we dont want print the eyre backtrace through
// interpriter code, as this is a hot path
fn realmain() -> eyre::Result<bool> {
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
            return Ok(true);
        }
    };

    exec::run(prog)?;

    Ok(false)
}
