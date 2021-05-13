use skate::{realmain, ExitCode};

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
