use std::io::{self};

use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term::emit;
use diagnostics::{CompError, RtError};
use wasm_bindgen::prelude::*;

struct ColorWriter(Vec<u8>);

impl io::Write for ColorWriter {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    fn write_vectored(&mut self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize> {
        self.0.write_vectored(bufs)
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.0.write_all(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}

// TODO: Implement this, and then spin off to a crate
impl termcolor::WriteColor for ColorWriter {
    fn supports_color(&self) -> bool {
        false
    }

    fn set_color(&mut self, _spec: &termcolor::ColorSpec) -> io::Result<()> {
        Ok(())
    }

    fn reset(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[wasm_bindgen(catch)]
pub fn run_code(code: &str) -> Result<String, JsValue> {
    let mut err_files = SimpleFiles::new();
    let id = err_files.add("source.sk", code);
    let err_config = codespan_reporting::term::Config::default();

    let mut output = Vec::new();

    // TODO: Use bytecode, and also suport dumping bytecode
    let result = skate_treewalk::run(code, id, &mut output);

    match match result {
        Ok(v) => Ok(v),
        Err(e) => match e.downcast::<RtError>() {
            Ok(v) => Err(v.0),
            Err(e) => match e.downcast::<CompError>() {
                Ok(e) => Err(e.0),
                Err(e) => Err(e).unwrap(),
            },
        },
    } {
        Ok(_) => Ok(String::from_utf8(output).unwrap()),
        Err(diag) => {
            let mut writer = ColorWriter(Vec::new());
            emit(&mut writer, &err_config, &err_files, &diag).unwrap();
            Err(String::from_utf8(writer.0).unwrap().into())
        }
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}
