use std::collections::HashMap;
use std::io;

use codespan_reporting::diagnostic::Diagnostic;
use diagnostics::span::Spanned;
use diagnostics::RtError;
use eyre::Result;
use parser::{Function, Item, Program};
use rt_common::RT;
use value::{BigValue, Heap, HeapKey, Value};

mod exec;
mod lvalue;
mod scope;
mod utils;

// #[derive(Debug, Default)]
pub(crate) struct VM<'a, 'b> {
    pub(crate) functions: HashMap<&'a str, &'a Spanned<Function<'a>>>,
    pub(crate) heap: Heap,
    pub(crate) output: &'b mut dyn io::Write,
}

impl<'a, 'b> VM<'a, 'b> {
    pub fn new(p: &'a [Item<'a>], output: &'b mut dyn io::Write) -> eyre::Result<Self> {
        let mut functions: HashMap<&str, &Spanned<Function>> = HashMap::new();
        for i in p {
            match i {
                Item::Function(_, f) => {
                    if let Some(old_fn) = functions.insert(&f.name, f) {
                        // TODO: Should these be the function span of the name span
                        let err = Diagnostic::error()
                            .with_message(format!("Function `{}` defined twice", &f.name.node))
                            .with_labels(vec![
                                old_fn.secondary_label().with_message("Defined once here"),
                                f.secondary_label().with_message("Defined again here"),
                            ]);
                        return Err(RtError(err).into());
                    }
                }
                Item::Import(_) => {
                    // TODO
                }
                Item::Const(..) => {
                    // TODO
                }
            };
        }

        Ok(Self {
            functions,
            heap: Default::default(),
            output,
        })
    }

    pub fn add_to_heap(&mut self, v: BigValue) -> HeapKey {
        self.heap.insert(v)
    }
}

// Err -> Exit err due to type error/rt error
// Ok(false) -> Exit sucess
// Ok(true) -> Exit err due to user code request
pub fn run(p: Program, output: &mut dyn io::Write) -> Result<bool> {
    // If nothing failed, we suceed
    let mut vm = VM::new(&p, output)?;

    let result = vm.call(
        Spanned {
            node: "main",
            // TODO: give a real span to ensure the no main fn error is good
            span: Default::default(),
        },
        &[],
        // ditto
        Default::default(),
    )?;

    // TODO: Figure out exit codes 0/1/101
    // One for script exited with error, one for IIE
    let is_fail = match result {
        Value::Null => false,
        Value::Int(x) => x != 0,
        x => {
            // TODO: Give a span
            return Err(RtError(Diagnostic::error().with_message(format!(
                "`main` returned `{:?}` with type `{}`, expected `null` or `int`",
                vm.dbg_val(&x),
                vm.type_name(&x),
            )))
            .into());
        }
    };

    Ok(is_fail)
}

impl rt_common::RT for VM<'_, '_> {
    fn heap(&self) -> &value::Heap {
        &self.heap
    }

    fn heap_mut(&mut self) -> &mut value::Heap {
        &mut self.heap
    }
}
