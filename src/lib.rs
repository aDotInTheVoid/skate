// mod env;
// mod exec;

use std::io;

use diagnostics::CompError;

// Keep this without a drop impl (even implicit), as we may exit while
// holding one
#[wasm_bindgen::prelude::wasm_bindgen]
pub enum ExitCode {
    Ok,
    CompErr,
    RtErr,
    ProgErr,
}

pub fn run(prog: &str, main_file_id: usize, output: &mut dyn io::Write) -> eyre::Result<ExitCode> {
    let prog = parser::ProgramParser::new().parse(diagnostics::FileId(main_file_id), prog);

    let prog = match prog {
        Ok(p) => p,
        Err(e) => return Err(CompError(rt_common::parse_error_labeled(e, main_file_id)).into()),
    };

    match treewalk::run(prog, output) {
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
